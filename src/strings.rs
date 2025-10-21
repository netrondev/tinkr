pub fn address_to_short(addr: &str) -> String {
    if addr.len() <= 10 {
        return addr.to_string();
    }
    format!("{}...{}", &addr[..6], &addr[addr.len() - 4..])
}

#[test]
fn test_address_to_short() {
    assert_eq!(
        address_to_short("0x1234567890abcdef1234567890abcdef12345678"),
        "0x1234...5678"
    );
    assert_eq!(address_to_short("0x12345"), "0x12345");
    assert_eq!(address_to_short("0x1234"), "0x1234");
    assert_eq!(address_to_short("0x1"), "0x1");
}

pub fn capitalize_first(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    s[0..1].to_uppercase() + &s[1..].to_lowercase()
}

#[test]
fn test_capitalize_first() {
    let test = "etHerEum";

    assert_eq!(capitalize_first(test), "Ethereum");
}
