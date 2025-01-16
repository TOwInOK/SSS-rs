use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};
use sss_std::prelude::Layouts;
use tokio::sync::broadcast::Receiver;
use tracing::info;

use crate::tools::layout_build;

pub async fn serve(
    address: String,
    layouts: Layouts,
    mut shutdown_rx: Receiver<()>,
) -> anyhow::Result<()> {
    let app = Router::new().route("/", get(move || root(layouts)));

    info!("try to bind {} address", &address);
    let addr = address.parse::<SocketAddr>()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("address {} binded", &address);

    info!("run web server on {}", &address);

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            shutdown_rx.recv().await.ok();
            info!("Shutting down Web server");
        })
        .await?;
    Ok(())
}

async fn root(layouts: Layouts) -> Html<String> {
    match layout_build(&layouts).await {
        Ok(html) => Html(html),
        Err(e) => Html(format!("Error building layout: {}", e)),
    }
}
