use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bytes(pub u64);

impl Bytes {
    pub fn to_kb(&self) -> f64 {
        self.0 as f64 / 1024.0
    }

    pub fn to_mb(&self) -> f64 {
        self.0 as f64 / 1024.0 / 1024.0
    }

    pub fn to_gb(&self) -> f64 {
        self.0 as f64 / 1024.0 / 1024.0 / 1024.0
    }

    pub fn from_option_i64(size: Option<i64>) -> Self {
        Bytes(size.unwrap_or(0) as u64)
    }
}

impl std::fmt::Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 < 1024 {
            write!(f, "{} B", self.0)
        } else if self.0 < 1024 * 1024 {
            write!(f, "{:.2} KB", self.to_kb())
        } else if self.0 < 1024 * 1024 * 1024 {
            write!(f, "{:.2} MB", self.to_mb())
        } else {
            write!(f, "{:.2} GB", self.to_gb())
        }
    }
}
