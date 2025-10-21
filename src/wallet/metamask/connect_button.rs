#[cfg(not(feature = "ssr"))]
use crate::metamask::wallet::MetaMaskWallet;

use crate::metamask::wallet::WalletState;

use crate::wallet::evm::ChainIdToInfo;

use crate::components::{
    Align, Button, Dropdown, DropdownItem, DropdownMenu, DropdownSide, DropdownTrigger, Tooltip,
    button::{BtnColor, ButtonIcon},
};
use leptos::prelude::*;

#[cfg(not(feature = "ssr"))]
use leptos::task::spawn_local;

use phosphor_leptos::{CARET_DOWN, Icon, IconWeight, WALLET};

// #[cfg(feature = "ssr")]
// use crate::metamask::checksum::fix_capitalization;

use web_sys::MouseEvent;
#[cfg(not(feature = "ssr"))]
use web_sys::window;

#[component]
pub fn WalletConnectButton() -> impl IntoView {
    let (wallet_state, set_wallet_state) = signal(WalletState::default());

    #[cfg(not(feature = "ssr"))]
    let wallet = StoredValue::new(MetaMaskWallet::new());

    // Check if already connected on mount
    #[cfg(not(feature = "ssr"))]
    {
        use crate::metamask::adapter::{ethereum_request, is_metamask_installed};
        use crate::metamask::verify_save::verify_wallet_belongs_to_user;
        use wasm_bindgen::JsValue;

        Effect::new(move |_| {
            if is_metamask_installed() {
                spawn_local(async move {
                    // Check if we have existing accounts
                    let params = js_sys::Object::new();
                    js_sys::Reflect::set(
                        &params,
                        &JsValue::from_str("method"),
                        &JsValue::from_str("eth_accounts"),
                    )
                    .ok();

                    if let Ok(accounts) = ethereum_request(params.into()).await {
                        let accounts_array = js_sys::Array::from(&accounts);
                        if accounts_array.length() > 0 {
                            if let Some(account) = accounts_array.get(0).as_string() {
                                // Verify the wallet belongs to the logged-in user
                                match verify_wallet_belongs_to_user(account.clone()).await {
                                    Ok(true) => {
                                        // Wallet belongs to user, proceed with connection
                                        let mut w = wallet.get_value();

                                        // Get chain ID
                                        let chain_params = js_sys::Object::new();
                                        js_sys::Reflect::set(
                                            &chain_params,
                                            &JsValue::from_str("method"),
                                            &JsValue::from_str("eth_chainId"),
                                        )
                                        .ok();

                                        let chain_id = if let Ok(chain_result) =
                                            ethereum_request(chain_params.into()).await
                                        {
                                            chain_result.as_string()
                                        } else {
                                            None
                                        };

                                        // Update state
                                        w.state.connected = true;
                                        w.state.account = Some(account.to_lowercase());
                                        w.state.chain_id = chain_id;
                                        set_wallet_state.set(w.state.clone());
                                        wallet.set_value(w.clone());

                                        // Store wallet address in local storage
                                        if let Some(window) = window() {
                                            if let Ok(Some(storage)) = window.local_storage() {
                                                let _ = storage.set_item(
                                                    "wallet_address",
                                                    &account.to_lowercase(),
                                                );
                                            }
                                        }
                                    }
                                    Ok(false) => {
                                        // Wallet doesn't belong to user, disconnect it
                                        leptos::logging::log!(
                                            "Wallet {} doesn't belong to logged-in user, disconnecting",
                                            account
                                        );
                                        // Keep wallet disconnected, don't update state
                                    }
                                    Err(e) => {
                                        leptos::logging::error!(
                                            "Failed to verify wallet ownership: {:?}",
                                            e
                                        );
                                        // On error, keep wallet disconnected for safety
                                    }
                                }
                            }
                        }
                    }
                });
            }
        });
    }

    let connect_wallet = move |ev: MouseEvent| {
        ev.prevent_default();

        #[cfg(not(feature = "ssr"))]
        spawn_local(async move {
            let mut w = wallet.get_value();

            println!("Connecting to wallet...");

            // Clear any existing error and set loading state
            w.state.error = None;
            w.state.loading = true;
            set_wallet_state.set(w.state.clone());

            match w.connect().await {
                Ok(account) => {
                    set_wallet_state.set(w.state.clone());
                    leptos::logging::log!("Connected to MetaMask: {}", account);

                    // Store wallet address in local storage
                    #[cfg(not(feature = "ssr"))]
                    if let Some(window) = window() {
                        if let Ok(Some(storage)) = window.local_storage() {
                            let _ = storage.set_item("wallet_address", &account.to_lowercase());
                        }
                    }

                    // Always request signature for verification (either for existing user or new wallet)
                    // Create a message to sign
                    let timestamp = js_sys::Date::now();
                    let sign_message = format!(
                        "Sign this message to verify you control this wallet address.\n\nTimestamp: {}",
                        timestamp
                    );

                    // Request signature
                    match w.sign_message(&sign_message).await {
                        Ok(signature) => {
                            leptos::logging::log!("Message signed successfully");

                            // Save to database with signature verification
                            let chain_id = w.state.chain_id.clone();
                            spawn_local(async move {
                                use crate::metamask::verify_save::verify_and_save_wallet;

                                let veri_result = verify_and_save_wallet(
                                    account.clone(),
                                    chain_id,
                                    sign_message,
                                    signature,
                                )
                                .await;

                                match veri_result {
                                    Ok(_) => {
                                        leptos::logging::log!(
                                            "Wallet verified and saved successfully"
                                        );

                                        if let Some(window) = window() {
                                            let _ = window.location().set_href("/");
                                        }
                                    }
                                    Err(e) => {
                                        leptos::logging::error!(
                                            "Failed to verify and save wallet: {:?}",
                                            e
                                        );
                                    }
                                }
                            });
                        }
                        Err(err) => {
                            leptos::logging::error!("Failed to sign message: {}", err);
                            w.state.error =
                                Some(format!("Failed to sign verification message: {}", err));
                            w.state.connected = false;
                            w.state.account = None;
                            set_wallet_state.set(w.state.clone());
                        }
                    }
                }
                Err(err) => {
                    w.state.error = Some(err.clone());
                    w.state.loading = false;
                    set_wallet_state.set(w.state.clone());
                }
            }

            wallet.set_value(w);
        });
    };

    let disconnect_wallet = move || {
        set_wallet_state.update(|state| {
            state.connected = false;
            state.account = None;
            state.chain_id = None;
            state.error = None;
            state.loading = false;
        });

        // Clear wallet address from local storage
        #[cfg(not(feature = "ssr"))]
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item("wallet_address");
            }
        }
    };

    let switch_to_mainnet = move || {
        #[cfg(not(feature = "ssr"))]
        spawn_local(async move {
            use crate::metamask::adapter::ethereum_request;
            use wasm_bindgen::JsValue;

            let params = js_sys::Object::new();
            js_sys::Reflect::set(
                &params,
                &JsValue::from_str("method"),
                &JsValue::from_str("wallet_switchEthereumChain"),
            )
            .ok();

            let params_array = js_sys::Array::new();
            let chain_params = js_sys::Object::new();
            js_sys::Reflect::set(
                &chain_params,
                &JsValue::from_str("chainId"),
                &JsValue::from_str("0x1"),
            )
            .ok();
            params_array.push(&chain_params);

            js_sys::Reflect::set(&params, &JsValue::from_str("params"), &params_array).ok();

            if let Err(_) = ethereum_request(params.into()).await {
                set_wallet_state.update(|state| {
                    state.error = Some("Failed to switch chain".to_string());
                });
            }
        });
    };

    view! {
        <div class="flex flex-col items-center">
            {move || {
                let state = wallet_state.get();

                if state.connected {
                    let chain_id_for_name = state.chain_id.clone();

                    view! {
                        <Dropdown class="relative">
                            <DropdownTrigger icon=ButtonIcon::Icon(WALLET)>
                                <span class="font-medium">{
                                    state.account.as_ref().map(|addr| {
                                        if addr.len() > 6 {
                                            format!("{}...{}", &addr[..6], &addr[addr.len()-4..])
                                        } else {
                                            addr.clone()
                                        }
                                    }).unwrap_or_else(|| "No Address".to_string())
                                }</span>
                                <Icon icon=CARET_DOWN />
                            </DropdownTrigger>

                            <DropdownMenu side=DropdownSide::Right>
                                <div class="px-4 py-2 border-b border-neutral-200 dark:border-neutral-700">
                                    <div class="flex items-center space-x-2">
                                        <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                                        <span class="text-sm text-neutral-600 dark:text-neutral-400">Connected</span>
                                    </div>
                                    {chain_id_for_name.as_ref().and_then(|id| {
                                        u64::from_str_radix(id.trim_start_matches("0x"), 16).ok()
                                    }).map(|chain_id| {
                                        view! {
                                            <div class="mt-1">
                                                <ChainIdToInfo chain_id={chain_id} />
                                            </div>
                                        }
                                    })}
                                </div>

                                {move || {
                                    let current_chain_id = wallet_state.get().chain_id;
                                    if current_chain_id.as_ref() != Some(&"0x1".to_string()) {
                                        view! {
                                            <DropdownItem
                                                on_click={Callback::from(switch_to_mainnet)}
                                            >
                                                <span class="text-blue-600 dark:text-blue-400">Switch to Mainnet</span>
                                            </DropdownItem>
                                        }.into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }
                                }}

                                <DropdownItem
                                    on_click={Callback::from(disconnect_wallet)}
                                    class="block w-full text-left px-4 py-2 text-sm text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20".into()
                                >
                                    "Disconnect Wallet"
                                </DropdownItem>
                            </DropdownMenu>
                        </Dropdown>
                    }.into_any()
                } else {
                    if let Some(error) = state.error.clone() {
                        view! {
                            <Tooltip label=format!("{}. Check your wallet.",error) align=Align::Left>
                                <Button
                                    icon=ButtonIcon::Icon(WALLET)
                                    on:click=connect_wallet
                                    disabled=state.loading
                                    color=BtnColor::Error
                                >
                                    "Error"
                                </Button>
                            </Tooltip>
                        }.into_any()
                    } else {
                        view! {
                            <Button
                                icon=ButtonIcon::Icon(WALLET)
                                on:click=connect_wallet
                                disabled=state.loading
                            >
                                {if state.loading { "Connecting..." } else { "Connect Wallet" }}
                            </Button>
                        }.into_any()
                    }
                }
            }}

        </div>
    }
}
