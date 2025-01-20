use std::path::Path;

use tracing::{info, instrument};

use crate::{
    cli::{args::Args, subcommands::GenType},
    tools::{refresh_html, refresh_pdf, refresh_png, refresh_settings},
    HTML, PDF, PNG,
};

#[instrument(skip(args, output_type, output_name))]
pub async fn command_gen(
    output_type: &GenType,
    output_name: &str,
    args: &Args,
) -> anyhow::Result<()> {
    info!("Start generating {}", &output_type);
    info!("Start loading config & theme");
    refresh_settings(&args.config_path, args.theme.as_ref(), args.layout.as_ref())
        .await
        .map_err(|e| anyhow::anyhow!("Load failed with error: {}", e))?;
    refresh_html()
        .await
        .map_err(|e| anyhow::anyhow!("Html refresh failed with error: {}", e))?;
    info!("Load successfully!");
    info!("Try to generate {}", &output_type);
    info!("{} generated successfully!", &output_type);

    let output_file_name = format!("{output_name}.{}", &output_type);
    let path = Path::new(&output_file_name);
    info!(
        "Try to write file on disk by path: {}",
        path.to_str().unwrap_or("Invalid path")
    );

    use tokio::fs::write;
    match output_type {
        GenType::Html => write(path, &*HTML.read().await).await?,
        GenType::Png => {
            refresh_png()
                .await
                .map_err(|e| anyhow::anyhow!("Png refresh failed with error: {}", e))?;
            write(path, &*PNG.read().await).await?
        }
        GenType::Pdf => {
            refresh_pdf()
                .await
                .map_err(|e| anyhow::anyhow!("Pdf refresh failed with error: {}", e))?;
            write(path, &*PDF.read().await).await?
        }
    }

    Ok(())
}
