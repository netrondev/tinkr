use crate::RecordId;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Partial)]
#[partial(
    "LinkAccountData",
    derive(Debug, Clone, Serialize, Deserialize),
    omit(id)
)]
pub struct AdapterAccount {
    pub id: RecordId,
    pub access_token: String,
    pub account_type: AccountType,
    pub expires_at: i64,
    pub provider_account_id: String,
    pub provider: String,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub token_type: String,
    pub user_id: RecordId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    OAuth,
    Email,
    Credentials,
}
