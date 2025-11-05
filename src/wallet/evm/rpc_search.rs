use leptos::prelude::*;
use crate::components::Input;

use crate::wallet::evm::{chains::RpcInfo, search_rpcs_server};

#[component]
pub fn RpcSearch<F>(on_select: F) -> impl IntoView
where
    F: Fn(RpcInfo) + Clone + Send + Sync + 'static,
{
    let search_query = RwSignal::new(String::new());
    let search_results = RwSignal::new(Vec::<RpcInfo>::new());
    let is_searching = RwSignal::new(false);
    let selected_rpc = RwSignal::new(None::<RpcInfo>);

    let search_action = Action::new(move |query: &String| {
        let query = query.clone();
        async move {
            if query.trim().is_empty() || query.len() < 2 {
                return Ok(vec![]);
            }
            search_rpcs_server(query).await
        }
    });

    // Handle search results
    Effect::new(move || {
        if let Some(result) = search_action.value().get() {
            match result {
                Ok(rpcs) => {
                    search_results.set(rpcs);
                }
                Err(_) => {
                    search_results.set(vec![]);
                }
            }
            is_searching.set(false);
        }
    });

    // Debounced search effect
    Effect::new(move || {
        let query = search_query.get();
        if query.trim().is_empty() {
            search_results.set(vec![]);
            return;
        }

        if query.len() < 2 {
            return;
        }

        is_searching.set(true);
        search_action.dispatch(query);
    });

    // Show dropdown when user is searching, has results, or has typed enough characters
    let should_show_dropdown = move || {
        is_searching.get()
            || !search_results.get().is_empty()
            || (!search_query.get().trim().is_empty() && search_query.get().len() >= 2)
    };

    view! {
        <div class="flex w-full">
            <div class="relative max-w-xl mx-auto w-full">
                <Show when=move || selected_rpc.get().is_some()>
                    <div class="mb-2 p-2 bg-neutral-100 dark:bg-neutral-700 rounded flex items-center justify-between">
                        <div class="flex items-center space-x-2">
                            {move || {
                                if let Some(rpc) = selected_rpc.get() {
                                    view! {
                                        <>
                                            {rpc
                                                .icon
                                                .clone()
                                                .map(|icon| {
                                                    view! {
                                                        <img
                                                            src=format!(
                                                                "https://icons.llamao.fi/icons/chains/rsz_{}.jpg",
                                                                icon,
                                                            )
                                                            alt=rpc.name.clone()
                                                            class="w-6 h-6 rounded-full object-cover"
                                                            onerror="this.style.display='none'"
                                                        />
                                                    }
                                                        .into_any()
                                                })
                                                .unwrap_or_else(|| {
                                                    view! {
                                                        <div class="w-6 h-6 rounded-full bg-neutral-300 dark:bg-neutral-600 flex items-center justify-center">
                                                            <span class="text-xs font-bold">
                                                                {rpc.name.chars().next().unwrap_or('?')}
                                                            </span>
                                                        </div>
                                                    }
                                                        .into_any()
                                                })}
                                            <span class="text-sm">
                                                {format!("{} (Chain ID: {})", rpc.name, rpc.chain_id)}
                                            </span>
                                        </>
                                    }
                                        .into_any()
                                } else {
                                    view! { <></> }.into_any()
                                }
                            }}
                        </div>
                        <button
                            class="text-xs text-blue-500 hover:underline"
                            on:click=move |_| {
                                selected_rpc.set(None);
                                search_query.set(String::new());
                            }
                        >
                            "Change"
                        </button>
                    </div>
                </Show>
                <Input
                    placeholder="Search for blockchain RPCs..."
                    class="w-full"
                    value=search_query
                    on_input=Box::new(move |val| {
                        selected_rpc.set(None);
                        search_query.set(val);
                    })
                />

                <Show when=should_show_dropdown>
                    <div class="absolute left-0 top-full mt-1 w-full bg-white dark:bg-neutral-800 rounded-md shadow-lg ring-1 ring-black ring-opacity-5 dark:ring-neutral-700 z-50 max-h-64 overflow-y-auto">
                        <div class="py-1">
                            <Show when=move || is_searching.get()>
                                <div class="px-4 py-2 text-sm text-neutral-500 dark:text-neutral-400">
                                    "Searching..."
                                </div>
                            </Show>

                            <Show when=move || {
                                !is_searching.get() && search_results.get().is_empty()
                                    && !search_query.get().trim().is_empty()
                                    && (search_query.get().len() >= 2)
                            }>
                                <div class="px-4 py-2 text-sm text-neutral-500 dark:text-neutral-400">
                                    "No RPCs found"
                                </div>
                            </Show>

                            <For
                                each=move || search_results.get()
                                key=|rpc| rpc.chain_id
                                children={
                                    let on_select = on_select.clone();
                                    move |rpc| {
                                        let rpc_clone = rpc.clone();
                                        let rpc_name = rpc.name.clone();
                                        let on_select = on_select.clone();
                                        view! {
                                            <button
                                                class="w-full text-left px-4 py-2 text-sm text-neutral-700 dark:text-neutral-300 hover:bg-neutral-100 dark:hover:bg-neutral-700 cursor-pointer"
                                                on:click=move |_| {
                                                    leptos::logging::log!("Selected RPC: {}", rpc_name);
                                                    selected_rpc.set(Some(rpc_clone.clone()));
                                                    search_query.set(String::new());
                                                    search_results.set(vec![]);
                                                    on_select(rpc_clone.clone());
                                                }
                                            >
                                                <div class="flex items-center space-x-3">
                                                    {rpc
                                                        .icon
                                                        .clone()
                                                        .map(|icon| {
                                                            view! {
                                                                <img
                                                                    src=format!(
                                                                        "https://icons.llamao.fi/icons/chains/rsz_{}.jpg",
                                                                        icon,
                                                                    )
                                                                    alt=rpc.name.clone()
                                                                    class="w-8 h-8 rounded-full object-cover"
                                                                    onerror="this.style.display='none'"
                                                                />
                                                            }
                                                                .into_any()
                                                        })
                                                        .unwrap_or_else(|| {
                                                            view! {
                                                                <div class="w-8 h-8 rounded-full bg-neutral-300 dark:bg-neutral-600 flex items-center justify-center">
                                                                    <span class="text-xs font-bold">
                                                                        {rpc.name.chars().next().unwrap_or('?')}
                                                                    </span>
                                                                </div>
                                                            }
                                                                .into_any()
                                                        })} <div class="flex-1">
                                                        <div class="font-medium">
                                                            {rpc.name.clone()} " - " {rpc.network.clone()}
                                                        </div>
                                                        <div class="text-xs text-neutral-500 dark:text-neutral-400">
                                                            "Chain ID: " {rpc.chain_id} " • "
                                                            {rpc.native_currency.clone()}
                                                        </div>
                                                    </div>
                                                </div>
                                            </button>
                                        }
                                    }
                                }
                            />
                        </div>
                    </div>
                </Show>
            </div>
        </div>
    }
}
