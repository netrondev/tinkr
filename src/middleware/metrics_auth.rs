use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use base64::prelude::*;

/// Middleware for Basic Auth protection of metrics endpoints
///
/// In debug mode, auth is skipped to allow easy browser access during development.
/// In release mode, requires Basic Auth with credentials from environment variables:
/// - METRICS_USERNAME (default: "metrics")
/// - METRICS_PASSWORD (default: "metrics123")
#[tracing::instrument(skip(request, next))]
pub async fn metrics_auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // SKIP AUTH IN DEBUG MODE
    // this lets /metrics work in the browser without auth while developing.
    if cfg!(debug_assertions) {
        return Ok(next.run(request).await);
    }

    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if let Some(auth_header) = auth_header {
        if auth_header.starts_with("Basic ") {
            let encoded = &auth_header[6..];
            if let Ok(decoded_bytes) = BASE64_STANDARD.decode(encoded) {
                if let Ok(decoded_str) = String::from_utf8(decoded_bytes) {
                    let parts: Vec<&str> = decoded_str.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        let username = parts[0];
                        let password = parts[1];

                        // Check against environment variables
                        let expected_username = std::env::var("METRICS_USERNAME")
                            .unwrap_or_else(|_| "metrics".to_string());
                        let expected_password = std::env::var("METRICS_PASSWORD")
                            .unwrap_or_else(|_| "metrics123".to_string());

                        if username == expected_username && password == expected_password {
                            return Ok(next.run(request).await);
                        }
                    }
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
