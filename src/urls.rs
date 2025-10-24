// List of fallback gateways
// let gateways = vec![
//     "https://ipfs.io/ipfs/",
//     "https://gateway.pinata.cloud/ipfs/",
//     "https://cloudflare-ipfs.com/ipfs/",
//     "https://dweb.link/ipfs/",
// ];

use serde::Serialize;

pub fn normalize_url(url: &str) -> String {
    let trimmed = url.trim();
    if trimmed.starts_with("ipfs://") {
        trimmed.replace("ipfs://", "https://ipfs.io/ipfs/")
    } else {
        trimmed.to_string()
    }
}

/// Builds a URL query string from any serializable struct.
/// Fields with `None` values (Option<T>) are automatically omitted.
///
/// # Example
/// ```
/// use serde::Serialize;
/// use tinkr::urls::build_query_string;
///
/// #[derive(Serialize)]
/// struct SearchParams {
///     q: String,
///     cat: Option<String>,
/// }
///
/// let params = SearchParams {
///     q: "search term".to_string(),
///     cat: Some("category".to_string()),
/// };
///
/// let query = build_query_string(&params).unwrap();
/// assert_eq!(query, "?q=search%20term&cat=category");
///
/// // With None value
/// let params2 = SearchParams {
///     q: "test".to_string(),
///     cat: None,
/// };
///
/// let query2 = build_query_string(&params2).unwrap();
/// assert_eq!(query2, "?q=test");
/// ```
pub fn build_query_string<T: Serialize>(params: &T) -> Result<String, serde_json::Error> {
    let value = serde_json::to_value(params)?;

    let mut pairs = Vec::new();

    if let serde_json::Value::Object(map) = value {
        for (key, val) in map {
            match val {
                serde_json::Value::Null => continue, // Skip null values
                serde_json::Value::String(s) => {
                    pairs.push(format!(
                        "{}={}",
                        urlencoding::encode(&key),
                        urlencoding::encode(&s)
                    ));
                }
                serde_json::Value::Number(n) => {
                    pairs.push(format!(
                        "{}={}",
                        urlencoding::encode(&key),
                        urlencoding::encode(&n.to_string())
                    ));
                }
                serde_json::Value::Bool(b) => {
                    pairs.push(format!("{}={}", urlencoding::encode(&key), b));
                }
                serde_json::Value::Array(arr) => {
                    // Handle arrays by repeating the key
                    for item in arr {
                        if let Some(s) = item.as_str() {
                            pairs.push(format!(
                                "{}={}",
                                urlencoding::encode(&key),
                                urlencoding::encode(s)
                            ));
                        } else if let Some(n) = item.as_f64() {
                            pairs.push(format!("{}={}", urlencoding::encode(&key), n));
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    if pairs.is_empty() {
        Ok(String::new())
    } else {
        Ok(format!("?{}", pairs.join("&")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct SearchParams {
        q: String,
        cat: Option<String>,
    }

    #[derive(Serialize)]
    struct ComplexParams {
        name: String,
        age: u32,
        active: bool,
        tags: Vec<String>,
        optional: Option<String>,
    }

    #[test]
    fn test_basic_query_string() {
        let params = SearchParams {
            q: "search term".to_string(),
            cat: Some("category".to_string()),
        };

        let query = build_query_string(&params).unwrap();
        assert_eq!(query, "?q=search%20term&cat=category");
    }

    #[test]
    fn test_query_string_with_none() {
        let params = SearchParams {
            q: "test".to_string(),
            cat: None,
        };

        let query = build_query_string(&params).unwrap();
        assert_eq!(query, "?q=test");
    }

    #[test]
    fn test_empty_query_string() {
        let params = SearchParams {
            q: "".to_string(),
            cat: None,
        };

        let query = build_query_string(&params).unwrap();
        // Empty string value is still included
        assert_eq!(query, "?q=");
    }

    #[test]
    fn test_complex_params() {
        let params = ComplexParams {
            name: "John Doe".to_string(),
            age: 30,
            active: true,
            tags: vec!["rust".to_string(), "web".to_string()],
            optional: None,
        };

        let query = build_query_string(&params).unwrap();
        // Note: order might vary due to HashMap in serde_json
        assert!(query.contains("name=John%20Doe"));
        assert!(query.contains("age=30"));
        assert!(query.contains("active=true"));
        assert!(query.contains("tags=rust"));
        assert!(query.contains("tags=web"));
        assert!(!query.contains("optional"));
    }

    #[test]
    fn test_special_characters() {
        let params = SearchParams {
            q: "hello & goodbye".to_string(),
            cat: Some("category/test".to_string()),
        };

        let query = build_query_string(&params).unwrap();
        assert!(query.contains("hello%20%26%20goodbye"));
        assert!(query.contains("category%2Ftest"));
    }

    #[test]
    fn test_normalize_url_ipfs() {
        let url = "ipfs://QmHash123";
        assert_eq!(normalize_url(url), "https://ipfs.io/ipfs/QmHash123");
    }

    #[test]
    fn test_normalize_url_regular() {
        let url = "https://example.com";
        assert_eq!(normalize_url(url), "https://example.com");
    }
}
