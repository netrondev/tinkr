use axum::extract::{OriginalUri, Request};
use tower_http::trace::TraceLayer;
use tracing::info_span;

/// Creates a pre-configured TraceLayer for HTTP requests
///
/// Features:
/// - Extracts real IP from x-forwarded-for or x-real-ip headers
/// - Falls back to ConnectInfo socket address
/// - Logs method, path, and remote_addr in span
pub fn create_trace_layer() -> TraceLayer<
    tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>,
    fn(&Request) -> tracing::Span,
> {
    TraceLayer::new_for_http().make_span_with(|request: &Request| {
        let path = if let Some(path) = request.extensions().get::<OriginalUri>() {
            path.0.path().to_owned()
        } else {
            request.uri().path().to_owned()
        };

        let remote_addr = request
            .headers()
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.split(',').next())
            .map(|s| s.trim().to_owned())
            .or_else(|| {
                request
                    .headers()
                    .get("x-real-ip")
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.to_owned())
            })
            .or_else(|| {
                request
                    .extensions()
                    .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                    .map(|ci| ci.0.to_string())
            });

        info_span!(
            "http_request",
            method = ?request.method(),
            path,
            remote_addr = ?remote_addr,
        )
    })
}
