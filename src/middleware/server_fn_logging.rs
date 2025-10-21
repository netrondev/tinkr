use axum::{body::Body, extract::Request, response::Response};
use futures::future::BoxFuture;
use http::StatusCode;
use std::task::{Context, Poll};
use tower::{Layer, Service};
use tracing::{error, info};

pub enum ServerFnLoggingStatus {
    Informational,
    Success,
    Redirection,
    ClientError,
    ServerError,
}

impl From<StatusCode> for ServerFnLoggingStatus {
    fn from(status: StatusCode) -> Self {
        if status.is_informational() {
            ServerFnLoggingStatus::Informational
        } else if status.is_success() {
            ServerFnLoggingStatus::Success
        } else if status.is_redirection() {
            ServerFnLoggingStatus::Redirection
        } else if status.is_client_error() {
            ServerFnLoggingStatus::ClientError
        } else {
            ServerFnLoggingStatus::ServerError
        }
    }
}

/// Add to your app:
/// ```
/// let app = Router::new()
///     .layer(ServerFnLoggingLayer)
///     // ... rest of your routes
/// ```
#[derive(Clone)]
pub struct ServerFnLoggingLayer;

impl<S> Layer<S> for ServerFnLoggingLayer {
    type Service = ServerFnLoggingService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ServerFnLoggingService { inner }
    }
}

#[derive(Clone)]
pub struct ServerFnLoggingService<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for ServerFnLoggingService<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let path = req.uri().path().to_string();
        let method = req.method().clone();

        let future = self.inner.call(req);

        Box::pin(async move {
            let start = std::time::Instant::now();
            let result = future.await;
            let duration = start.elapsed();

            match &result {
                Ok(response) => {
                    let status: StatusCode = response.status();
                    let statustype: ServerFnLoggingStatus = status.into();

                    // Note: Axum's Body type doesn't allow easy access to content without consuming it
                    // To print response bodies, consider using a different middleware approach or
                    // implement body printing at the handler level
                    tracing::debug!("Response status: {} for path: {}", status, path);

                    match statustype {
                        ServerFnLoggingStatus::Informational => {
                            info!(
                                path = %path,
                                method = %method,
                                status = %status,
                                duration = ?duration,
                                ""
                            );
                        }
                        ServerFnLoggingStatus::Success => {
                            info!(
                                path = %path,
                                method = %method,
                                status = %status,
                                duration = ?duration,
                                ""
                            );
                        }
                        ServerFnLoggingStatus::Redirection => {
                            info!(
                                path = %path,
                                method = %method,
                                status = %status,
                                duration = ?duration,
                                ""
                            );
                        }
                        ServerFnLoggingStatus::ClientError => {
                            error!(
                                path = %path,
                                method = %method,
                                status = %status,
                                duration = ?duration,
                                ""
                            );
                        }
                        ServerFnLoggingStatus::ServerError => {
                            error!(
                                path = %path,
                                method = %method,
                                status = %status,
                                duration = ?duration,
                                ""
                            );
                        }
                    }
                }
                Err(_) => {
                    error!(
                        path = %path,
                        method = %method,
                        duration = ?duration,
                        "Server function failed"
                    );
                }
            }

            result
        })
    }
}
