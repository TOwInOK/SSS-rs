use sss_std::{prelude::Layouts, themes::Themes};
use tokio::signal;
#[cfg(not(windows))]
use tokio::signal::unix::SignalKind;
use tracing::{error, info, instrument, warn};

use crate::{
    tools::refresh,
    web::{file_watcher::check_file_loop, serve::serve},
};

#[instrument(skip(is_web, is_watch, path, layouts, address, themes))]
pub async fn command_run(
    is_watch: bool,
    is_web: bool,
    path: &str,
    layouts: &Option<Layouts>,
    address: &str,
    themes: &Option<Themes>,
) -> anyhow::Result<()> {
    info!("Start run command");
    if !is_web && !is_watch {
        warn!("Nothing to run!\nTry to use sss_cli run -h to show avaiable options");
        return Ok(());
    }

    refresh(path, themes.as_ref(), layouts.as_ref())
        .await
        .map_err(|e| anyhow::anyhow!("Load failed: {}", e))?;

    let (shutdown_tx, _) = tokio::sync::broadcast::channel(1);
    if is_watch {
        let path_clone = path.to_string();
        let themes = themes.clone();
        let layouts = layouts.clone();
        let shutdown_rx = shutdown_tx.subscribe();

        tokio::spawn(async move {
            if let Err(e) =
                check_file_loop(path_clone, themes.as_ref(), layouts.as_ref(), shutdown_rx).await
            {
                error!("File watching error: {}", e);
            }
        });
    }

    if is_web {
        let shutdown_rx = shutdown_tx.subscribe();
        let address = address.to_string();

        tokio::spawn(async move {
            if let Err(e) = serve(address, shutdown_rx).await {
                error!("Web server error: {}", e);
            }
        });
    }

    #[cfg(not(windows))]
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Received Ctrl+C, initiating shutdown");
        }
        _ = async {
            let mut sigterm = signal::unix::signal(SignalKind::terminate()).unwrap();
            sigterm.recv().await;
        } => {
            info!("Received SIGTERM, initiating shutdown");
        }
    };

    #[cfg(windows)]
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Received Ctrl+C, initiating shutdown");
        }
    };

    let _ = shutdown_tx.send(());
    info!("Shutting down gracefully...");

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    info!("Shutted down gracefully!");
    Ok(())
}
