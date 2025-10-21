use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

use crate::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Id {
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Thing {
    /// Table name
    pub tb: String,
    pub id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct RecordId(Thing);

impl RecordId {
    pub fn from_table_key(table: &str, key: &str) -> Self {
        Self(Thing {
            tb: table.to_string(),
            id: Id::String(key.to_string()),
        })
    }

    pub fn table(&self) -> &str {
        self.0.tb.as_str()
    }

    pub fn key(&self) -> &str {
        match &self.0.id {
            Id::String(s) => s.as_str(),
        }
    }

    pub fn to_raw_string(&self) -> String {
        self.key().to_string()
    }

    pub fn from_table_and_id(table: &str, id: String) -> Self {
        Self::from_table_key(table, &id)
    }
}

impl From<String> for RecordId {
    fn from(record_id: String) -> Self {
        let parts: Vec<&str> = record_id.split(':').collect();
        if parts.len() != 2 {
            return Self::from_table_key("unknown", "unknown");
        }
        Self::from_table_key(parts[0], parts[1])
    }
}

impl FromStr for RecordId {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(AppError::GenericError("Invalid RecordId format".into()));
        }
        Ok(Self::from_table_key(parts[0], parts[1]))
    }
}

impl fmt::Display for RecordId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.0.tb, self.key())
    }
}

#[cfg(feature = "ssr")]
impl Into<surrealdb::RecordId> for RecordId {
    fn into(self) -> surrealdb::RecordId {
        let asd: surrealdb::RecordId =
            serde_json::from_str(&serde_json::to_string(&self).unwrap()).unwrap();
        asd
    }
}

#[cfg(feature = "ssr")]
impl From<surrealdb::RecordId> for RecordId {
    fn from(record_id: surrealdb::RecordId) -> Self {
        let asd: Self = serde_json::from_str(&serde_json::to_string(&record_id).unwrap()).unwrap();
        asd
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Datetime(pub DateTime<Utc>);

impl From<DateTime<Utc>> for Datetime {
    fn from(v: DateTime<Utc>) -> Self {
        Self(v.into())
    }
}

impl Datetime {
    pub fn format(&self, fmt: &str) -> String {
        self.0.format(fmt).to_string()
    }

    pub fn inner(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl fmt::Display for Datetime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_rfc3339())
    }
}

#[cfg(feature = "ssr")]
#[test]
fn test_serialization() -> Result<(), AppError> {
    use crate::surrealtypes::RecordId as FrontendRecordId;
    use surrealdb::RecordId as BackendRecordId;

    // 1. Create a backend RecordId (what SurrealDB returns)
    let be_r = BackendRecordId::from_table_key("meter", "12345");

    let be_s = serde_json::to_string(&be_r).unwrap();
    let fe_r: FrontendRecordId = serde_json::from_str(&be_s).unwrap();

    let fe_s = serde_json::to_string(&fe_r).unwrap();
    assert_eq!(be_s, fe_s);

    let be_r2: BackendRecordId = serde_json::from_str(&fe_s).unwrap();

    println!("Backend RecordId: {:?}", be_r);
    println!("Backend RecordId: {:?}", be_r2);

    Ok(())
}
