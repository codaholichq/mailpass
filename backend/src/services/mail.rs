use std::net::TcpStream;

use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use trust_dns_resolver::TokioAsyncResolver;

#[derive(Deserialize)]
pub struct Request {
    pub email: String,
}

#[derive(Serialize)]
pub struct Response {
    pub status: bool,
    pub message: String,
    pub data: Value, // can take "", {} or [{}]
}

pub async fn check(Json(payload): Json<Request>) -> impl IntoResponse {
    let email = &payload.email;
    let parts: Vec<&str> = email.split('@').collect();

    if parts.len() != 2 {
        let response = Response {
            status: false,
            message: "Invalid email format".to_string(),
            data: json!({}),
        };
        return Json(json!(response));
    }

    let domain = parts[1];

    // First, check if the domain is reachable
    if let Err(_) = TcpStream::connect((domain, 80)) {
        let response = Response {
            status: false,
            message: "Trash email address".to_string(),
            data: json!({}),
        };
        return Json(json!(response));
    }

    // If the domain is reachable, proceed to check DNS records
    let resolver = TokioAsyncResolver::tokio_from_system_conf().unwrap();

    // Check if the domain has MX records or A records
    let mx_records = resolver.mx_lookup(domain).await;
    let a_records = resolver.ipv4_lookup(domain).await;

    let is_valid = mx_records.is_ok() || a_records.is_ok();

    if is_valid {
        let response = Response {
            status: true,
            message: "Valid email address".to_string(),
            data: json!({}),
        };
        Json(json!(response))
    } else {
        let response = Response {
            status: false,
            message: "Trash email address".to_string(),
            data: json!({}),
        };
        Json(json!(response))
    }
}
