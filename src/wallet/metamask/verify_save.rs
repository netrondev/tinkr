use crate::AppError;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use leptos::server_fn::ServerFnError;

#[cfg(feature = "ssr")]
use tracing::info;

#[cfg(feature = "ssr")]
use crate::metamask::checksum::fix_capitalization;

#[cfg(feature = "ssr")]
use crate::user::{AdapterUser, CreateUserData};
#[cfg(feature = "ssr")]
use crate::wallet::evm::chains::Chain;

#[cfg(feature = "ssr")]
use crate::wallet::wallet::{CreateWallet, Wallet};

#[cfg(feature = "ssr")]
use crate::EmailAddress;
#[cfg(feature = "ssr")]
use crate::theme::Theme;
#[cfg(feature = "ssr")]
use alloy::primitives::Address;
#[cfg(feature = "ssr")]
use alloy::signers::Signature;
#[cfg(feature = "ssr")]
use chrono::Utc;

#[cfg(feature = "ssr")]
use surrealdb::Datetime;

#[server]
pub async fn verify_and_save_wallet(
    address: String,
    chain_id: Option<String>,
    message: String,
    signature: String,
) -> Result<(), AppError> {
    info!("Verifying wallet for address: {}", address);

    // Verify the signature
    let recovered_address = {
        // Parse the signature (remove 0x prefix if present)
        let sig_str = signature.trim_start_matches("0x");
        let sig_bytes = hex::decode(sig_str)?;
        // .map_err(|e| ServerFnError::new(format!("Invalid signature format: {}", e)))?;

        if sig_bytes.len() != 65 {
            return Err(AppError::new(
                "Invalid signature length, expected 65 bytes".to_string(),
            ));
        }

        // Create signature from bytes
        let signature = Signature::try_from(sig_bytes.as_slice())
            .map_err(|e| ServerFnError::new(format!("Failed to parse signature: {}", e)))?;

        // Recover address from message using the signature's built-in method
        signature
            .recover_address_from_msg(&message)
            .map_err(|e| ServerFnError::new(format!("Failed to recover address: {}", e)))?
    };

    // Verify recovered address matches the provided address
    let fixed_capi = fix_capitalization(&address);
    let provided_address = Address::parse_checksummed(fixed_capi, None)
        .map_err(|e| AppError::new(format!("Checksum failed: {}", e)))?;

    if recovered_address != provided_address {
        return Err(AppError::new(
            "Signature verification failed: recovered address does not match provided address"
                .to_string(),
        ));
    }

    // at this point we know the signature is valid and the address matches
    // check if the user is currently logged in (could by by email)

    let user_is_logged_in_already = crate::session::get_user().await;

    match user_is_logged_in_already {
        Ok(user) => {
            info!("User is logged in: {}", user.email);
            // User is logged in, proceed to save wallet

            let user = crate::session::get_user().await?;
            let now = Utc::now();

            // Check if wallet already exists
            let existing = Wallet::get_by_user(user.id.clone()).await?;
            let wallet_exists = existing
                .iter()
                .any(|w| w.address.to_lowercase() == address.to_lowercase());

            if !wallet_exists {
                let chain_name = if let Some(id) = chain_id.as_ref() {
                    // Parse hex chain ID to u64
                    if let Ok(chain_id_num) = u64::from_str_radix(id.trim_start_matches("0x"), 16) {
                        if let Ok(chain) = Chain::get_by_chain_id(chain_id_num).await {
                            chain.name
                        } else {
                            "EVM Chain".to_string()
                        }
                    } else {
                        "EVM Chain".to_string()
                    }
                } else {
                    "EVM Chain".to_string()
                };

                let _new_wallet = Wallet::new(CreateWallet {
                    address: address.to_lowercase(),
                    label: format!("MetaMask - {}", chain_name),
                    wallet_type: "metamask".to_string(),
                    chain_type: Some("evm".to_string()),
                    chain_id,
                    created_by_user_id: user.id,
                    created_at: Datetime::from(now),
                    updated_at: Datetime::from(now),
                    is_primary: existing.is_empty(), // Set as primary if first wallet
                })
                .await?;
            }
        }
        Err(e) => {
            info!("User not logged in, attempting to create wallet: {}", e);
            // lets search for the wallet in the db.
            let existing_wallet = Wallet::get_by_address(address.clone()).await?;
            let now = Utc::now();
            let user = match existing_wallet {
                Some(wallet) => wallet.get_user().await?,
                None => {
                    let newuser = AdapterUser::create_user(CreateUserData {
                        email: EmailAddress::create_blank(),
                        email_verified: None,
                        image: None,
                        name: address.to_string(),
                        theme: Theme::System,
                        address1: None,
                        address2: None,
                        address3: None,
                        postcode: None,
                        phone: None,
                        telephone: None,
                        first_name: None,
                        last_name: None,
                    })
                    .await?;

                    let _new_wallet = Wallet::new(CreateWallet {
                        address: address.to_lowercase(),
                        label: "Primary auth wallet".to_string(),
                        wallet_type: "metamask".to_string(),
                        chain_type: Some("evm".to_string()),
                        chain_id,
                        created_by_user_id: newuser.id.clone(),
                        created_at: Datetime::from(now),
                        updated_at: Datetime::from(now),
                        is_primary: true,
                    })
                    .await?;

                    newuser
                }
            };

            let session = user.new_session().await?;
            let cookie = session.build_session_cookie();

            // logs in the user by setting the session cookie
            use http::header::HeaderValue;
            use leptos_axum::ResponseOptions;

            if let Some(resp) = use_context::<ResponseOptions>() {
                resp.insert_header(
                    axum::http::header::SET_COOKIE,
                    HeaderValue::from_str(&cookie.to_string()).unwrap(),
                );
            }

            // If the wallet exists, we can link it to a new user or handle it accordingly
            // For now, we will just return an error
            return Ok(());
        }
    }

    Ok(())
}

#[server]
pub async fn verify_wallet_belongs_to_user(wallet_address: String) -> Result<bool, ServerFnError> {
    // Get the current logged-in user
    let user = match crate::session::get_user().await {
        Ok(user) => user,
        Err(_) => {
            // No user logged in, wallet should not be shown as connected
            return Ok(false);
        }
    };

    // Get all wallets for this user
    let user_wallets = Wallet::get_by_user(user.id).await?;

    // Check if any of the user's wallets match the connected address
    let wallet_belongs_to_user = user_wallets
        .iter()
        .any(|w| w.address.to_lowercase() == wallet_address.to_lowercase());

    Ok(wallet_belongs_to_user)
}
