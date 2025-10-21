use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;

/// Middleware function for logging requests and responses
pub async fn logging_middleware(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    let path = uri.path().to_string();
    let query = uri.query().map(|q| q.to_string());

    // Log the incoming request
    tracing::debug!(
        method = %method,
        path = %path,
        query = ?query,
        "REQ"
    );

    // Call the next middleware/handler
    let response = next.run(req).await;

    // Log the response
    let duration = start.elapsed();
    let status = response.status();

    tracing::debug!(
        method = %method,
        path = %path,
        status = %status,
        duration = ?duration,
        "RES"
    );

    response
}
