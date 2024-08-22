use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{field, info_span, Span};

use crate::services::email_validator::validate_email;

pub fn setup_routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/email", post(validate_email))
        .route("/health", get(health))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http().make_span_with(make_span)))
}

async fn home() -> Json<serde_json::Value> {
    let response = json!({
        "message": "Welcome to the MailPass API V1"
    });
    Json(response)
}

async fn health() -> StatusCode {
    StatusCode::NO_CONTENT
}

fn make_span(request: &Request<Body>) -> Span {
    let headers = request.headers();
    let path = request.uri().path();
    info_span!("incoming request", path, ?headers, trace_id = field::Empty)
}
