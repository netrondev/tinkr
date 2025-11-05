use leptos::prelude::*;

#[cfg(feature = "ssr")]
use image::{ImageFormat, imageops::FilterType};

#[cfg(feature = "ssr")]
pub mod image_handler {
    use crate::AppError;
    use axum::{
        extract::Query,
        http::{StatusCode, header},
        response::{IntoResponse, Response},
    };
    use image::{ImageFormat, imageops::FilterType};
    use serde::Deserialize;
    use std::path::Path;

    #[derive(Deserialize)]
    pub struct ImageParams {
        pub src: String,
        #[serde(default)]
        pub width: Option<u32>,
        #[serde(default)]
        pub height: Option<u32>,
        #[serde(default)]
        pub quality: Option<u8>,
    }

    /// Get the site root directory from LEPTOS_SITE_ROOT env var, defaults to "target/site"
    fn get_site_root() -> String {
        std::env::var("LEPTOS_SITE_ROOT").unwrap_or_else(|_| "target/site".to_string())
    }

    /// Server function to get the optimized image URL
    #[tracing::instrument(name = "get_optimized_image_url", skip_all, fields(src = %src, width = ?width, height = ?height, quality = ?quality))]
    pub async fn get_optimized_image_url_ssr(
        src: String,
        width: Option<u32>,
        height: Option<u32>,
        quality: Option<u8>,
    ) -> Result<String, AppError> {
        use std::path::Path;

        let site_root = get_site_root();

        // Resolve URL path to filesystem path (same logic as image_handler)
        let resolved_src = if src.contains("..") {
            return Err(AppError::new("Path traversal attempt detected".to_string()));
        } else if src.starts_with('/') {
            if src.starts_with("/uploads/") {
                src.trim_start_matches('/').to_string()
            } else if src.starts_with("/images/")
                || src.starts_with("/logo")
                || src.starts_with("/favicon")
            {
                format!("{}{}", site_root, src)
            } else {
                format!("{}{}", site_root, src)
            }
        } else {
            src.clone()
        };

        // Validate input path exists
        if !Path::new(&resolved_src).exists() {
            return Err(AppError::new(format!(
                "Source image not found: {} (resolved to {})",
                src, resolved_src
            )));
        }

        // Generate optimized path
        let quality = quality.unwrap_or(80);
        let cache_key = format!(
            "{}_{}x{}_q{}",
            src.replace("/", "_").replace(".", "_"),
            width.unwrap_or(0),
            height.unwrap_or(0),
            quality
        );

        let optimized_dir = "uploads/optimized";
        let optimized_path = format!("{}/{}.jpg", optimized_dir, cache_key);

        // Check if optimized version exists
        if !Path::new(&optimized_path).exists() {
            // Create optimized directory if it doesn't exist
            tokio::fs::create_dir_all(optimized_dir)
                .await
                .map_err(|e| {
                    AppError::new(format!("Failed to create optimized directory: {}", e))
                })?;

            // Load and process image using resolved path
            let img = image::open(&resolved_src)
                .map_err(|e| AppError::new(format!("Failed to open image: {}", e)))?;

            let processed = if let (Some(w), Some(h)) = (width, height) {
                img.resize_to_fill(w, h, FilterType::Lanczos3)
            } else if let Some(w) = width {
                img.resize(w, u32::MAX, FilterType::Lanczos3)
            } else if let Some(h) = height {
                img.resize(u32::MAX, h, FilterType::Lanczos3)
            } else {
                img
            };

            // Save as progressive JPEG
            use jpeg_encoder::{Encoder, ColorType};

            let mut encoder = Encoder::new_file(&optimized_path, quality)
                .map_err(|e| AppError::new(format!("Failed to create encoder: {}", e)))?;

            // Enable progressive encoding
            encoder.set_progressive(true);

            // Convert image color type
            let color_type = match processed.color() {
                image::ColorType::Rgb8 => ColorType::Rgb,
                image::ColorType::Rgba8 => ColorType::Rgba,
                image::ColorType::L8 => ColorType::Luma,
                _ => {
                    // Convert to RGB if not supported
                    let rgb_img = processed.to_rgb8();
                    encoder.encode(
                        rgb_img.as_raw(),
                        rgb_img.width() as u16,
                        rgb_img.height() as u16,
                        ColorType::Rgb
                    ).map_err(|e| AppError::new(format!("Failed to encode progressive JPEG: {}", e)))?;
                    return Ok(format!("/{}", optimized_path));
                }
            };

            encoder.encode(
                processed.as_bytes(),
                processed.width() as u16,
                processed.height() as u16,
                color_type
            ).map_err(|e| AppError::new(format!("Failed to encode progressive JPEG: {}", e)))?;
        }

        Ok(format!("/{}", optimized_path))
    }

    /// Resolve URL path to filesystem path
    /// - `/images/foo.jpg` → `{site_root}/images/foo.jpg` (site_root from LEPTOS_SITE_ROOT or "target/site")
    /// - `/uploads/foo.jpg` → `uploads/foo.jpg`
    /// - `uploads/foo.jpg` → `uploads/foo.jpg` (relative paths unchanged)
    fn resolve_image_path(src: &str) -> Result<String, StatusCode> {
        // Security: prevent path traversal
        if src.contains("..") {
            tracing::error!(
                src = %src,
                "Path traversal attempt detected"
            );
            return Err(StatusCode::BAD_REQUEST);
        }

        let site_root = get_site_root();

        let resolved = if src.starts_with('/') {
            // URL path - need to resolve to filesystem
            if src.starts_with("/uploads/") {
                // /uploads/foo.jpg → uploads/foo.jpg
                src.trim_start_matches('/').to_string()
            } else if src.starts_with("/images/")
                || src.starts_with("/logo")
                || src.starts_with("/favicon")
            {
                // /images/foo.jpg → {site_root}/images/foo.jpg
                // /logo.svg → {site_root}/logo.svg
                format!("{}{}", site_root, src)
            } else {
                // Other root paths → {site_root}/...
                format!("{}{}", site_root, src)
            }
        } else {
            // Relative path - use as-is
            src.to_string()
        };

        Ok(resolved)
    }

    #[tracing::instrument(name = "serve_optimized_image", skip_all, fields(src = %params.src, width = ?params.width, height = ?params.height, quality = ?params.quality))]
    pub async fn serve_optimized_image(
        Query(params): Query<ImageParams>,
    ) -> Result<Response, StatusCode> {
        // Resolve URL path to filesystem path
        let resolved_src = resolve_image_path(&params.src)?;

        let quality = params.quality.unwrap_or(80);
        let cache_key = format!(
            "{}_{}x{}_q{}",
            params.src.replace("/", "_").replace(".", "_"),
            params.width.unwrap_or(0),
            params.height.unwrap_or(0),
            quality
        );

        let optimized_dir = "uploads/optimized";
        let optimized_path = format!("{}/{}.jpg", optimized_dir, cache_key);

        // Check if optimized version exists in cache
        if !Path::new(&optimized_path).exists() {
            // Validate source path exists
            if !Path::new(&resolved_src).exists() {
                tracing::error!(
                    src = %params.src,
                    resolved_src = %resolved_src,
                    "Source image not found"
                );
                return Err(StatusCode::NOT_FOUND);
            }

            tracing::debug!(
                src = %params.src,
                optimized_path = %optimized_path,
                "Generating optimized image"
            );

            // Create optimized directory if it doesn't exist
            tokio::fs::create_dir_all(optimized_dir)
                .await
                .map_err(|e| {
                    tracing::error!(
                        error = %e,
                        dir = %optimized_dir,
                        "Failed to create optimized directory"
                    );
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

            // Load and process image using resolved path
            let img = image::open(&resolved_src).map_err(|e| {
                tracing::error!(
                    error = %e,
                    src = %params.src,
                    resolved_src = %resolved_src,
                    "Failed to open source image"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            let processed = if let (Some(w), Some(h)) = (params.width, params.height) {
                img.resize_to_fill(w, h, FilterType::Lanczos3)
            } else if let Some(w) = params.width {
                img.resize(w, u32::MAX, FilterType::Lanczos3)
            } else if let Some(h) = params.height {
                img.resize(u32::MAX, h, FilterType::Lanczos3)
            } else {
                img
            };

            // Save as progressive JPEG
            use jpeg_encoder::{Encoder, ColorType};

            let mut encoder = Encoder::new_file(&optimized_path, quality).map_err(|e| {
                tracing::error!(
                    error = %e,
                    path = %optimized_path,
                    "Failed to create encoder"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            // Enable progressive encoding
            encoder.set_progressive(true);

            // Convert image color type
            let encode_result = match processed.color() {
                image::ColorType::Rgb8 => {
                    encoder.encode(
                        processed.as_bytes(),
                        processed.width() as u16,
                        processed.height() as u16,
                        ColorType::Rgb
                    )
                }
                image::ColorType::Rgba8 => {
                    encoder.encode(
                        processed.as_bytes(),
                        processed.width() as u16,
                        processed.height() as u16,
                        ColorType::Rgba
                    )
                }
                image::ColorType::L8 => {
                    encoder.encode(
                        processed.as_bytes(),
                        processed.width() as u16,
                        processed.height() as u16,
                        ColorType::Luma
                    )
                }
                _ => {
                    // Convert to RGB if not supported
                    let rgb_img = processed.to_rgb8();
                    encoder.encode(
                        rgb_img.as_raw(),
                        rgb_img.width() as u16,
                        rgb_img.height() as u16,
                        ColorType::Rgb
                    )
                }
            };

            encode_result.map_err(|e| {
                tracing::error!(
                    error = %e,
                    path = %optimized_path,
                    "Failed to encode progressive JPEG"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            tracing::info!(
                src = %params.src,
                optimized_path = %optimized_path,
                width = ?params.width,
                height = ?params.height,
                quality = %quality,
                "Successfully generated optimized image"
            );
        } else {
            tracing::debug!(
                optimized_path = %optimized_path,
                "Serving cached optimized image"
            );
        }

        // Read and serve the optimized image
        let image_data = tokio::fs::read(&optimized_path).await.map_err(|e| {
            tracing::error!(
                error = %e,
                path = %optimized_path,
                "Failed to read optimized image"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok((
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, "image/jpeg"),
                (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
            ],
            image_data,
        )
            .into_response())
    }
}

#[server]
pub async fn get_optimized_image_url(
    src: String,
    width: Option<u32>,
    height: Option<u32>,
    quality: Option<u8>,
) -> Result<String, ServerFnError> {
    let result = image_handler::get_optimized_image_url_ssr(src, width, height, quality).await?;
    Ok(result)
}

#[component]
pub fn Image(
    #[prop(into)] src: String,
    #[prop(optional, into)] alt: Option<String>,
    #[prop(optional)] width: Option<u32>,
    #[prop(optional)] height: Option<u32>,
    #[prop(optional)] quality: Option<u8>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] fallback_src: Option<String>,
    #[prop(optional, default = true)] lazy: bool,
) -> impl IntoView {
    let (current_src, set_current_src) = signal(src.clone());
    let (has_error, set_has_error) = signal(false);
    let (is_loading, set_is_loading) = signal(true);

    // Clone fallback_src for use in closures
    let fallback_src_clone = fallback_src.clone();
    let fallback_src_clone2 = fallback_src.clone();

    // Use server function during SSR to pre-generate optimized image
    // This will run on the server and return the optimized path
    let optimized_resource = Resource::new(
        move || (current_src.get(), width, height, quality),
        move |(src_val, w, h, q)| async move {
            // Only optimize if we have optimization params
            if w.is_some() || h.is_some() || q.is_some() {
                // Call server function to generate optimized image
                match get_optimized_image_url(src_val.clone(), w, h, q).await {
                    Ok(optimized_path) => optimized_path,
                    Err(_) => src_val, // Fallback to original on error
                }
            } else {
                src_val
            }
        },
    );

    let final_src = move || {
        if has_error.get() {
            // Use fallback if available
            return fallback_src_clone.clone().unwrap_or_default();
        }

        // Use optimized path from resource if available
        match optimized_resource.get() {
            Some(path) => path,
            None => current_src.get(), // Loading state - use original
        }
    };

    let img_class = move || {
        let base_class = class
            .clone()
            .unwrap_or_else(|| "bg-black w-full h-full object-cover".to_string());
        // if is_loading.get() {
        //     format!("{} opacity-0 transition-opacity duration-300", base_class)
        // } else {
        //     format!("{} opacity-100 transition-opacity duration-300", base_class)
        // }
        base_class
    };

    // let loading_attr = if lazy { "lazy" } else { "eager" };

    view! {
        <Transition fallback=move || {
            view! {
                <div
                    class="bg-neutral-900 animate-pulse"
                    style=format!(
                        "width: {}px; height: {}px;",
                        width.unwrap_or(400),
                        height.unwrap_or(300),
                    )
                ></div>
            }
        }>

            {move || {
                let src = final_src();
                let clas = img_class();
                view! {
                    <img
                        src=src
                        // alt=alt.clone().unwrap_or_else(|| "Image".to_string())
                        class=clas
                        loading="lazy"
                        fetchpriority="high"
                    />
                }
            }}

        </Transition>
    }
}

#[component]
pub fn ImageIPFSold(
    #[prop(into)] src: String,
    #[prop(optional, into)] alt: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] fallback_src: Option<String>,
) -> impl IntoView {
    let (current_gateway_index, set_current_gateway_index) = signal(0);
    let (use_fallback, set_use_fallback) = signal(false);

    // List of fallback gateways
    let gateways = vec![
        "https://ipfs.io/ipfs/",
        "https://gateway.pinata.cloud/ipfs/",
        "https://cloudflare-ipfs.com/ipfs/",
        "https://dweb.link/ipfs/",
    ];

    // Clone for event handlers
    let gateways_len = gateways.len();
    let has_fallback = fallback_src.is_some();
    let fallback_src_clone = fallback_src.clone();

    // Check if this is an IPFS URL
    let is_ipfs = src.starts_with("ipfs://");
    let source_url = if is_ipfs {
        src.strip_prefix("ipfs://").unwrap_or(&src).to_string()
    } else {
        src.clone()
    };

    let file_url = move || {
        if source_url.trim().is_empty() {
            return fallback_src_clone.clone().unwrap_or_default();
        }

        let use_fallback_local = use_fallback.get();

        if use_fallback_local {
            fallback_src_clone.clone().unwrap_or_default()
        } else if is_ipfs {
            let index = current_gateway_index.get();
            if index < gateways.len() {
                format!("{}{}", gateways[index], source_url)
            } else {
                String::new()
            }
        } else {
            source_url.clone()
        }
    };

    view! {
        <img
            src=move || file_url()
            alt=alt.clone().unwrap_or_else(|| "Image".to_string())
            class=class.clone().unwrap_or_else(|| "w-full h-full object-cover".to_string())
            loading="lazy"

            on:error=move |_| {
                if !use_fallback.get() {
                    if is_ipfs {
                        let next_index = current_gateway_index.get() + 1;
                        if next_index < gateways_len {
                            set_current_gateway_index.set(next_index);
                        } else if has_fallback {
                            set_use_fallback.set(true);
                        }
                    } else if has_fallback {
                        set_use_fallback.set(true);
                    }
                }
            }
        />
    }
}
