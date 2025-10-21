// List of fallback gateways
// let gateways = vec![
//     "https://ipfs.io/ipfs/",
//     "https://gateway.pinata.cloud/ipfs/",
//     "https://cloudflare-ipfs.com/ipfs/",
//     "https://dweb.link/ipfs/",
// ];

pub fn normalize_url(url: &str) -> String {
    let trimmed = url.trim();
    if trimmed.starts_with("ipfs://") {
        trimmed.replace("ipfs://", "https://ipfs.io/ipfs/")
    } else {
        trimmed.to_string()
    }
}
