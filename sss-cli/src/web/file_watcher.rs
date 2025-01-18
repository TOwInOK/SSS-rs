use std::{path::Path, time::Duration};

use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use sss_std::{prelude::Layouts, themes::Themes};
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::tools::refresh;

pub async fn check_file_loop(
    path: String,
    themes: Option<&Themes>,
    layouts: Option<&Layouts>,
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
) -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::channel(100);

    let mut watcher: RecommendedWatcher = Watcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        },
        notify::Config::default().with_poll_interval(Duration::from_millis(200)),
    )?;
    watcher.watch(Path::new(&path), RecursiveMode::NonRecursive)?;

    info!("Start watch changes for file: {}", &path);
    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                match event {
                    Event { kind, .. } if kind.is_modify() => {
                        info!("File content updated");
                        if let Err(e) = refresh(&path, themes, layouts).await {
                            error!("Load failed: {}", e);
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
