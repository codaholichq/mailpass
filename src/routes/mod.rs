use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_json::json;

use crate::services::email_validator::validate_email;

pub fn setup_routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/email", post(validate_email))
}

async fn home() -> Json<serde_json::Value> {
    let response = json!({
        "message": "Welcome to the MailPass API V1"
    });
    Json(response)
}
