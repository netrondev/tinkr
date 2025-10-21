use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EmailAddress(pub String);

impl EmailAddress {
    pub fn create_blank() -> Self {
        EmailAddress("".to_string())
    }

    // Email validation

    pub fn validate_email(&self) -> bool {
        #[cfg(feature = "ssr")]
        {
            let email_regex = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
            return email_regex.is_match(&self.0);
        }

        #[cfg(not(feature = "ssr"))]
        {
            return self.0.contains('@');
        }
    }

    pub fn create_test_email() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let timestamp_nanos = now.as_nanos();

        let random_email = format!("test_{}@test.com", timestamp_nanos.to_string());

        EmailAddress(random_email)
    }

    pub fn is_empty(&self) -> bool {
        let email = &self.0.clone();
        email.is_empty()
    }
}

impl FromStr for EmailAddress {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || !s.contains('@') {
            return Err("Invalid email address".to_string());
        }
        Ok(EmailAddress(s.to_string()))
    }
}

impl std::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for EmailAddress {
    fn from(s: &str) -> Self {
        // Your validation logic here
        EmailAddress::from_str(s).expect("Invalid email address")
        // or handle errors however you prefer
    }
}

impl Default for EmailAddress {
    fn default() -> Self {
        EmailAddress("".to_string())
    }
}
