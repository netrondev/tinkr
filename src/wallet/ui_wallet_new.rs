use leptos::prelude::*;

#[cfg(not(feature = "ssr"))]
use crate::metamask::WalletConnectButton;

#[component]
pub fn NewWalletForm() -> impl IntoView {
    view! {
        <div>
            <NewWalletFormContent />
        </div>
    }
}

#[cfg(not(feature = "ssr"))]
fn metamask_section() -> impl IntoView {
    view! { <WalletConnectButton /> }
}

#[cfg(feature = "ssr")]
fn metamask_section() -> impl IntoView {
    view! {
        <p class="text-sm text-gray-500">"MetaMask connection is only available in the browser"</p>
    }
}

#[component]
fn NewWalletFormContent() -> impl IntoView {
    // Wallet mode: "generate" for new wallet, "import" for existing wallet, "connect" for external wallet
    let (wallet_mode, set_wallet_mode) = signal("connect".to_string());
    let (address, set_address) = signal(String::new());
    let (label, set_label) = signal(String::new());
    let (wallet_type, set_wallet_type) = signal("phantom".to_string());
    let (private_key, set_private_key) = signal(String::new());
    let (chain_id, set_chain_id) = signal("0x1".to_string()); // Default to Ethereum mainnet
    let (is_primary, set_is_primary) = signal(false);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);
    let (success_message, set_success_message) = signal(Option::<String>::None);

    let submit_action = Action::new(move |_| {
        let mode = wallet_mode.get();
        let address_val = address.get();
        let label_val = label.get();
        let wallet_type_val = wallet_type.get();
        let private_key_val = private_key.get();
        let chain_id_val = chain_id.get();
        let is_primary_val = is_primary.get();

        async move {
            set_loading.set(true);
            set_error.set(None);
            set_success_message.set(None);

            let result = match mode.as_str() {
                "generate" => {
                    generate_new_evm_wallet(label_val, chain_id_val, is_primary_val).await
                }
                "import" => {
                    import_evm_wallet_with_private_key(
                        label_val,
                        private_key_val,
                        chain_id_val,
                        is_primary_val,
                    )
                    .await
                }
                "connect" => {
                    create_wallet_with_data(address_val, label_val, wallet_type_val, is_primary_val)
                        .await
                }
                _ => Err(leptos::server_fn::ServerFnError::new("Invalid wallet mode")),
            };

            match result {
                Ok(_) => {
                    set_success_message.set(Some(
                        "Wallet created successfully! Redirecting...".to_string(),
                    ));
                    // Redirect immediately
                    window().location().set_href("/wallets").unwrap();
                }
                Err(e) => {
                    set_error.set(Some(e.to_string()));
                }
            }

            set_loading.set(false);
        }
    });

    view! {
        <div class="bg-gray-50 dark:bg-neutral-900">
            <div class="max-w-2xl mx-auto py-8 px-4">
                <div class="bg-white dark:bg-neutral-800 rounded-lg shadow p-6">
                    <div class="flex items-center justify-between mb-6">
                        <h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">
                            "Add Wallet"
                        </h1>
                        <button
                            class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
                            on:click=move |_| {
                                window().location().set_href("/wallets").unwrap();
                            }
                        >
                            "← Back to Wallets"
                        </button>
                    </div>

                    // Wallet mode selector
                    <div class="mb-6">
                        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                            "Choose wallet option:"
                        </label>
                        <div class="grid grid-cols-3 gap-3">
                            <button
                                type="button"
                                class=move || {
                                    format!(
                                        "p-3 rounded-lg border-2 {}",
                                        if wallet_mode.get() == "generate" {
                                            "border-purple-500 bg-purple-50 dark:bg-purple-900/20"
                                        } else {
                                            "border-gray-300 dark:border-gray-600 hover:border-purple-300"
                                        },
                                    )
                                }
                                on:click=move |_| set_wallet_mode.set("generate".to_string())
                            >
                                <div class="text-sm font-medium">"Generate New"</div>
                                <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                                    "Create a new EVM wallet"
                                </div>
                            </button>
                            <button
                                type="button"
                                class=move || {
                                    format!(
                                        "p-3 rounded-lg border-2 {}",
                                        if wallet_mode.get() == "import" {
                                            "border-purple-500 bg-purple-50 dark:bg-purple-900/20"
                                        } else {
                                            "border-gray-300 dark:border-gray-600 hover:border-purple-300"
                                        },
                                    )
                                }
                                on:click=move |_| set_wallet_mode.set("import".to_string())
                            >
                                <div class="text-sm font-medium">"Import Wallet"</div>
                                <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                                    "Use private key"
                                </div>
                            </button>
                            <button
                                type="button"
                                class=move || {
                                    format!(
                                        "p-3 rounded-lg border-2 {}",
                                        if wallet_mode.get() == "connect" {
                                            "border-purple-500 bg-purple-50 dark:bg-purple-900/20"
                                        } else {
                                            "border-gray-300 dark:border-gray-600 hover:border-purple-300"
                                        },
                                    )
                                }
                                on:click=move |_| set_wallet_mode.set("connect".to_string())
                            >
                                <div class="text-sm font-medium">"Connect Existing"</div>
                                <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                                    "External wallet"
                                </div>
                            </button>
                        </div>
                    </div>

                    <form on:submit=move |e| {
                        e.prevent_default();
                        submit_action.dispatch(());
                    }>
                        <div class="space-y-4">

                            <div>
                                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                    "Label"
                                </label>
                                <input
                                    type="text"
                                    required
                                    class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500 bg-white dark:bg-neutral-700 text-gray-900 dark:text-gray-100"
                                    placeholder="e.g., My Main Wallet"
                                    prop:value=move || label.get()
                                    on:input=move |e| set_label.set(event_target_value(&e))
                                />
                            </div>

                            // Show different fields based on wallet mode
                            {move || match wallet_mode.get().as_str() {
                                "generate" => {
                                    view! {
                                        <>
                                            <div class="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-md p-4">
                                                <p class="text-sm text-green-700 dark:text-green-300">
                                                    "A new EVM wallet will be generated with a secure private key. The private key will be stored securely in your account."
                                                </p>
                                            </div>

                                            <div>
                                                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                                    "Chain"
                                                </label>
                                                <select
                                                    class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500 bg-white dark:bg-neutral-700 text-gray-900 dark:text-gray-100"
                                                    prop:value=move || chain_id.get()
                                                    on:change=move |e| set_chain_id.set(event_target_value(&e))
                                                >
                                                    <option value="0x1">"Ethereum Mainnet"</option>
                                                    <option value="0x89">"Polygon"</option>
                                                    <option value="0x38">"BSC"</option>
                                                    <option value="0xa4b1">"Arbitrum"</option>
                                                    <option value="0xa">"Optimism"</option>
                                                    <option value="0x2105">"Base"</option>
                                                </select>
                                            </div>
                                        </>
                                    }
                                        .into_any()
                                }
                                "import" => {
                                    view! {
                                        <>
                                            <div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-md p-4">
                                                <p class="text-sm text-yellow-700 dark:text-yellow-300">
                                                    "⚠️ Warning: Only import private keys from wallets you control. Your private key will be encrypted and stored securely."
                                                </p>
                                            </div>

                                            <div>
                                                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                                    "Private Key"
                                                </label>
                                                <input
                                                    type="password"
                                                    required
                                                    class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500 bg-white dark:bg-neutral-700 text-gray-900 dark:text-gray-100 font-mono"
                                                    placeholder="Enter your private key (with or without 0x prefix)"
                                                    prop:value=move || private_key.get()
                                                    on:input=move |e| {
                                                        set_private_key.set(event_target_value(&e))
                                                    }
                                                />
                                            </div>

                                            <div>
                                                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                                    "Chain"
                                                </label>
                                                <select
                                                    class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500 bg-white dark:bg-neutral-700 text-gray-900 dark:text-gray-100"
                                                    prop:value=move || chain_id.get()
                                                    on:change=move |e| set_chain_id.set(event_target_value(&e))
                                                >
                                                    <option value="0x1">"Ethereum Mainnet"</option>
                                                    <option value="0x89">"Polygon"</option>
                                                    <option value="0x38">"BSC"</option>
                                                    <option value="0xa4b1">"Arbitrum"</option>
                                                    <option value="0xa">"Optimism"</option>
                                                    <option value="0x2105">"Base"</option>
                                                </select>
                                            </div>
                                        </>
                                    }
                                        .into_any()
                                }
                                _ => {
                                    view! {
                                        <>
                                            <div>
                                                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                                    "Wallet Address"
                                                </label>
                                                <input
                                                    type="text"
                                                    required
                                                    class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500 bg-white dark:bg-neutral-700 text-gray-900 dark:text-gray-100"
                                                    placeholder="Enter wallet address (Solana or Ethereum)"
                                                    prop:value=move || address.get()
                                                    on:input=move |e| set_address.set(event_target_value(&e))
                                                />
                                            </div>

                                            <div>
                                                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                                    "Wallet Type"
                                                </label>
                                                <select
                                                    class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500 bg-white dark:bg-neutral-700 text-gray-900 dark:text-gray-100"
                                                    prop:value=move || wallet_type.get()
                                                    on:change=move |e| {
                                                        set_wallet_type.set(event_target_value(&e))
                                                    }
                                                >
                                                    <option value="phantom">"Phantom"</option>
                                                    <option value="solflare">"Solflare"</option>
                                                    <option value="backpack">"Backpack"</option>
                                                    <option value="metamask">"MetaMask"</option>
                                                    <option value="other">"Other"</option>
                                                </select>
                                            </div>
                                        </>
                                    }
                                        .into_any()
                                }
                            }}

                            <div class="flex items-center">
                                <input
                                    type="checkbox"
                                    id="is_primary"
                                    class="h-4 w-4 text-purple-600 focus:ring-purple-500 border-gray-300 rounded"
                                    prop:checked=move || is_primary.get()
                                    on:change=move |e| set_is_primary.set(event_target_checked(&e))
                                />
                                <label
                                    for="is_primary"
                                    class="ml-2 block text-sm text-gray-700 dark:text-gray-300"
                                >
                                    "Set as primary wallet"
                                </label>
                            </div>

                            // Show MetaMask connection only for connect mode
                            {move || {
                                if wallet_mode.get() == "connect" {
                                    view! {
                                        <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-md p-4">
                                            <p class="text-sm text-blue-700 dark:text-blue-300 mb-4">
                                                "Connect your wallet directly using the button below, or manually enter a wallet address."
                                            </p>

                                            <div class="mt-4">
                                                <p class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                                                    "MetaMask Wallet:"
                                                </p>
                                                {metamask_section()}
                                            </div>
                                        </div>
                                    }
                                        .into_any()
                                } else {
                                    view! { <></> }.into_any()
                                }
                            }}
                        </div>

                        <Show when=move || success_message.get().is_some()>
                            <div class="mt-4 p-3 bg-green-100 dark:bg-green-900/20 border border-green-400 dark:border-green-800 text-green-700 dark:text-green-300 rounded">
                                {move || success_message.get().unwrap_or_default()}
                            </div>
                        </Show>

                        <Show when=move || error.get().is_some()>
                            <div class="mt-4 p-3 bg-red-100 dark:bg-red-900/20 border border-red-400 dark:border-red-800 text-red-700 dark:text-red-300 rounded">
                                {move || error.get().unwrap_or_default()}
                            </div>
                        </Show>

                        <div class="mt-6 flex space-x-3">
                            <button
                                type="submit"
                                disabled=move || loading.get()
                                class="flex-1 bg-purple-600 text-white py-2 px-4 rounded-md hover:bg-purple-700 disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus:ring-2 focus:ring-purple-500"
                            >
                                {move || {
                                    if loading.get() {
                                        "Processing..."
                                    } else {
                                        match wallet_mode.get().as_str() {
                                            "generate" => "Generate Wallet",
                                            "import" => "Import Wallet",
                                            _ => "Add Wallet",
                                        }
                                    }
                                }}
                            </button>
                            <button
                                type="button"
                                class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-md hover:bg-gray-50 dark:hover:bg-neutral-700"
                                on:click=move |_| {
                                    window().location().set_href("/wallets").unwrap();
                                }
                            >
                                "Cancel"
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}

#[server]
pub async fn generate_new_evm_wallet(
    label: String,
    chain_id: String,
    is_primary: bool,
) -> Result<(), leptos::server_fn::ServerFnError> {
    use crate::wallet::wallet::Wallet;
    let user = crate::session::get_user().await?;

    // Generate the new EVM wallet
    Wallet::generate_evm(user.id, label, Some(chain_id), is_primary).await?;

    Ok(())
}

#[server]
pub async fn import_evm_wallet_with_private_key(
    label: String,
    private_key: String,
    chain_id: String,
    is_primary: bool,
) -> Result<(), leptos::server_fn::ServerFnError> {
    use crate::StorageAuthed;
    use crate::keys::{Key, KeyCreate};
    use crate::wallet::wallet::{CreateWallet, Wallet};
    use alloy::signers::local::PrivateKeySigner;
    use surrealdb::Datetime;

    let user = crate::session::get_user().await?;
    let now = chrono::Utc::now();

    // Parse the private key (remove 0x prefix if present)
    let private_key_clean = private_key.trim_start_matches("0x");

    // Convert hex string to bytes
    let private_key_bytes = hex::decode(private_key_clean).map_err(|e| {
        leptos::server_fn::ServerFnError::new(format!("Invalid private key format: {}", e))
    })?;

    // Convert to fixed size array for alloy
    let mut key_bytes = [0u8; 32];
    if private_key_bytes.len() != 32 {
        return Err(leptos::server_fn::ServerFnError::new(
            "Private key must be 32 bytes",
        ));
    }
    key_bytes.copy_from_slice(&private_key_bytes);

    // Create a signer from the private key
    let signer = PrivateKeySigner::from_bytes(&key_bytes.into()).map_err(|e| {
        leptos::server_fn::ServerFnError::new(format!("Invalid private key: {}", e))
    })?;

    // Get the address
    let address = format!("{:?}", signer.address());

    // If setting as primary, unset other wallets first
    if is_primary {
        use crate::db_init;
        let client = db_init().await?;
        client
            .query("UPDATE wallet SET is_primary = false WHERE created_by_user_id = $user_id;")
            .bind(("user_id", user.id.clone()))
            .await?;
    }

    // Create the wallet record
    let wallet = Wallet::new(CreateWallet {
        address: address.clone(),
        label: label.clone(),
        wallet_type: "imported".to_string(),
        chain_type: Some("evm".to_string()),
        chain_id: Some(chain_id),
        created_by_user_id: user.id.clone(),
        created_at: Datetime::from(now),
        updated_at: Datetime::from(now),
        is_primary,
    })
    .await?;

    // Store the private key securely in the Key storage
    let key_data = KeyCreate {
        name: format!("EVM Wallet: {}", label),
        key_for: Some(wallet.id.clone()),
        key_public: Some(address),
        key_private: Some(private_key_clean.to_string()),
        key_apikey: None,
        key_token: None,
        description: format!("Imported private key for EVM wallet: {}", label),
        expires_at: None,
    };

    // Get the user to create the key
    use crate::user::AdapterUser;
    let adapter_user = AdapterUser::get_user(user.id).await?;
    Key::create_by_user(adapter_user, key_data).await?;

    Ok(())
}

#[server]
pub async fn create_wallet_with_data(
    address: String,
    label: String,
    wallet_type: String,
    is_primary: bool,
) -> Result<(), leptos::server_fn::ServerFnError> {
    // Determine chain type based on wallet type
    let (chain_type, chain_id) = match wallet_type.as_str() {
        "metamask" => (Some("evm".to_string()), Some("0x1".to_string())), // Default to mainnet
        "phantom" | "solflare" | "backpack" => (Some("solana".to_string()), None),
        _ => (None, None),
    };
    use crate::wallet::wallet::{CreateWallet, Wallet};
    let user = crate::session::get_user().await?;
    let now = chrono::Utc::now();

    use surrealdb::Datetime;

    // If setting as primary, unset other wallets first
    if is_primary {
        use crate::db_init;
        let client = db_init().await?;
        client
            .query("UPDATE wallet SET is_primary = false WHERE created_by_user_id = $user_id;")
            .bind(("user_id", user.id.clone()))
            .await?;
    }

    let _new_wallet = Wallet::new(CreateWallet {
        address,
        label,
        wallet_type,
        chain_type,
        chain_id,
        created_by_user_id: user.id,
        created_at: Datetime::from(now),
        updated_at: Datetime::from(now),
        is_primary,
    })
    .await?;

    Ok(())
}
