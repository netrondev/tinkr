/// Fixes the capitalization of Ethereum addresses to conform to the checksum standard.
/// EIP-55.
#[cfg(feature = "ssr")]
pub fn fix_capitalization(address: &str) -> String {
    // Remove '0x' prefix if present and convert to lowercase
    let address = address.trim_start_matches("0x").to_lowercase();

    // Compute Keccak-256 hash of the lowercase address
    let mut hasher = alloy::primitives::Keccak256::new();
    hasher.update(address.as_bytes());
    let hash = hasher.finalize();

    // Build the checksummed address
    let mut checksummed = String::with_capacity(42);
    checksummed.push_str("0x");

    for (i, ch) in address.chars().enumerate() {
        if ch.is_alphabetic() {
            // Check if the corresponding hex digit in hash is >= 8
            let hash_byte = hash[i / 2];
            let hash_nibble = if i % 2 == 0 {
                hash_byte >> 4
            } else {
                hash_byte & 0x0f
            };

            if hash_nibble >= 8 {
                checksummed.push(ch.to_ascii_uppercase());
            } else {
                checksummed.push(ch);
            }
        } else {
            checksummed.push(ch);
        }
    }

    checksummed
}
