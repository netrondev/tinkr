use leptos::prelude::*;
use crate::components::NavigationBackButton;

use crate::wallet::evm::{
    chains::RpcInfo, get_account_info, get_block_info, get_chain_info, RpcSearch,
};

#[component]
pub fn EthereumDemo() -> impl IntoView {
    let selected_rpc = RwSignal::new(None::<String>);
    let selected_rpc_info = RwSignal::new(None::<RpcInfo>);

    let chain_info = Action::new(move |_: &()| {
        let rpc = selected_rpc.get_untracked();
        async move { get_chain_info(rpc).await }
    });

    let block_info = Action::new(move |block: &Option<u64>| {
        let block = *block;
        let rpc = selected_rpc.get_untracked();
        async move { get_block_info(rpc, block).await }
    });

    let account_info = Action::new(move |address: &String| {
        let address = address.clone();
        let rpc = selected_rpc.get_untracked();
        async move { get_account_info(rpc, address).await }
    });

    let (address_input, set_address_input) = signal(String::new());

    view! {
        <div class="p-4 space-y-4 min-h-screen bg-white dark:bg-neutral-900 text-neutral-900 dark:text-neutral-100">
            <div class="flex items-center space-x-3">
                <NavigationBackButton />
                <h2 class="text-2xl font-bold text-neutral-900 dark:text-neutral-100">
                    "Ethereum RPC Demo"
                </h2>
            </div>

            // RPC Selection Section
            <div class="border border-neutral-200 dark:border-neutral-700 rounded-lg p-4 bg-neutral-50 dark:bg-neutral-800">
                <h3 class="text-lg font-semibold mb-2 text-neutral-800 dark:text-neutral-200">
                    "Select RPC Endpoint"
                </h3>
                <RpcSearch on_select=move |rpc| {
                    selected_rpc.set(Some(rpc.rpc_url.clone()));
                    selected_rpc_info.set(Some(rpc));
                } />

                // Show selected RPC info
                <Show when=move || {
                    selected_rpc_info.get().is_some()
                }>
                    {move || {
                        let rpc_info = selected_rpc_info.get().unwrap();
                        view! {
                            <div class="mt-4 p-3 bg-neutral-100 dark:bg-neutral-700 rounded">
                                <div class="flex items-start space-x-3">
                                    {rpc_info
                                        .icon
                                        .clone()
                                        .map(|icon| {
                                            view! {
                                                <img
                                                    src=format!(
                                                        "https://icons.llamao.fi/icons/chains/rsz_{}.jpg",
                                                        icon,
                                                    )
                                                    alt=rpc_info.name.clone()
                                                    class="w-10 h-10 rounded-full object-cover flex-shrink-0"
                                                    onerror="this.style.display='none'"
                                                />
                                            }
                                                .into_any()
                                        })
                                        .unwrap_or_else(|| {
                                            view! {
                                                <div class="w-10 h-10 rounded-full bg-neutral-300 dark:bg-neutral-600 flex items-center justify-center flex-shrink-0">
                                                    <span class="text-sm font-bold">
                                                        {rpc_info.name.chars().next().unwrap_or('?')}
                                                    </span>
                                                </div>
                                            }
                                                .into_any()
                                        })} <div class="flex-1 min-w-0">
                                        <p class="text-sm font-medium">"Selected RPC:"</p>
                                        <p class="text-sm">
                                            {rpc_info.name.clone()} " • Chain ID: "
                                            {rpc_info.chain_id}
                                        </p>
                                        <p class="text-xs text-neutral-600 dark:text-neutral-400 truncate">
                                            {rpc_info.rpc_url.clone()}
                                        </p>
                                        {rpc_info
                                            .explorer_url
                                            .clone()
                                            .map(|url| {
                                                view! {
                                                    <a
                                                        href=url
                                                        target="_blank"
                                                        class="text-xs text-blue-500 hover:underline"
                                                    >
                                                        "View Explorer"
                                                    </a>
                                                }
                                            })}
                                    </div>
                                </div>
                            </div>
                        }
                    }}
                </Show>
            </div>

            // Chain Info Section
            <div class="border border-neutral-200 dark:border-neutral-700 rounded-lg p-4 bg-neutral-50 dark:bg-neutral-800">
                <h3 class="text-lg font-semibold mb-2 text-neutral-800 dark:text-neutral-200">
                    "Chain Information"
                </h3>
                <button
                    class="bg-blue-500 hover:bg-blue-600 dark:bg-blue-600 dark:hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-md disabled:opacity-50 disabled:cursor-not-allowed"
                    on:click=move |_| {
                        chain_info.dispatch(());
                    }
                    disabled=move || selected_rpc.get().is_none()
                >
                    {move || {
                        if selected_rpc.get().is_none() {
                            "Select an RPC first"
                        } else {
                            "Get Chain Info"
                        }
                    }}
                </button>
                {move || match chain_info.value().get() {
                    Some(Ok(chaininfo)) => {
                        view! {
                            <div class="mt-4 space-y-3 p-4 bg-white dark:bg-neutral-900 rounded-lg border border-neutral-200 dark:border-neutral-700">
                                <p class="font-semibold text-lg text-neutral-800 dark:text-neutral-200">
                                    "Chain Information:"
                                </p>
                                <div class="space-y-2 text-sm text-neutral-700 dark:text-neutral-300">
                                    <p>"Chain ID: " {chaininfo.chain_id}</p>
                                    <p>"Network ID: " {chaininfo.info.network_id}</p>
                                    <p>"Name: " {chaininfo.name}</p>
                                    <p>"Short Name: " {chaininfo.info.short_name}</p>
                                    <p>"Chain: " {chaininfo.info.chain}</p>
                                    <p>"Latest Block: " {chaininfo.latest_block}</p>
                                    <p>"Gas Price: " {chaininfo.gas_price} " wei"</p>

                                    <p class="font-semibold mt-3 text-neutral-800 dark:text-neutral-200">
                                        "Native Currency:"
                                    </p>
                                    <div class="ml-4 p-2 bg-neutral-100 dark:bg-neutral-800 rounded">
                                        <p>"Name: " {chaininfo.info.native_currency.name}</p>
                                        <p>"Symbol: " {chaininfo.info.native_currency.symbol}</p>
                                        <p>
                                            "Decimals: " {chaininfo.info.native_currency.decimals}
                                        </p>
                                    </div>

                                    <p>
                                        "Info URL: "
                                        <a
                                            href=chaininfo.info.info_url.clone()
                                            target="_blank"
                                            class="text-blue-500 dark:text-blue-400 hover:underline"
                                        >
                                            {chaininfo.info.info_url.clone()}
                                        </a>
                                    </p>

                                    {chaininfo
                                        .info
                                        .slip44
                                        .map(|slip| view! { <p>"SLIP-44: " {slip}</p> })}
                                    {chaininfo
                                        .info
                                        .tvl
                                        .map(|tvl| {
                                            view! { <p>"TVL: $" {format!("{:.2}", tvl)}</p> }
                                        })}
                                    {chaininfo
                                        .info
                                        .status
                                        .as_ref()
                                        .map(|status| view! { <p>"Status: " {status.clone()}</p> })}
                                    {chaininfo
                                        .info
                                        .chain_slug
                                        .as_ref()
                                        .map(|slug| view! { <p>"Chain Slug: " {slug.clone()}</p> })}

                                    {(!chaininfo.info.faucets.is_empty())
                                        .then(|| {
                                            view! {
                                                <div>
                                                    <p class="font-semibold mt-3 text-neutral-800 dark:text-neutral-200">
                                                        "Faucets:"
                                                    </p>
                                                    <ul class="ml-4 list-disc">
                                                        {chaininfo
                                                            .info
                                                            .faucets
                                                            .iter()
                                                            .map(|faucet| {
                                                                view! {
                                                                    <li>
                                                                        <a
                                                                            href=faucet.clone()
                                                                            target="_blank"
                                                                            class="text-blue-500 dark:text-blue-400 hover:underline"
                                                                        >
                                                                            {faucet.clone()}
                                                                        </a>
                                                                    </li>
                                                                }
                                                            })
                                                            .collect::<Vec<_>>()}
                                                    </ul>
                                                </div>
                                            }
                                        })}

                                    {chaininfo
                                        .info
                                        .explorers
                                        .as_ref()
                                        .and_then(|explorers| {
                                            (!explorers.is_empty())
                                                .then(|| {
                                                    view! {
                                                        <div>
                                                            <p class="font-semibold mt-3 text-neutral-800 dark:text-neutral-200">
                                                                "Explorers:"
                                                            </p>
                                                            <ul class="ml-4 list-disc">
                                                                {explorers
                                                                    .iter()
                                                                    .map(|explorer| {
                                                                        view! {
                                                                            <li>
                                                                                <a
                                                                                    href=explorer.url.clone()
                                                                                    target="_blank"
                                                                                    class="text-blue-500 dark:text-blue-400 hover:underline"
                                                                                >
                                                                                    {explorer.name.clone()}
                                                                                    {explorer
                                                                                        .standard
                                                                                        .as_ref()
                                                                                        .map(|std| format!(" ({})", std))
                                                                                        .unwrap_or_default()}
                                                                                </a>
                                                                            </li>
                                                                        }
                                                                    })
                                                                    .collect::<Vec<_>>()}
                                                            </ul>
                                                        </div>
                                                    }
                                                })
                                        })}

                                    {chaininfo
                                        .info
                                        .features
                                        .as_ref()
                                        .and_then(|features| {
                                            (!features.is_empty())
                                                .then(|| {
                                                    view! {
                                                        <div>
                                                            <p class="font-semibold mt-3 text-neutral-800 dark:text-neutral-200">
                                                                "Features:"
                                                            </p>
                                                            <ul class="ml-4 list-disc">
                                                                {features
                                                                    .iter()
                                                                    .map(|feature| {
                                                                        view! { <li>{feature.name.clone()}</li> }
                                                                    })
                                                                    .collect::<Vec<_>>()}
                                                            </ul>
                                                        </div>
                                                    }
                                                })
                                        })}

                                    {chaininfo
                                        .info
                                        .ens
                                        .as_ref()
                                        .map(|ens| {
                                            view! {
                                                <div>
                                                    <p class="font-semibold mt-3 text-neutral-800 dark:text-neutral-200">
                                                        "ENS:"
                                                    </p>
                                                    <p class="ml-4">"Registry: " {ens.registry.clone()}</p>
                                                </div>
                                            }
                                        })}

                                    {chaininfo
                                        .info
                                        .parent
                                        .as_ref()
                                        .map(|parent| {
                                            view! {
                                                <div>
                                                    <p class="font-semibold mt-3 text-neutral-800 dark:text-neutral-200">
                                                        "Parent Chain:"
                                                    </p>
                                                    <div class="ml-4 p-2 bg-neutral-100 dark:bg-neutral-800 rounded">
                                                        <p>"Type: " {parent.parent_type.clone()}</p>
                                                        <p>"Chain: " {parent.chain.clone()}</p>
                                                        {parent
                                                            .bridges
                                                            .as_ref()
                                                            .and_then(|bridges| {
                                                                (!bridges.is_empty())
                                                                    .then(|| {
                                                                        view! {
                                                                            <div>
                                                                                <p>"Bridges:"</p>
                                                                                <ul class="ml-4 list-disc">
                                                                                    {bridges
                                                                                        .iter()
                                                                                        .map(|bridge| {
                                                                                            view! {
                                                                                                <li>
                                                                                                    <a
                                                                                                        href=bridge.url.clone()
                                                                                                        target="_blank"
                                                                                                        class="text-blue-500 dark:text-blue-400 hover:underline"
                                                                                                    >
                                                                                                        {bridge.url.clone()}
                                                                                                    </a>
                                                                                                </li>
                                                                                            }
                                                                                        })
                                                                                        .collect::<Vec<_>>()}
                                                                                </ul>
                                                                            </div>
                                                                        }
                                                                    })
                                                            })}
                                                    </div>
                                                </div>
                                            }
                                        })}

                                    <div>
                                        <p class="font-semibold mt-3 text-neutral-800 dark:text-neutral-200">
                                            "Available RPCs:"
                                        </p>
                                        <ul class="ml-4 list-disc">
                                            {chaininfo
                                                .info
                                                .rpc
                                                .iter()
                                                .take(5)
                                                .map(|rpc| {
                                                    view! {
                                                        <li class="text-xs">
                                                            {rpc.url.clone()}
                                                            {rpc
                                                                .is_open_source
                                                                .unwrap_or(false)
                                                                .then(|| " (Open Source)")}
                                                        </li>
                                                    }
                                                })
                                                .collect::<Vec<_>>()}
                                            {(chaininfo.info.rpc.len() > 5)
                                                .then(|| {
                                                    view! {
                                                        <li class="text-xs text-neutral-500 dark:text-neutral-400">
                                                            "... and " {chaininfo.info.rpc.len() - 5} " more"
                                                        </li>
                                                    }
                                                })}
                                        </ul>
                                    </div>
                                </div>
                            </div>
                        }
                            .into_any()
                    }
                    Some(Err(e)) => {
                        view! {
                            <p class="text-red-500 dark:text-red-400 mt-2">
                                "Error: " {e.to_string()}
                            </p>
                        }
                            .into_any()
                    }
                    None => view! { <div /> }.into_any(),
                }}
            </div>

            // Block Info Section
            <div class="border border-neutral-200 dark:border-neutral-700 rounded-lg p-4 bg-neutral-50 dark:bg-neutral-800">
                <h3 class="text-lg font-semibold mb-2 text-neutral-800 dark:text-neutral-200">
                    "Block Information"
                </h3>
                <div class="space-x-2">
                    <button
                        class="bg-green-500 hover:bg-green-600 dark:bg-green-600 dark:hover:bg-green-700 text-white font-bold py-2 px-4 rounded-md disabled:opacity-50 disabled:cursor-not-allowed"
                        on:click=move |_| {
                            block_info.dispatch(None);
                        }
                        disabled=move || selected_rpc.get().is_none()
                    >
                        {move || {
                            if selected_rpc.get().is_none() {
                                "Select an RPC first"
                            } else {
                                "Get Latest Block"
                            }
                        }}
                    </button>
                    <button
                        class="bg-green-500 hover:bg-green-600 dark:bg-green-600 dark:hover:bg-green-700 text-white font-bold py-2 px-4 rounded-md disabled:opacity-50 disabled:cursor-not-allowed"
                        on:click=move |_| {
                            block_info.dispatch(Some(1));
                        }
                        disabled=move || selected_rpc.get().is_none()
                    >
                        "Get Block #1"
                    </button>
                </div>
                {move || match block_info.value().get() {
                    Some(Ok(info)) => {
                        view! {
                            <div class="mt-4 space-y-2 p-4 bg-white dark:bg-neutral-900 rounded-lg border border-neutral-200 dark:border-neutral-700 text-sm text-neutral-700 dark:text-neutral-300">
                                <p>"Block Number: " <span class="font-mono">{info.number}</span></p>
                                <p>
                                    "Block Hash: "
                                    <span class="font-mono text-xs break-all">{info.hash}</span>
                                </p>
                                <p>"Timestamp: " {info.timestamp}</p>
                                <p>"Gas Used: " {info.gas_used}</p>
                                <p>"Gas Limit: " {info.gas_limit}</p>
                            </div>
                        }
                            .into_any()
                    }
                    Some(Err(e)) => {
                        view! {
                            <p class="text-red-500 dark:text-red-400 mt-2">
                                "Error: " {e.to_string()}
                            </p>
                        }
                            .into_any()
                    }
                    None => view! { <div /> }.into_any(),
                }}
            </div>

            // Account Info Section
            <div class="border border-neutral-200 dark:border-neutral-700 rounded-lg p-4 bg-neutral-50 dark:bg-neutral-800">
                <h3 class="text-lg font-semibold mb-2 text-neutral-800 dark:text-neutral-200">
                    "Account Information"
                </h3>
                <div class="space-y-2">
                    <input
                        type="text"
                        placeholder="Enter Ethereum address (0x...)"
                        class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-900 text-neutral-900 dark:text-neutral-100 placeholder-neutral-400 dark:placeholder-neutral-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400"
                        on:input=move |ev| set_address_input.set(event_target_value(&ev))
                        prop:value=move || address_input.get()
                    />
                    <button
                        class="bg-purple-500 hover:bg-purple-600 dark:bg-purple-600 dark:hover:bg-purple-700 text-white font-bold py-2 px-4 rounded-md disabled:opacity-50 disabled:cursor-not-allowed"
                        on:click=move |_| {
                            let addr = address_input.get();
                            if !addr.is_empty() && selected_rpc.get().is_some() {
                                account_info.dispatch(addr);
                            }
                        }
                        disabled=move || {
                            selected_rpc.get().is_none() || address_input.get().is_empty()
                        }
                    >
                        {move || {
                            if selected_rpc.get().is_none() {
                                "Select an RPC first"
                            } else {
                                "Get Account Info"
                            }
                        }}
                    </button>
                </div>
                {move || match account_info.value().get() {
                    Some(Ok(info)) => {
                        view! {
                            <div class="mt-4 space-y-2 p-4 bg-white dark:bg-neutral-900 rounded-lg border border-neutral-200 dark:border-neutral-700 text-sm text-neutral-700 dark:text-neutral-300">
                                <p>
                                    "Address: "
                                    <span class="font-mono text-xs">{info.address}</span>
                                </p>
                                <p>
                                    "Balance: " <span class="font-mono">{info.balance}</span> " wei"
                                </p>
                                <p>"Nonce: " <span class="font-mono">{info.nonce}</span></p>
                            </div>
                        }
                            .into_any()
                    }
                    Some(Err(e)) => {
                        view! {
                            <p class="text-red-500 dark:text-red-400 mt-2">
                                "Error: " {e.to_string()}
                            </p>
                        }
                            .into_any()
                    }
                    None => view! { <div /> }.into_any(),
                }}
            </div>
        </div>
    }
}
