use std::path::Path;

use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;
use tracing::info;

use crate::tools::refresh_settings;

pub async fn check_file_loop(
    path: String,
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
) -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::channel(100);

    let mut last_modified = std::time::Instant::now();
    let debounce_duration = std::time::Duration::from_millis(100);

    let mut watcher: RecommendedWatcher = Watcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        },
        notify::Config::default(),
    )?;
    watcher.watch(Path::new(&path), RecursiveMode::NonRecursive)?;

    info!("Start watch changes for file: {}", &path);

    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                match event {
                    Event { kind, .. } if kind.is_modify() => {
                        let now = std::time::Instant::now();
                        if now.duration_since(last_modified) >= debounce_duration {
                            info!("File content updated\nTry save into memory");
                            refresh_settings(&path).await;
                            last_modified = now;
                        }
                    }
                    _ => (),
                }
            }
            _ = shutdown_rx.recv() => {
                info!("Shutting down file watcher");
                break;
            }
        }
    }

    Ok(())
}
