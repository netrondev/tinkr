use axum::{http::StatusCode, response::IntoResponse};

/// Standard health check endpoint handler
/// Returns 200 OK with "OK" body
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
