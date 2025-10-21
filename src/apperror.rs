#[cfg(feature = "ssr")]
use bytes::Bytes;
#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;

use leptos::prelude::{FromServerFnError, ServerFnErrorErr};

use leptos::server_fn::codec::JsonEncoding;
#[cfg(feature = "ssr")]
use leptos::server_fn::Encodes;

use serde::{Deserialize, Serialize};

use std::env::VarError;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    Db,
    VideoCompressionFailed,
    GenericError(String),
    InputOutputError(String),
    Github(String),
    Reqwest(String),
    MultipartError(String),
    ErrorReason(String),
    ResendError(String),
    AuthError(String),
    DatabaseError(String),
    EnvVarError(String),
    NotFound(String),

    // #[error("Provider error: {0}")]
    Provider(String),
    // #[error("Invalid address: {0}")]
    InvalidAddress(String),
    // #[error("Configuration error: {0}")]
    Config(String),

    DeserializationError(String),

    ServerFnError(ServerFnErrorErr),

    DiskError(String),
}

impl AppError {
    #[track_caller]
    pub fn new(message: impl ToString) -> Self {
        #[cfg(feature = "ssr")]
        {
            let location = std::panic::Location::caller();
            let location_text = format!("{}:{}", location.file(), location.line());

            tracing::error!(
                error = %message.to_string(),
                location = %location_text,
            );
        }

        AppError::GenericError(message.to_string())
    }
}

impl FromServerFnError for AppError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        println!("Converting ServerFnError to : {:?}", value);
        AppError::ServerFnError(value)
    }

    #[cfg(feature = "ssr")]
    fn ser(&self) -> Bytes {
        tracing::error!(
            error = %self,
            "AppError.ser():"
        );

        Self::Encoder::encode(self).unwrap_or_else(|e| {
            Self::Encoder::encode(&Self::from_server_fn_error(
                ServerFnErrorErr::Serialization(e.to_string()),
            ))
            .expect(
                "error serializing should success at least with the \
                 Serialization error",
            )
        })
    }
}

#[cfg(feature = "ssr")]
impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match &self {
            AppError::ErrorReason(text) => {
                tracing::error!(error = %text, "Bad request error");
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    format!("Error: {}", text),
                )
            }
            AppError::NotFound(text) => {
                tracing::warn!(resource = %text, "Resource not found");
                (
                    axum::http::StatusCode::NOT_FOUND,
                    format!("Not found: {}", text),
                )
            }
            AppError::Db => {
                tracing::error!("Database error occurred");
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                )
            }
            AppError::VideoCompressionFailed => {
                tracing::error!("Video compression failed");
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Video processing error".to_string(),
                )
            }
            AppError::GenericError(msg) => {
                tracing::error!(error = %msg, "Generic error");
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error: {}", msg),
                )
            }
            AppError::InputOutputError(msg) => {
                tracing::error!(error = %msg, "I/O error");
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::Github(msg) => {
                tracing::error!(error = %msg, "GitHub API error");
                (
                    axum::http::StatusCode::BAD_GATEWAY,
                    "External service error".to_string(),
                )
            }
            AppError::Reqwest(msg) => {
                tracing::error!(error = %msg, "HTTP request error");
                (
                    axum::http::StatusCode::BAD_GATEWAY,
                    "External service error".to_string(),
                )
            }
            AppError::MultipartError(msg) => {
                tracing::error!(error = %msg, "Multipart upload error");
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    "File upload error".to_string(),
                )
            }
            AppError::ResendError(msg) => {
                tracing::error!(error = %msg, "Email service error");
                (
                    axum::http::StatusCode::BAD_GATEWAY,
                    "Email service error".to_string(),
                )
            }
            AppError::AuthError(msg) => {
                tracing::warn!(error = %msg, "Authentication error");
                (
                    axum::http::StatusCode::UNAUTHORIZED,
                    "Authentication required".to_string(),
                )
            }
            AppError::DatabaseError(msg) => {
                tracing::error!(error = %msg, "Database operation failed");
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                )
            }
            AppError::EnvVarError(msg) => {
                tracing::error!(error = %msg, "Environment configuration error");
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Configuration error".to_string(),
                )
            }
            AppError::Provider(msg) => {
                tracing::error!(error = %msg, "Blockchain provider error");
                (
                    axum::http::StatusCode::BAD_GATEWAY,
                    "Blockchain service error".to_string(),
                )
            }
            AppError::InvalidAddress(msg) => {
                tracing::warn!(error = %msg, "Invalid blockchain address");
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    "Invalid address format".to_string(),
                )
            }
            AppError::Config(msg) => {
                tracing::error!(error = %msg, "Configuration error");
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Configuration error".to_string(),
                )
            }
            AppError::DeserializationError(msg) => {
                tracing::error!(error = %msg, "Deserialization failed");
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    "Invalid data format".to_string(),
                )
            }
            AppError::ServerFnError(msg) => {
                tracing::error!(error = %msg, "Server function error");
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    "Server function error".to_string(),
                )
            }
            AppError::DiskError(msg) => {
                tracing::error!(error = %msg, "Disk operation error");
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Disk operation error".to_string(),
                )
            }
        };

        let body = axum::Json(serde_json::json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

#[cfg(feature = "ssr")]
impl From<surrealdb::Error> for AppError {
    fn from(error: surrealdb::Error) -> Self {
        tracing::error!(error = %error, "SurrealDB error");
        Self::DatabaseError(error.to_string())
    }
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        #[cfg(feature = "ssr")]
        tracing::error!(error = %error, "String error");

        Self::ErrorReason(error)
    }
}

impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        #[cfg(feature = "ssr")]
        tracing::error!(error = %error, "String error");

        Self::ErrorReason(error.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        tracing::error!(error = %error, "I/O error");
        Self::InputOutputError(error.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<hex::FromHexError> for AppError {
    fn from(error: hex::FromHexError) -> Self {
        tracing::error!(error = %error, "Hex decoding error");
        Self::DeserializationError(format!("Hex decoding error: {}", error))
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Db => write!(f, "Database error"),
            AppError::VideoCompressionFailed => write!(f, "Video compression failed"),
            AppError::GenericError(msg) => write!(f, "Generic error: {}", msg),
            AppError::InputOutputError(msg) => write!(f, "Input/Output error: {}", msg),
            AppError::Github(msg) => write!(f, "GitHub error: {}", msg),
            AppError::Reqwest(msg) => write!(f, "Reqwest error: {}", msg),
            AppError::MultipartError(msg) => write!(f, "Multipart error: {}", msg),
            AppError::ErrorReason(msg) => write!(f, "Error reason: {}", msg),
            AppError::ResendError(msg) => write!(f, "Resend error: {}", msg),
            AppError::AuthError(msg) => write!(f, "Authentication error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::EnvVarError(msg) => write!(f, "Environment variable error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Provider(msg) => write!(f, "Provider error: {}", msg),
            AppError::InvalidAddress(msg) => write!(f, "Invalid address: {}", msg),
            AppError::Config(msg) => write!(f, "Configuration error: {}", msg),
            AppError::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            AppError::ServerFnError(msg) => write!(f, "Server function error: {}", msg),
            AppError::DiskError(msg) => write!(f, "Disk error: {}", msg),
        }
    }
}

impl From<VarError> for AppError {
    fn from(error: VarError) -> Self {
        #[cfg(feature = "ssr")]
        tracing::error!(error = %error, "Environment variable error");
        Self::EnvVarError(error.to_string())
    }
}

// Note: Cannot implement From<AppError> for ServerFnError due to conflicting implementations
// Instead, use this helper method in server functions:
#[cfg(feature = "ssr")]
impl AppError {
    #[track_caller]
    pub fn into_server_fn_error(self) -> ServerFnError {
        let location = std::panic::Location::caller();
        let location_text = format!("{}:{}", location.file(), location.line());

        tracing::error!(
            error = %self,
            location = %location_text,
            "Converting AppError to ServerFnError"
        );

        ServerFnError::new(self.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        tracing::error!(error = %error, "HTTP client error");
        Self::Reqwest(error.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<leptos::prelude::ServerFnError> for AppError {
    fn from(error: leptos::prelude::ServerFnError) -> Self {
        tracing::error!(error = %error, "Server function error");
        Self::GenericError(format!("{error}"))
    }
}

#[cfg(feature = "ssr")]
impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        tracing::error!(error = %error, "JSON serialization error");
        Self::DeserializationError(format!("JSON error: {}", error))
    }
}

/// This will help print out the offending part of the JSON that caused the deserialization error.
///
/// ### EXAMPLE:
///  ```rs  
///    let parsed = serde_json::from_str::<Vec<ClockifyTimeEntry>>(&time_sheets_text)
///             .map_err(|e| serde_detail_error(&e, &time_sheets_text))?;
/// ```
///
#[cfg(feature = "ssr")]
pub fn serde_detail_error(e: &serde_json::Error, original: &str) -> AppError {
    tracing::error!(
        error = %e,
        line = e.line(),
        column = e.column(),
        "JSON deserialization error"
    );

    // Print the problematic part of the JSON
    if let Some(start) = original.char_indices().nth(e.column().saturating_sub(20)) {
        let snippet: String = original[start.0..].chars().take(40).collect();
        tracing::error!(snippet = %snippet, "Error context");

        AppError::new(format!(
            "Deserialization error: {} snippet: {}",
            e.to_string(),
            snippet
        ))
    } else {
        AppError::new(format!("Deserialization error: {}", e.to_string()))
    }
}

#[cfg(feature = "ssr")]
impl From<resend_rs::Error> for AppError {
    fn from(err: resend_rs::Error) -> Self {
        AppError::ResendError(err.to_string())
    }
}

impl serde::ser::StdError for AppError {}
