//! Downloading, normalization, and cache lookup for SVG payloads.
//! *note:* Icons are cached in `target/icon_cache` for 30 days to avoid unnecessary downloads during repeated builds and test runs.

use std::{
    env, fs,
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

/// Loads SVG content either from the local cache or from the upstream Tabler repository.
pub(crate) fn load_icon(
    style_variant: &str,
    name: &str,
) -> String {
    let style_variant = style_variant.to_lowercase();
    let icon_url = download_link(&style_variant, name);

    let cache_filename = format!("{}_{}.svg", style_variant, name)
        .replace('/', "_")
        .replace('\\', "_");
    let cache_path = cache_dir().join(cache_filename);
    let max_cache_age = Duration::from_secs(60 * 60 * 24 * 30);

    if cache_path.exists() && is_cache_fresh(&cache_path, max_cache_age) {
        fs::read_to_string(&cache_path).expect("Failed to read cached icon")
    } else {
        let content = reqwest::blocking::get(&icon_url)
            .expect("Failed to download icon")
            .text()
            .expect("Failed to read icon content");
        let content = normalize_downloaded_svg(&content, &style_variant, name);

        fs::write(&cache_path, &content).expect("Failed to cache icon");
        content
    }
}

/// Resolves the cache directory used during macro expansion.
fn cache_dir() -> PathBuf {
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let cache_dir = PathBuf::from(target_dir).join("icon_cache");

    fs::create_dir_all(&cache_dir).expect("Failed to create cache directory");
    cache_dir
}

/// Returns `true` when the cached icon file is younger than the configured max age.
fn is_cache_fresh(
    cache_path: &Path,
    max_age: Duration,
) -> bool {
    if let Ok(metadata) = fs::metadata(cache_path) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(duration) = SystemTime::now().duration_since(modified) {
                return duration < max_age;
            }
        }
    }

    false
}

/// Builds the upstream download URL for a specific style/name pair.
fn download_link(
    style_variant: &str,
    name: &str,
) -> String {
    format!(
        "https://raw.githubusercontent.com/tabler/tabler-icons/refs/heads/main/icons/{}/{}.svg",
        style_variant, name
    )
}

/// Strips any leading response noise and returns the SVG payload starting at `<svg`.
fn normalize_downloaded_svg(
    content: &str,
    style_variant: &str,
    name: &str,
) -> String {
    let content = content.trim_start();

    if let Some(position) = content.find("<svg") {
        content[position..].to_string()
    } else {
        panic!(
            "{} is not correct name with this style: {}!\nPlease check tabler for correct name!\nGot content:{:#?}",
            name, style_variant, &content
        );
    }
}
