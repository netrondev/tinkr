/// Hash a string to a 32-bit integer (matches JavaScript's hashCode behavior)
pub fn hash_code(name: &str) -> u32 {
    let mut hash: i32 = 0;
    for ch in name.chars() {
        let character = ch as i32;
        hash = ((hash << 5).wrapping_sub(hash)).wrapping_add(character);
        hash = hash & hash; // Convert to 32bit integer
    }
    hash.abs() as u32
}

/// Simple modulus operation (num % max)
pub fn get_modulus(num: u32, max: u32) -> u32 {
    num % max
}

/// Extract digit at position from number
pub fn get_digit(number: u32, ntn: u32) -> u32 {
    ((number / 10_u32.pow(ntn)) % 10)
}

/// Check if digit at position is even
pub fn get_boolean(number: u32, ntn: u32) -> bool {
    (get_digit(number, ntn) % 2) == 0
}

/// Calculate angle from x,y coordinates (returns degrees)
pub fn get_angle(x: f64, y: f64) -> f64 {
    y.atan2(x) * 180.0 / std::f64::consts::PI
}

/// Get signed unit value (can be negative based on digit parity)
pub fn get_unit(number: u32, range: i32, index: Option<u32>) -> i32 {
    let value = (number as i32) % range;
    if let Some(idx) = index {
        if (get_digit(number, idx) % 2) == 0 {
            return -value;
        }
    }
    value
}

/// Get color from palette based on modulus
pub fn get_random_color(number: u32, colors: &[String], range: usize) -> String {
    colors[(number as usize) % range].clone()
}

/// Get contrasting color (black or white) for readability
pub fn get_contrast(hexcolor: &str) -> String {
    let hex = hexcolor.trim_start_matches('#');

    // Convert hex to RGB
    let r = u32::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u32::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u32::from_str_radix(&hex[4..6], 16).unwrap_or(0);

    // Calculate YIQ luminance
    let yiq = ((r * 299) + (g * 587) + (b * 114)) / 1000;

    // Return appropriate contrast color
    if yiq >= 128 {
        "#000000".to_string()
    } else {
        "#FFFFFF".to_string()
    }
}
