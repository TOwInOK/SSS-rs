use tokio::fs;
use tracing::{info, instrument};

use crate::{
    cli::args::Args,
    tools::{layout_build, load},
};

#[instrument(skip(args))]
pub async fn command_gen(args: &Args) -> anyhow::Result<()> {
    info!("Start generating html");
    info!("Start loading config & theme");
    load(&args.config_path, &args.theme, &args.layout)
        .await
        .map_err(|e| anyhow::anyhow!("Load failed: {}", e))?;
    info!("Load successfully!");
    info!("Try to generate html");
    let html = layout_build(&args.layout)
        .await
        .map_err(|e| anyhow::anyhow!("Layout build error: {}", e))?;
    info!("Html generated successfully!");
    info!("Try to write html on disk by path: {}", &args.file_output);
    fs::write(&args.file_output, &html).await?;
    info!("Html successfully saved by path: {}", &args.file_output);
    Ok(())
}
