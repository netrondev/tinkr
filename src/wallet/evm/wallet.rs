use crate::wallet::evm::chains::RpcInfo;

#[cfg(feature = "ssr")]
use crate::wallet::evm::get_account_info;

use crate::components::Input;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use futures::stream::{self, StreamExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainBalance {
    pub chain_info: RpcInfo,
    pub balance: String,
    pub nonce: u64,
    pub error: Option<String>,
}

#[server]
pub async fn get_wallet_balances(
    address: String,
    include_testnets: bool,
) -> Result<Vec<ChainBalance>, ServerFnError> {
    // Validate address format
    if !address.starts_with("0x") || address.len() != 42 {
        return Err(ServerFnError::new("Invalid Ethereum address format"));
    }

    #[cfg(feature = "ssr")]
    {
        // Get popular chains
        let popular_chains = vec![
            "ethereum",
            "polygon",
            "arbitrum",
            "optimism",
            "base",
            "avalanche",
            "bnb",
            "fantom",
            "gnosis",
            "celo",
        ];

        let mut all_chains = Vec::new();

        // Search for each popular chain
        for chain in popular_chains {
            use crate::wallet::evm::search_rpcs_server;

            if let Ok(results) = search_rpcs_server(chain.to_string()).await {
                all_chains.extend(results);
            }
        }

        // Filter out testnets if requested
        if !include_testnets {
            all_chains.retain(|chain| {
                !chain.network.to_lowercase().contains("test")
                    && !chain.name.to_lowercase().contains("test")
                    && !chain.name.to_lowercase().contains("goerli")
                    && !chain.name.to_lowercase().contains("sepolia")
                    && !chain.name.to_lowercase().contains("holesky")
            });
        }

        // Remove duplicates by chain_id
        all_chains.sort_by_key(|c| c.chain_id);
        all_chains.dedup_by_key(|c| c.chain_id);

        // Limit to reasonable number
        all_chains.truncate(20);

        // Fetch balances in parallel
        let results: Vec<ChainBalance> = stream::iter(all_chains)
            .map(|chain_info| {
                let address = address.clone();
                async move {
                    match get_account_info(Some(chain_info.rpc_url.clone()), address).await {
                        Ok(account) => ChainBalance {
                            chain_info,
                            balance: account.balance,
                            nonce: account.nonce,
                            error: None,
                        },
                        Err(e) => ChainBalance {
                            chain_info,
                            balance: "0".to_string(),
                            nonce: 0,
                            error: Some(e.to_string()),
                        },
                    }
                }
            })
            .buffer_unordered(10) // Process up to 10 requests concurrently
            .collect()
            .await;

        Ok(results)
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("This function should only be called on the server")
}

#[component]
pub fn WalletExplorer() -> impl IntoView {
    let address_input = RwSignal::new(String::new());
    let (include_testnets, set_include_testnets) = signal(false);
    let balances = RwSignal::new(Vec::<ChainBalance>::new());
    let is_loading = RwSignal::new(false);
    let error_message = RwSignal::new(None::<String>);

    let fetch_balances = Action::new(move |(address, include_testnets): &(String, bool)| {
        let address = address.clone();
        let include_testnets = *include_testnets;
        async move {
            is_loading.set(true);
            error_message.set(None);
            let result = get_wallet_balances(address, include_testnets).await;
            is_loading.set(false);
            result
        }
    });

    // Handle results
    Effect::new(move || {
        if let Some(result) = fetch_balances.value().get() {
            match result {
                Ok(data) => {
                    balances.set(data);
                }
                Err(e) => {
                    error_message.set(Some(e.to_string()));
                    balances.set(vec![]);
                }
            }
        }
    });

    // Format balance from wei to ETH
    let format_balance = |balance: &str| -> String {
        if balance == "0" {
            return "0".to_string();
        }

        // Simple conversion - for production use ethers-rs or similar
        if let Ok(wei) = balance.parse::<u128>() {
            let eth = wei as f64 / 1e18;
            if eth < 0.0001 {
                format!("{:.2e}", eth)
            } else {
                format!("{:.6}", eth)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            }
        } else {
            "Error".to_string()
        }
    };

    view! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold mb-6">"Wallet Explorer"</h1>

            <div class="max-w-2xl mx-auto mb-8">
                <div class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium mb-2">"Wallet Address"</label>
                        <Input
                            placeholder="0x..."
                            value=address_input
                            on_input=Box::new(move |val| address_input.set(val))
                            class="w-full"
                        />
                    </div>

                    <div class="flex items-center space-x-2">
                        <input
                            type="checkbox"
                            id="include-testnets"
                            checked=move || include_testnets.get()
                            on:change=move |_| set_include_testnets.update(|v| *v = !*v)
                            class="rounded"
                        />
                        <label for="include-testnets" class="text-sm">
                            "Include Testnets"
                        </label>
                    </div>

                    <button
                        class="w-full bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded-md disabled:opacity-50 disabled:cursor-not-allowed"
                        on:click=move |_| {
                            let addr = address_input.get();
                            if !addr.is_empty() {
                                fetch_balances.dispatch((addr, include_testnets.get()));
                            }
                        }
                        disabled=move || address_input.get().is_empty() || is_loading.get()
                    >
                        {move || if is_loading.get() { "Loading..." } else { "Check Balances" }}
                    </button>
                </div>

                <Show when=move || error_message.get().is_some()>
                    <div class="mt-4 p-4 bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-200 rounded-lg">
                        {move || error_message.get().unwrap_or_default()}
                    </div>
                </Show>
            </div>

            <Show when=move || !balances.get().is_empty()>
                <div class="space-y-4">
                    <h2 class="text-2xl font-bold mb-4">"Balances Across Chains"</h2>

                    <div class="grid gap-4">
                        <For
                            each=move || balances.get()
                            key=|balance| balance.chain_info.chain_id
                            children=move |balance| {
                                let has_balance = balance.balance != "0" && balance.error.is_none();
                                let balance_formatted = format_balance(&balance.balance);
                                let explorer_url = balance.chain_info.explorer_url.clone();
                                let explorer_url_show = explorer_url.clone();
                                let explorer_url_href = explorer_url.clone();

                                view! {
                                    <div class=if has_balance {
                                        "border-2 border-green-500 dark:border-green-400"
                                    } else {
                                        "border border-neutral-200 dark:border-neutral-700"
                                    }
                                        .to_string() + " rounded-lg p-4 hover:shadow-lg">
                                        <div class="flex items-start justify-between">
                                            <div class="flex items-center space-x-3">
                                                {balance
                                                    .chain_info
                                                    .icon
                                                    .clone()
                                                    .map(|icon| {
                                                        view! {
                                                            <img
                                                                src=format!(
                                                                    "https://icons.llamao.fi/icons/chains/rsz_{}.jpg",
                                                                    icon,
                                                                )
                                                                alt=balance.chain_info.name.clone()
                                                                class="w-10 h-10 rounded-full object-cover"
                                                                onerror="this.style.display='none'"
                                                            />
                                                        }
                                                            .into_any()
                                                    })
                                                    .unwrap_or_else(|| {
                                                        view! {
                                                            <div class="w-10 h-10 rounded-full bg-neutral-300 dark:bg-neutral-600 flex items-center justify-center">
                                                                <span class="text-sm font-bold">
                                                                    {balance.chain_info.name.chars().next().unwrap_or('?')}
                                                                </span>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    })} <div>
                                                    <h3 class="font-semibold">
                                                        {balance.chain_info.name.clone()}
                                                    </h3>
                                                    <p class="text-sm text-neutral-600 dark:text-neutral-400">
                                                        "Chain ID: " {balance.chain_info.chain_id} " • "
                                                        {balance.chain_info.network.clone()}
                                                    </p>
                                                </div>
                                            </div>

                                            <div class="text-right">
                                                <Show
                                                    when=move || balance.error.is_none()
                                                    fallback=move || {
                                                        view! { <p class="text-sm text-red-500">"Error"</p> }
                                                    }
                                                >
                                                    <p class=if has_balance {
                                                        "text-lg font-bold text-green-600 dark:text-green-400"
                                                    } else {
                                                        "text-lg"
                                                    }>
                                                        {balance_formatted.clone()} " "
                                                        {balance.chain_info.native_currency.clone()}
                                                    </p>
                                                    <p class="text-sm text-neutral-600 dark:text-neutral-400">
                                                        "Nonce: " {balance.nonce}
                                                    </p>
                                                </Show>
                                            </div>
                                        </div>

                                        <Show when=move || explorer_url_show.is_some()>
                                            <div class="mt-2">
                                                <a
                                                    href=format!(
                                                        "{}/address/{}",
                                                        explorer_url_href.clone().unwrap_or_default(),
                                                        address_input.get(),
                                                    )
                                                    target="_blank"
                                                    class="text-sm text-blue-500 hover:underline"
                                                >
                                                    "View on Explorer ↗"
                                                </a>
                                            </div>
                                        </Show>
                                    </div>
                                }
                            }
                        />
                    </div>

                    <div class="mt-6 p-4 bg-neutral-100 dark:bg-neutral-800 rounded-lg">
                        <p class="text-sm text-neutral-600 dark:text-neutral-400">
                            "Total chains checked: " {balances.get().len()}
                        </p>
                        <p class="text-sm text-neutral-600 dark:text-neutral-400">
                            "Chains with balance: "
                            {move || {
                                balances
                                    .get()
                                    .iter()
                                    .filter(|b| b.balance != "0" && b.error.is_none())
                                    .count()
                            }}
                        </p>
                    </div>
                </div>
            </Show>
        </div>
    }
}
