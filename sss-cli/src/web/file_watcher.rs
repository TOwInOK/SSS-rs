use notify::{Event, RecursiveMode, Watcher, recommended_watcher};
use sss_std::prelude::{HtmlLayouts, Themes};
use std::path::Path;
use tokio::sync::mpsc;
use tracing::{debug, error, info, trace};
use xxhash_rust::xxh3::xxh3_64;

use crate::{settings::services::Services, tools::refresh};

// Function to watch a file and refresh settings upon modification
pub async fn check_file_loop(
    path: &Path,
    themes: Option<&Themes>,
    layouts: Option<&HtmlLayouts>,
    services: Option<&Services>,
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
) -> anyhow::Result<()> {
    // Create a channel to receive file events
    let (tx, mut rx) = mpsc::channel(100);

    // Define a watcher callback
    let watcher = move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            let _ = tx.try_send(event);
        }
    };

    // Initialize the file watcher
    let mut watcher = recommended_watcher(watcher)?;
    watcher.watch(Path::new(&path), RecursiveMode::NonRecursive)?;

    // Read the file content and calculate the initial hash
    let file_content = tokio::fs::read(&path).await?;
    let mut prevision_processed_hash = calculate_xxhash(&file_content);
    drop(file_content);

    info!("Start watching changes for file: {}", &path.display());

    loop {
        tokio::select! {
            // Receive file events
            Some(event) = rx.recv() => {
                    match event {
                        Event { kind, .. } if kind.is_modify() => {
                            trace!("Received modify event for {}", path.display());
                            // Read the file content
                            let file_content = tokio::fs::read(&path).await?;
                            // Calculate the current hash
                            let current_hash = calculate_xxhash(&file_content);
                            // Check if the hash has changed
                            if current_hash != prevision_processed_hash {
                                debug!("Hash is different:\nprevision: {}\ncurrent: {}", prevision_processed_hash, current_hash);
                                // Update the hash to the current one
                                prevision_processed_hash = current_hash;
                                    // Refresh settings
                                    if let Err(e) = refresh(path, themes, layouts, services).await {
                                        error!("Failed to refresh settings: {}", e);
                                    }
                            } else {
                                info!("Nothing to change");
                                trace!("Skipped identical update for {}", path.display());
                            }
                        }
                        _ => (),
                    }
            }
            // Handle shutdown signal
            _ = shutdown_rx.recv() => {
                info!("Shutting down file watcher");
                break;
            }
        }
    }

    Ok(())
}

// Function to calculate the xxHash of data
fn calculate_xxhash(data: &[u8]) -> u64 {
    xxh3_64(data)
}
