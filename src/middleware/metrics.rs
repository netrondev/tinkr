use axum::Router;
use axum_prometheus::PrometheusMetricLayer;

use super::metrics_auth::metrics_auth_middleware;

/// Creates a Prometheus metrics router with Basic Auth protection
///
/// Returns a tuple of (prometheus_layer, metrics_router)
/// The prometheus_layer should be added to your main app router
/// The metrics_router should be merged into your main app router
///
/// # Example
/// ```
/// let (prometheus_layer, metrics_router) = tinkr::middleware::create_metrics_setup();
/// let app = Router::new()
///     .merge(metrics_router)
///     .layer(prometheus_layer);
/// ```
pub fn create_metrics_setup() -> (PrometheusMetricLayer<'static>, Router) {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let metrics_router = Router::new()
        .route(
            "/metrics",
            axum::routing::get(|| async move { metric_handle.render() }),
        )
        .layer(axum::middleware::from_fn(metrics_auth_middleware));

    (prometheus_layer, metrics_router)
}
