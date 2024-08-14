use std::net::TcpStream;

use axum::{response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;
use trust_dns_resolver::TokioAsyncResolver;

#[derive(Deserialize)]
pub struct EmailRequest {
    pub email: String,
}

pub async fn validate_email(Json(payload): Json<EmailRequest>) -> impl IntoResponse {
    let email = &payload.email;
    let parts: Vec<&str> = email.split('@').collect();

    if parts.len() != 2 {
        return Json(json!({ "valid": false, "reason": "Invalid email format" }));
    }

    let domain = parts[1];

    // First, check if the domain is reachable
    if let Err(_) = TcpStream::connect((domain, 80)) {
        return Json(json!({ "valid": false, "reason": "Domain is not reachable" }));
    }

    // If the domain is reachable, proceed to check DNS records
    let resolver = TokioAsyncResolver::tokio_from_system_conf().unwrap();

    // Check if the domain has MX records or A records
    let mx_records = resolver.mx_lookup(domain).await;
    let a_records = resolver.ipv4_lookup(domain).await;

    let is_valid = mx_records.is_ok() || a_records.is_ok();

    if is_valid {
        Json(json!({ "valid": true }))
    } else {
        Json(json!({ "valid": false, "reason": "Domain does not exist" }))
    }
}
