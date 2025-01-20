use std::{path::Path, time::Duration};

use notify::{recommended_watcher, Event, RecursiveMode, Watcher};
use sss_std::{prelude::Layouts, themes::Themes};
use tokio::sync::mpsc;
use tracing::{error, info, trace};

use crate::tools::refresh;

pub async fn check_file_loop(
    path: String,
    themes: Option<&Themes>,
    layouts: Option<&Layouts>,
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
) -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::channel(1);

    let p = move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            let _ = tx.try_send(event);
        }
    };

    let mut watcher = recommended_watcher(p)?;
    watcher.watch(Path::new(&path), RecursiveMode::NonRecursive)?;

    info!("Start watching changes for file: {}", &path);

    let mut last_event_time = tokio::time::Instant::now() - Duration::from_millis(1);

    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                let now = tokio::time::Instant::now();
                if now.duration_since(last_event_time) >= Duration::from_millis(300) {
                    last_event_time = now;

                    match event {
                        Event { kind, .. } if kind.is_modify() => {
                            trace!("Received modify event");
                            if let Err(e) = refresh(&path, themes, layouts).await {
                                error!("Load failed: {}", e);
                            }
                        }
                        _ => (),
                    }
                } else {
                    trace!("Ignored event due to cooldown");
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
