use tower_http::CompressionLevel;
use tower_http::compression::CompressionLayer;

/// Creates a pre-configured compression layer for HTTP responses
///
/// Features:
/// - Supports Brotli, Deflate, Gzip, and Zstd compression
/// - Uses default compression quality
pub fn create_compression_layer() -> CompressionLayer {
    CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true)
        .quality(CompressionLevel::Default)
}
