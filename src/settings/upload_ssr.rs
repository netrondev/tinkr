#[cfg(feature = "ssr")]
use axum::{extract::Multipart, Json};

#[cfg(feature = "ssr")]
use axum_extra::extract::cookie::CookieJar;

#[cfg(feature = "ssr")]
use crate::AppError;

#[cfg(feature = "ssr")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub path: String,
}

#[cfg(feature = "ssr")]
pub async fn upload_avatar_handler(
    jar: CookieJar,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    use std::fs;
    use std::path::Path;

    // Get the session_token cookie
    let session_token = jar.get("session_token").map(|cookie| cookie.value());

    let user = if let Some(token) = session_token {
        use crate::user::AdapterUser;

        AdapterUser::get_user_from_session(token.to_string()).await?
    } else {
        return Err(AppError::AuthError("No session token found".into()));
    };

    // Create uploads directory if it doesn't exist
    let upload_dir = "uploads/avatars";
    fs::create_dir_all(upload_dir)?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::MultipartError(e.to_string()))?
    {
        if field.name() == Some("file") {
            let filename = field.file_name().map(|f| f.to_string());
            let content_type = field.content_type().map(|ct| ct.to_string());
            let data = field
                .bytes()
                .await
                .map_err(|e| AppError::MultipartError(e.to_string()))?;

            // Validate content type
            if let Some(ct) = &content_type {
                if !ct.starts_with("image/") {
                    return Err(AppError::ErrorReason(
                        "Invalid file type. Only images are allowed.".into(),
                    ));
                }
            }

            // Validate file size (5MB max)
            if data.len() > 5 * 1024 * 1024 {
                return Err(AppError::ErrorReason("File size exceeds 5MB limit.".into()));
            }

            // Generate unique filename
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| AppError::ErrorReason(e.to_string()))?
                .as_millis();

            let extension = filename
                .as_ref()
                .and_then(|f| Path::new(f).extension())
                .and_then(|ext| ext.to_str())
                .unwrap_or("jpg");

            let final_filename = format!("avatar_{}_{}.{}", user.id.key(), timestamp, extension);
            let saved_path = format!("{}/{}", upload_dir, final_filename);

            // Save file
            fs::write(&saved_path, data)?;

            return Ok(Json(UploadResponse { path: saved_path }));
        }
    }

    Err(AppError::ErrorReason("No file found in upload".into()))
}
