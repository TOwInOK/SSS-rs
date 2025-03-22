use sss_std::prelude::{HtmlLayouts, Themes};
use tokio::signal;
#[cfg(not(windows))]
use tokio::signal::unix::SignalKind;
use tracing::{error, info, instrument, trace, warn};

use crate::{
    SETTINGS,
    settings::services::Services,
    tools::refresh,
    web::{file_watcher::check_file_loop, serve::serve},
};

#[instrument(skip_all)]
#[allow(clippy::too_many_arguments)]
pub async fn command_run(
    is_watch: bool,
    is_web: bool,
    path: &str,
    layouts: &Option<HtmlLayouts>,
    address: &str,
    themes: &Option<Themes>,
    html: bool,
    png: bool,
    pdf: bool,
    json: bool,
    health: bool,
    share: bool,
    api: bool,
) -> anyhow::Result<()> {
    info!("Start run command");
    if !is_web && !is_watch {
        warn!("Nothing to run!\nTry to use sss_cli run -h to show avaiable options");
        return Ok(());
    }

    let services = if html || png || pdf || json || health || share || api {
        Some(Services {
            html,
            png,
            pdf,
            json,
            health,
            share,
            api,
        })
    } else {
        None
    };

    refresh(path, themes.as_ref(), layouts.as_ref(), services.as_ref())
        .await
        .map_err(|e| anyhow::anyhow!("Load failed: {}", e))?;

    let settings_services = &SETTINGS.read().await.services;
    let (shutdown_tx, _) = tokio::sync::broadcast::channel(1);
    trace!("{:#?}", &settings_services);
    if is_watch {
        if settings_services.html
            || settings_services.png
            || settings_services.pdf
            || settings_services.json
            || settings_services.health
            || settings_services.share
            || settings_services.api
        {
            let path_clone = path.to_string();
            let themes = themes.clone();
            let layouts = layouts.clone();
            let services = services.clone();
            let shutdown_rx = shutdown_tx.subscribe();

            tokio::spawn(async move {
                if let Err(e) = check_file_loop(
                    path_clone,
                    themes.as_ref(),
                    layouts.as_ref(),
                    services.as_ref(),
                    shutdown_rx,
                )
                .await
                {
                    error!("File watching error: {}", e);
                }
            });
        } else {
            warn!(
                "any flag too launch must be set! Awaiable flags:\n
            html,
            png,
            pdf,
            json,
            health,
            share,
            api,"
            );
        }
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
