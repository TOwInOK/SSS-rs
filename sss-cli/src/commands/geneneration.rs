use std::path::Path;

use tracing::{info, instrument};

use crate::{
    HTML, PDF, PNG,
    cli::{args::Args, subcommands::GenType},
    settings::services::Services,
    tools::{refresh_html, refresh_pdf, refresh_png, refresh_settings},
};

#[instrument(skip_all)]
pub async fn command_generation(
    output_type: &GenType,
    output_name: &str,
    args: &Args,
) -> anyhow::Result<()> {
    info!("Start generating {}", &output_type);
    info!("Start loading config & theme");
    let services = match &args.command {
        crate::cli::subcommands::Commands::Run {
            watch: _,
            serve: _,
            address: _,
            html,
            png,
            pdf,
            json,
            health,
            share,
            api,
        } => Some(Services {
            html: *html,
            png: *png,
            pdf: *pdf,
            json: *json,
            health: *health,
            share: *share,
            api: *api,
        }),
        _ => None,
    };
    refresh_settings(
        Path::new(&args.config_path),
        args.theme.as_ref(),
        args.layout.as_ref(),
        services.as_ref(),
    )
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
