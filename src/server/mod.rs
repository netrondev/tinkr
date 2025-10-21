use axum::serve::ListenerExt;
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// Creates an optimized TCP listener with TCP_NODELAY enabled
///
/// TCP_NODELAY disables Nagle's algorithm, reducing latency for small packets
/// This is beneficial for web applications with frequent small responses
pub async fn create_optimized_listener(
    addr: SocketAddr,
) -> Result<impl ListenerExt, std::io::Error> {
    tracing::info!("listening on http://{}", &addr);

    Ok(TcpListener::bind(&addr).await?.tap_io(|tcp_stream| {
        if let Err(err) = tcp_stream.set_nodelay(true) {
            tracing::warn!("failed to set TCP_NODELAY on incoming connection: {err:#}");
        }
    }))
}
