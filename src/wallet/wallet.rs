use partial_struct::Partial;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::db_init;

#[cfg(feature = "ssr")]
use crate::AppError;

#[cfg(not(feature = "ssr"))]
use crate::{Datetime, RecordId};

#[cfg(feature = "ssr")]
use surrealdb::{Datetime, RecordId};

#[cfg(feature = "ssr")]
use crate::user::AdapterUser;

#[cfg(feature = "ssr")]
use crate::keys::{Key, KeyCreate};

#[cfg(feature = "ssr")]
use crate::StorageAuthed;

#[derive(Debug, Clone, Serialize, Deserialize, Partial)]
#[partial("CreateWallet", derive(Serialize, Deserialize, Clone), omit(id))]
#[partial(
    "UpdateWallet",
    derive(Debug, Serialize, Deserialize, Clone),
    omit(created_by_user_id)
)]
pub struct Wallet {
    pub id: RecordId,
    pub address: String,
    pub label: String,
    pub wallet_type: String, // "phantom", "solflare", "backpack", "metamask", etc.
    pub chain_type: Option<String>, // "solana", "evm"
    pub chain_id: Option<String>, // For EVM: "0x1" (mainnet), "0x89" (polygon), etc.
    pub created_by_user_id: RecordId,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub is_primary: bool,
}

impl Default for Wallet {
    fn default() -> Self {
        Wallet {
            id: RecordId::from_table_key("wallet", "default"),
            address: "".to_string(),
            label: "My Wallet".to_string(),
            wallet_type: "phantom".to_string(),
            chain_type: Some("solana".to_string()),
            chain_id: None,
            created_by_user_id: RecordId::from_table_key("user", "default"),
            created_at: Datetime::from(chrono::Utc::now()),
            updated_at: Datetime::from(chrono::Utc::now()),
            is_primary: false,
        }
    }
}

#[cfg(feature = "ssr")]
impl Wallet {
    pub async fn new(data: CreateWallet) -> Result<Self, AppError> {
        let client = db_init().await?;
        let create_result: Option<Self> = client.create("wallet").content(data).await?;
        let created: Self =
            create_result.ok_or_else(|| AppError::AuthError("Could not create wallet".into()))?;
        Ok(created)
    }

    pub async fn get_by_address(address: String) -> Result<Option<Self>, AppError> {
        let client = db_init().await?;
        let mut result = client
            .query("SELECT * FROM wallet WHERE string::lowercase(address) = string::lowercase($address) ORDER BY is_primary DESC, label;")
            .bind(("address", address))
            .await?;
        let wallets: Vec<Self> = result.take(0)?;

        let first = wallets.first().cloned();

        Ok(first)
    }

    pub async fn get_user(&self) -> Result<AdapterUser, AppError> {
        let user = AdapterUser::get_user(self.created_by_user_id.clone()).await?;
        Ok(user)
    }

    pub async fn get_by_user(user_id: RecordId) -> Result<Vec<Self>, AppError> {
        let client = db_init().await?;
        let mut result = client
            .query("SELECT * FROM wallet WHERE created_by_user_id = $user_id ORDER BY is_primary DESC, label;")
            .bind(("user_id", user_id))
            .await?;
        let wallets: Vec<Self> = result.take(0)?;
        Ok(wallets)
    }

    pub async fn get_by_id_and_user(
        wallet_id: RecordId,
        user_id: RecordId,
    ) -> Result<Self, AppError> {
        let client = db_init().await?;
        let mut result = client
            .query("SELECT * FROM wallet WHERE id = $wallet_id AND created_by_user_id = $user_id LIMIT 1;")
            .bind(("wallet_id", wallet_id))
            .bind(("user_id", user_id))
            .await?;

        let wallet: Option<Self> = result.take(0)?;
        wallet.ok_or_else(|| AppError::AuthError("Wallet not found".into()))
    }

    pub async fn update_by_id_and_user(
        id: RecordId,
        user_id: RecordId,
        label: String,
        is_primary: bool,
    ) -> Result<Self, AppError> {
        let client = db_init().await?;

        // If setting as primary, unset other wallets first
        if is_primary {
            client
                .query("UPDATE wallet SET is_primary = false WHERE created_by_user_id = $user_id;")
                .bind(("user_id", user_id.clone()))
                .await?;
        }

        let mut result = client
            .query("UPDATE wallet SET label = $label, is_primary = $is_primary, updated_at = $updated_at WHERE id = $id AND created_by_user_id = $user_id RETURN AFTER;")
            .bind(("id", id))
            .bind(("user_id", user_id))
            .bind(("label", label))
            .bind(("is_primary", is_primary))
            .bind(("updated_at", Datetime::from(chrono::Utc::now())))
            .await?;
        let wallet: Option<Self> = result.take(0)?;
        wallet.ok_or_else(|| AppError::AuthError("Wallet not found or update failed".into()))
    }

    pub async fn delete_by_id_and_user(id: RecordId, user_id: RecordId) -> Result<(), AppError> {
        let client = db_init().await?;
        client
            .query("DELETE wallet WHERE id = $id AND created_by_user_id = $user_id;")
            .bind(("id", id))
            .bind(("user_id", user_id))
            .await?;
        Ok(())
    }

    /// Generate a new EVM wallet with a private key
    pub async fn generate_evm(
        user_id: RecordId,
        label: String,
        chain_id: Option<String>,
        is_primary: bool,
    ) -> Result<Self, AppError> {
        use alloy::signers::local::PrivateKeySigner;
        use chrono::Utc;

        // Generate a new random wallet
        let signer = PrivateKeySigner::random();

        // Get the address (with 0x prefix)
        let address = format!("{:?}", signer.address());

        // Get the private key as bytes and convert to hex string
        let private_key_bytes = signer.to_bytes();
        let private_key_hex = hex::encode(private_key_bytes);

        // Create the wallet record in the database
        let wallet_data = CreateWallet {
            address: address.clone(),
            label: label.clone(),
            wallet_type: "generated".to_string(),
            chain_type: Some("evm".to_string()),
            chain_id: chain_id.clone(),
            created_by_user_id: user_id.clone(),
            created_at: Datetime::from(Utc::now()),
            updated_at: Datetime::from(Utc::now()),
            is_primary,
        };

        let wallet = Self::new(wallet_data).await?;

        // Store the private key securely in the Key storage
        let key_data = KeyCreate {
            name: format!("EVM Wallet: {}", label),
            key_for: Some(wallet.id.clone()),
            key_public: Some(address),
            key_private: Some(private_key_hex),
            key_apikey: None,
            key_token: None,
            description: format!("Private key for EVM wallet: {}", label),
            expires_at: None,
        };

        // Get the user to create the key
        let user = AdapterUser::get_user(user_id).await?;
        Key::create_by_user(user, key_data).await?;

        Ok(wallet)
    }
}
