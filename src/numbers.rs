use std::str::FromStr;

/// Converts between human-readable token amounts and raw token units
///
/// # Arguments
/// * `input` - The input value as a string
/// * `decimals` - The number of decimal places for the token
/// * `to_raw` - If true, converts from human-readable to raw units (e.g., "1.5" -> "1500000")
///              If false, converts from raw units to human-readable (e.g., "1500000" -> "1.5")
///
/// # Examples
/// ```
/// // Human to raw: "1.5" with 6 decimals -> "1500000"
/// format_token_by_decimals("1.5".to_string(), 6, true);
///
/// // Raw to human: "1500000" with 6 decimals -> "1.5"
/// format_token_by_decimals("1500000".to_string(), 6, false);
/// ```
///
///
///

pub struct TokenAmountRaw(u128);
pub struct TokenAmountNice(String);

impl TokenAmountNice {
    pub fn new(amount: String) -> Self {
        TokenAmountNice(amount)
    }

    pub fn to_raw(&self, decimals: u8) -> TokenAmountRaw {
        let output = format_token_by_decimals(self.0.clone(), decimals, true);
        let typed = TokenAmountRaw::new(output.parse().unwrap_or(0));
        typed
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl FromStr for TokenAmountNice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TokenAmountNice(s.to_string()))
    }
}

impl TokenAmountRaw {
    pub fn new(amount: u128) -> Self {
        TokenAmountRaw(amount)
    }

    pub fn to_nice(&self, decimals: u32) -> TokenAmountNice {
        let divisor = 10u128.pow(decimals);
        let result = self.0 as f64 / divisor as f64;

        // Format with appropriate decimal places
        if result.fract() == 0.0 {
            TokenAmountNice(format!("{:.0}", result))
        } else {
            TokenAmountNice(format!("{}", result))
        }
    }
}

impl From<u128> for TokenAmountRaw {
    fn from(amount: u128) -> Self {
        TokenAmountRaw::new(amount)
    }
}

pub fn format_token_by_decimals(input: String, decimals: u8, to_raw: bool) -> String {
    if to_raw {
        // Convert from human-readable to raw units
        let num = match input.parse::<f64>() {
            Ok(n) => n,
            Err(_) => return "0".to_string(),
        };

        let multiplier = 10u128.pow(decimals.into());
        let result = (num * multiplier as f64) as u128;
        result.to_string()
    } else {
        // Convert from raw units to human-readable
        let num = match input.parse::<u128>() {
            Ok(n) => n,
            Err(_) => return "0".to_string(),
        };

        let divisor = 10u128.pow(decimals.into());
        let result = num as f64 / divisor as f64;

        // Format with appropriate decimal places
        if result.fract() == 0.0 {
            format!("{:.0}", result)
        } else {
            format!("{}", result)
        }
    }
}

/// goes from absolute token value to normal numbers
/// e.g. input: "1000000", decimals: 6 -> "1
pub fn format_number_dec(input: String, decimals: u8) -> String {
    // Parse the input string as a large integer
    let num = match input.parse::<u128>() {
        Ok(n) => n,
        Err(_) => return input, // Return original if parsing fails
    };

    // Divide by 10^decimals to get the actual value
    let divisor = 10u128.pow(decimals.into());
    let result = num / divisor;

    // Format with thousands separators
    let formatted = result.to_string();
    let mut chars: Vec<char> = formatted.chars().collect();
    let mut i = chars.len();
    let mut count = 0;

    while i > 0 {
        if count == 3 && i > 0 {
            chars.insert(i, ',');
            count = 0;
        }
        i -= 1;
        count += 1;
    }

    chars.into_iter().collect()
}

#[test]
fn test_format_number() {
    let input: u128 = 1000000000000000000000000;
    assert_eq!(format_number_dec(input.to_string(), 18), "1,000,000");

    // Test human to raw conversion
    let output = format_token_by_decimals("1000000".to_string(), 18, true);
    assert_eq!(input.to_string(), output);

    // Test raw to human conversion
    let human = format_token_by_decimals(input.to_string(), 18, false);
    assert_eq!(human, "1000000");

    // Test decimal conversion
    let raw = format_token_by_decimals("1.5".to_string(), 6, true);
    assert_eq!(raw, "1500000");

    // Test decimal to human
    let human_decimal = format_token_by_decimals("1500000".to_string(), 6, false);
    assert_eq!(human_decimal, "1.5");
}

#[allow(dead_code)]
pub fn format_price_f64(price: f64) -> String {
    if price < 0.0001 {
        format!("{:.12}", price)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    } else if price < 0.01 {
        format!("{:.6}", price)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    } else {
        format!("{:.4}", price)
    }
}

#[allow(dead_code)]
pub fn format_number_alt(num: f64) -> String {
    if num >= 1_000_000_000.0 {
        format!("{:.2}B", num / 1_000_000_000.0)
    } else if num >= 1_000_000.0 {
        format!("{:.2}M", num / 1_000_000.0)
    } else if num >= 1_000.0 {
        format!("{:.2}K", num / 1_000.0)
    } else {
        format!("{:.2}", num)
    }
}

pub fn format_price(price: &str) -> String {
    if let Ok(val) = price.parse::<f64>() {
        if val >= 1.0 {
            format!("{:.0}", val)
        } else if val >= 0.01 {
            format!("{:.4}", val)
        } else if val >= 0.0001 {
            format!("{:.6}", val)
        } else {
            format!("{:.8}", val)
        }
    } else {
        "0.00".to_string()
    }
}

pub fn format_price_usd(price: &str) -> String {
    if let Ok(val) = price.parse::<f64>() {
        if val >= 1.0 {
            format!("${:.2}", val)
        } else if val >= 0.01 {
            format!("${:.4}", val)
        } else if val >= 0.0001 {
            format!("${:.6}", val)
        } else {
            format!("${:.8}", val)
        }
    } else {
        "$0.00".to_string()
    }
}

pub fn format_volume_usd(volume: f64) -> String {
    if volume >= 1_000_000.0 {
        format!("${:.1}M", volume / 1_000_000.0)
    } else if volume >= 1_000.0 {
        format!("${:.1}K", volume / 1_000.0)
    } else {
        format!("${:.0}", volume)
    }
}

pub fn format_market_cap(mcap: f64) -> String {
    if mcap >= 1_000_000.0 {
        format!("${:.1}M", mcap / 1_000_000.0)
    } else if mcap >= 1_000.0 {
        format!("${:.1}K", mcap / 1_000.0)
    } else {
        format!("${:.0}", mcap)
    }
}

pub fn format_liquidity(liquidity: f64) -> String {
    if liquidity >= 1_000_000.0 {
        format!("${:.0}M", liquidity / 1_000_000.0)
    } else if liquidity >= 1_000.0 {
        format!("${:.0}K", liquidity / 1_000.0)
    } else {
        format!("${:.0}", liquidity)
    }
}

pub fn format_percentage(percent: f64) -> String {
    if percent > 0.0 {
        format!("+{:.2}%", percent)
    } else {
        format!("{:.2}%", percent)
    }
}

pub fn price_change_class(percent: f64) -> &'static str {
    if percent > 0.0 {
        "text-green-600 dark:text-green-400"
    } else if percent < 0.0 {
        "text-red-600 dark:text-red-400"
    } else {
        "text-neutral-600 dark:text-neutral-400"
    }
}

pub fn format_number_u64(num: u64) -> String {
    if num >= 1_000_000 {
        format!("{:.1}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.1}K", num as f64 / 1_000.0)
    } else {
        num.to_string()
    }
}
