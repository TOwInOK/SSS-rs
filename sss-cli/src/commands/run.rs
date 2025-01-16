use tokio::signal::{self, unix::SignalKind};
use tracing::{error, info, warn};

use crate::web::file_watcher::check_file_loop;

pub async fn command_run(
    is_watch: bool,
    is_web: bool,
    path: String,
) -> anyhow::Result<()> {
    if !is_web && !is_watch {
        warn!("Nothing to run!\nTry to use sss_cli run -h to show avaiable options");
        return Ok(());
    }

    let (shutdown_tx, _) = tokio::sync::broadcast::channel(1);

    if is_watch {
        let path_clone = path.clone();
        let shutdown_rx = shutdown_tx.subscribe();

        tokio::spawn(async move {
            if let Err(e) = check_file_loop(path_clone, shutdown_rx).await {
                error!("File watching error: {}", e);
            }
        });
    }

    if is_web {
        todo!()
    }

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
    }

    let _ = shutdown_tx.send(());
    info!("Shutting down gracefully...");

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    info!("Shutted down gracefully!");
    Ok(())
}
