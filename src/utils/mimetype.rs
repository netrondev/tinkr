#[cfg(feature = "ssr")]
use std::path::Path;

#[cfg(feature = "ssr")]
use crate::AppError;

#[derive(Debug, Clone)]
pub enum MimeType {
    Image,
    Video,
    Audio,
    Document,
    Archive,
    Other(String),
}

#[cfg(feature = "ssr")]
impl MimeType {
    fn from_mime_str(mime_str: &str) -> Self {
        match mime_str {
            // Prefix matches
            s if s.starts_with("image/") => MimeType::Image,
            s if s.starts_with("video/") => MimeType::Video,
            s if s.starts_with("audio/") => MimeType::Audio,

            // Exact matches for documents
            "application/pdf"
            | "application/msword"
            | "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
                MimeType::Document
            }

            // Exact matches for archives
            "application/zip" | "application/x-tar" | "application/x-gzip" => MimeType::Archive,

            // Everything else
            _ => MimeType::Other(mime_str.to_string()),
        }
    }
}

/// Detects the MIME type of a file at the given path using the `infer` crate.
#[cfg(feature = "ssr")]
pub async fn mimetype<P>(path: P) -> Result<MimeType, AppError>
where
    P: AsRef<Path>,
{
    use std::fs::File;
    use std::io::Read;

    // Try to open and read the file
    let mut file = File::open(&path)
        .map_err(|e| AppError::GenericError(format!("Failed to open file: {}", e)))?;

    // Read enough bytes for infer to detect the type (typically 4096 bytes is enough)
    let mut buffer = Vec::new();
    let bytes_to_read = 4096;
    buffer.resize(bytes_to_read, 0);

    let bytes_read = file
        .read(&mut buffer)
        .map_err(|e| AppError::GenericError(format!("Failed to read file: {}", e)))?;

    buffer.truncate(bytes_read);

    // Use infer to detect the MIME type
    if let Some(kind) = infer::get(&buffer) {
        return Ok(MimeType::from_mime_str(kind.mime_type()));
    }

    Err(AppError::GenericError(
        "Could not determine MIME type".to_string(),
    ))
}
