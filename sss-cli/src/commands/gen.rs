use tracing::{info, instrument};

use crate::{cli::args::Args, tools::refresh, HTML};

#[instrument(skip(args))]
pub async fn command_gen(args: &Args) -> anyhow::Result<()> {
    info!("Start generating html");
    info!("Start loading config & theme");
    refresh(&args.config_path, args.theme.as_ref(), args.layout.as_ref())
        .await
        .map_err(|e| anyhow::anyhow!("Load failed with error: {}", e))?;
    info!("Load successfully!");
    info!("Try to generate html");
    info!("Html generated successfully!");
    info!("Try to write html on disk by path: {}", &args.file_output);
    tokio::fs::write(&args.file_output, &*HTML.read().await).await?;
    info!("Html successfully saved by path: {}", &args.file_output);
    Ok(())
}
