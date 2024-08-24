use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::services::email_validator::validate_email;

pub fn setup_routes() -> Router {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    Router::new()
        .route("/", get(home))
        .route("/email", post(validate_email))
        .route("/health", get(health))
        .route("/error", get(error))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
}

async fn home() -> Json<serde_json::Value> {
    let response = json!({
        "message": "Welcome to the MailPass API V1"
    });
    Json(response)
}

async fn health() -> Result<&'static str, StatusCode> {
    Ok("Service is up and running")
}

async fn error() -> Result<&'static str, StatusCode> {
    Err(StatusCode::SERVICE_UNAVAILABLE)
}
