use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[cfg(feature = "ssr")]
pub mod ethereum;

#[cfg(feature = "ssr")]
pub use ethereum::*;

#[cfg(feature = "ssr")]
use crate::wallet::evm::chains::search_rpcs;

pub mod chains;
pub mod demo;
pub mod home;
pub mod rpc_search;
pub mod wallet;

pub use demo::EthereumDemo;
pub use home::EvmHome;
pub use rpc_search::RpcSearch;
pub use wallet::WalletExplorer;

// Re-export types for use in server functions
pub mod types {
    use serde::{Deserialize, Serialize};

    use super::chains::Chain;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BlockInfo {
        pub number: u64,
        pub hash: String,
        pub timestamp: u64,
        pub gas_used: String,
        pub gas_limit: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccountInfo {
        pub address: String,
        pub balance: String,
        pub nonce: u64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ChainInfo {
        pub chain_id: u64,
        pub name: String,
        pub latest_block: u64,
        pub gas_price: String,
        pub info: Chain,
    }
}

// Server function for RPC search
#[server]
pub async fn search_rpcs_server(query: String) -> Result<Vec<chains::RpcInfo>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        if query.trim().is_empty() {
            return Ok(vec![]);
        }

        search_rpcs(query)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("This function should only be called on the server")
}

// Server functions for Ethereum RPC calls
#[server]
pub async fn get_chain_info(rpc_url: Option<String>) -> Result<types::ChainInfo, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let client = match rpc_url {
            Some(url) => EthereumClient::new_with_rpc(url).await,
            None => EthereumClient::new().await,
        }
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let chain_info = client
            .get_chain_info()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(chain_info)
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("This function should only be called on the server")
}

#[server]
pub async fn get_block_info(
    rpc_url: Option<String>,
    block_number: Option<u64>,
) -> Result<types::BlockInfo, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let client = match rpc_url {
            Some(url) => EthereumClient::new_with_rpc(url).await,
            None => EthereumClient::new().await,
        }
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let block_info = client
            .get_block_info(block_number)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(block_info)
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("This function should only be called on the server")
}

#[server]
pub async fn get_account_info(
    rpc_url: Option<String>,
    address: String,
) -> Result<types::AccountInfo, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let client = match rpc_url {
            Some(url) => EthereumClient::new_with_rpc(url).await,
            None => EthereumClient::new().await,
        }
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let account_info = client
            .get_account_info(&address)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(account_info)
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("This function should only be called on the server")
}

#[server]
pub async fn get_chain_by_id(chain_id: u64) -> Result<chains::Chain, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        chains::Chain::get_by_chain_id(chain_id)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("This function should only be called on the server")
}

#[component]
pub fn ChainIdToInfo(chain_id: u64) -> impl IntoView {
    use leptos::prelude::*;

    let chain_info = Resource::new(
        move || chain_id,
        |id| async move { get_chain_by_id(id).await },
    );

    view! {
        <Suspense fallback=move || view! { <div class="animate-pulse">"Loading chain info..."</div> }>
            {move || {
                chain_info.get().map(|result| {
                    match result {
                        Ok(chain) => {
                            view! {
                                <div class="flex items-center gap-2">
                                    {chain.icon.as_ref().map(|icon_name| {
                                        view! {
                                            <img
                                                src=format!("https://icons.llamao.fi/icons/chains/rsz_{}.jpg", icon_name)
                                                alt=chain.name.clone()
                                                class="w-6 h-6 rounded-full"
                                                onerror="this.style.display='none'"
                                            />
                                        }
                                    })}
                                    <span class="font-medium">{chain.name}</span>
                                    <span class="text-sm text-neutral-500 dark:text-neutral-400">
                                        "(Chain ID: " {chain.chain_id} ")"
                                    </span>
                                </div>
                            }.into_any()
                        }
                        Err(_) => {
                            view! {
                                <div class="text-sm text-neutral-500 dark:text-neutral-400">
                                    "Chain " {chain_id} " not found"
                                </div>
                            }.into_any()
                        }
                    }
                })
            }}
        </Suspense>
    }
}

// #[component]
// pub fn EvmHeader() -> impl IntoView {
//     let nav_items = vec![
//         // NavItem {
//         //     name: "Explorer",
//         //     url: "/evm".to_string(),
//         // },
//         // NavItem {
//         //     name: "Wallet Lookup",
//         //     url: "/evm/wallet".to_string(),
//         // },
//         // NavItem {
//         //     name: "RPC Demo",
//         //     url: "/evm/demo".to_string(),
//         // },
//     ];

//     view! { <AppHeader title="EVM Explorer" nav_items=nav_items /> }
// }

#[component]
pub fn EvmApp() -> impl IntoView {
    view! {
        <div>
            // <EvmHeader />
            <div class="p-5">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/evm") view=home::EvmHome />
                    <Route path=path!("/evm/wallet") view=wallet::WalletExplorer />
                    <Route path=path!("/evm/demo") view=demo::EthereumDemo />
                </Routes>
            </div>
        </div>
    }
}
