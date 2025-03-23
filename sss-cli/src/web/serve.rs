use axum::{
    Json, Router,
    body::Bytes,
    http::{StatusCode, header},
    response::{Html, IntoResponse},
    routing::get,
};
use serde::Serialize;
use serde_json::json;
use tokio::sync::broadcast::Receiver;
use tracing::{error, info, instrument};
use utoipa::{OpenApi, ToSchema};
use utoipa_scalar::{Scalar, Servable};

use crate::{HTML, PDF, PNG, SETTINGS, settings::SSSCliSettings};

#[derive(OpenApi)]
#[openapi(
    paths(
        root,
        get_png,
        get_pdf,
        get_json,
        healthcheck,
        gen_base64
    ),
    components(
        schemas(SSSCliSettings),
        schemas(HealthResponse),
        schemas(Base64Out)
    ),
    tags(
        (name = "card-generator", description = "Endpoints for generating and retrieving card html/png/pdf documents"),
        (name = "raw-data", description = "Endpoints for accessing raw configuration data"),
        (name = "system", description = "System and monitoring endpoints")
    ),
    security(()),
    info(
        title = "SSS-cli",
        description = "SSS-rs (Skill, Slick, Style) is a library and CLI tool for creating stylish developer cards.",
        license(
            name = "Apache License 2.0",
            url = "https://github.com/TOwInOK/SSS-rs/blob/main/LICENSE"
        ),
        contact(
            name = "TOwInOK",
            url = "https://github.com/TOwInOK"
        )
    ),
    external_docs(
        url = "https://github.com/TOwInOK/SSS-rs",
        description = "Main repo"
    ),
    servers(
        (url = "http://localhost:8081", description = "Local development server")
    )
)]
struct ApiDoc;

pub async fn serve(
    address: String,

    mut shutdown_rx: Receiver<()>,
) -> anyhow::Result<()> {
    let services = &SETTINGS.read().await.services;

    let mut app = Router::new().route("/", get(root)); // Базовый маршрут всегда доступен

    if services.png {
        app = app.route("/png", get(get_png));
    }

    if services.pdf {
        app = app.route("/pdf", get(get_pdf));
    }

    if services.json {
        app = app.route("/json", get(get_json));
    }

    if services.health {
        app = app.route("/health", get(healthcheck));
    }

    if services.share {
        app = app.route("/share", get(gen_base64));
    }

    if services.api {
        app = app.merge(Scalar::with_url("/api", ApiDoc::openapi()));
    }

    info!("try to bind {} address", &address);
    let addr = address.parse::<std::net::SocketAddr>()?;
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

#[utoipa::path(
    get,
    path = "/",
    tag = "card-generator",
    responses(
        (status = 200, description = "Successfully retrieved HTML page with card generator interface", content_type = "text/html")
    ),
    summary = "Get card generator HTML page",
    description = "Returns the main HTML page containing the card generator interface"
)]
#[instrument]
async fn root() -> Html<String> {
    Html(HTML.read().await.clone())
}

#[utoipa::path(
    get,
    path = "/pdf",
    tag = "card-generator",
    responses(
        (status = 200, description = "Successfully retrieved PDF document", content_type = "application/pdf"),
        (status = 404, description = "PDF document has not been generated yet or is not available")
    ),
    summary = "Get generated PDF document",
    description = "Returns the generated card document in PDF format"
)]
#[instrument]
async fn get_pdf() -> impl IntoResponse {
    let data = PDF.read().await.clone();

    if data.is_empty() {
        return (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/plain")],
            Bytes::from("PDF not found"),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/pdf"),
            (header::CONTENT_LENGTH, data.len().to_string().as_str()),
            (
                header::CONTENT_DISPOSITION,
                "inline; filename=\"document.pdf\"",
            ),
        ],
        Bytes::from(data),
    )
        .into_response()
}
#[utoipa::path(
    get,
    path = "/png",
    tag = "card-generator",
    responses(
        (status = 200, description = "Successfully retrieved PNG image", content_type = "image/png"),
        (status = 404, description = "Image has not been generated yet or is not available")
    ),
    summary = "Get generated PNG image",
    description = "Returns the generated card image in PNG format"
)]
#[instrument]
async fn get_png() -> impl IntoResponse {
    let data = PNG.read().await.clone();

    if data.is_empty() {
        return (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/plain")],
            Bytes::from("Image not found"),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "image/png"),
            (header::CONTENT_LENGTH, data.len().to_string().as_str()),
        ],
        Bytes::from(data),
    )
        .into_response()
}
#[utoipa::path(
    get,
    path = "/json",
    tag = "raw-data",
    responses(
        (status = 200, description = "Successfully retrieved current settings", body = SSSCliSettings)
    ),
    summary = "Get current settings",
    description = "Returns the current configuration settings of the service"
)]
async fn get_json() -> Json<SSSCliSettings> {
    Json(SETTINGS.read().await.clone())
}

#[derive(Serialize, ToSchema)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "system",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    ),
    summary = "Check service health",
    description = "Returns the health status of the service and its version"
)]
#[instrument]
async fn healthcheck() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy",
        version: env!("CARGO_PKG_VERSION"),
    })
}
#[derive(Clone, ToSchema, Serialize)]
struct Base64Out {
    pub base64: String,
}

#[utoipa::path(
    get,
    path = "/share",
    tag = "raw-data",
    responses(
        (status = 200, description = "encoded toml config", body = Base64Out)
    ),
    summary = "Convert settings to base64",
    description = "Return converted settings to toml to base64"
)]
#[instrument]
async fn gen_base64() -> impl IntoResponse {
    let toml = toml::to_string(&*SETTINGS.read().await);
    match toml {
        Ok(e) => {
            let base64 = base64_light::base64_encode(&e);
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/json")],
                Bytes::from(json!(Base64Out { base64 }).to_string()),
            )
                .into_response()
        }
        Err(e) => {
            error!("Toml error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, "text/plain")],
                Bytes::from("toml converter got error"),
            )
                .into_response()
        }
    }
}
