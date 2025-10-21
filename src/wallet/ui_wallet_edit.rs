use crate::wallet::Wallet;

#[cfg(not(feature = "ssr"))]
use crate::RecordId;

#[cfg(feature = "ssr")]
use surrealdb::RecordId;

use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};
use phosphor_leptos::{CHECK, Icon, IconWeight, WALLET, X};

#[cfg(feature = "ssr")]
use crate::session::get_user;

#[component]
pub fn WalletEdit() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();

    let wallet_id = move || {
        let id_str = params.read().get("wallet_id").unwrap_or_default();
        if id_str.is_empty() {
            None
        } else {
            Some(RecordId::from_table_key("wallet", &id_str))
        }
    };

    let wallet_resource = Resource::new(wallet_id, |id_opt| async move {
        match id_opt {
            Some(id) => get_wallet_by_id_for_edit(id)
                .await
                .map_err(|e| format!("{:?}", e)),
            None => Err("Invalid wallet ID".to_string()),
        }
    });

    let (label, set_label) = signal(String::new());
    let (is_primary, set_is_primary) = signal(false);
    let (is_loading, set_is_loading) = signal(false);

    // Update form fields when wallet loads
    Effect::new(move |_| {
        if let Some(Ok(wallet)) = wallet_resource.get() {
            set_label.set(wallet.label.clone());
            set_is_primary.set(wallet.is_primary);
        }
    });

    let update_wallet = Action::new(move |_: &()| {
        let current_wallet_id = wallet_id();
        let current_label = label.get();
        let current_is_primary = is_primary.get();

        async move {
            set_is_loading.set(true);
            let result = match current_wallet_id {
                Some(id) => update_wallet_data(id, current_label, current_is_primary).await,
                None => Err(leptos::server_fn::ServerFnError::ServerError(
                    "Invalid wallet ID".to_string(),
                )),
            };
            set_is_loading.set(false);
            result
        }
    });

    // Clone navigate for the effect
    let navigate_clone = navigate.clone();
    Effect::new(move |_| {
        if let Some(Ok(_)) = update_wallet.value().get() {
            if let Some(id) = wallet_id() {
                navigate_clone(&format!("/wallets/w/{}", id.key()), Default::default());
            }
        }
    });

    view! {
        <div class="max-w-2xl mx-auto p-6">
            <Suspense fallback=move || view! {
                <div class="bg-white dark:bg-neutral-800 rounded-lg shadow">
                    <div class="p-6">
                        <div class="animate-pulse space-y-4">
                            <div class="h-6 bg-gray-200 dark:bg-gray-700 rounded w-1/3"></div>
                            <div class="h-10 bg-gray-200 dark:bg-gray-700 rounded"></div>
                            <div class="h-10 bg-gray-200 dark:bg-gray-700 rounded"></div>
                        </div>
                    </div>
                </div>
            }>
                {move || {
                    match wallet_resource.get() {
                        Some(Ok(wallet)) => {
                            view! {
                                <div class="bg-white dark:bg-neutral-800 rounded-lg shadow">
                                    <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
                                        <div class="flex items-center gap-3">
                                            <Icon icon=WALLET color="purple" weight=IconWeight::Duotone size="2rem" />
                                            <div>
                                                <h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">"Edit Wallet"</h1>
                                                <p class="text-sm text-gray-500 dark:text-gray-400">"Update wallet information"</p>
                                            </div>
                                        </div>
                                    </div>

                                    <form
                                        class="p-6 space-y-6"
                                        on:submit=move |ev| {
                                            ev.prevent_default();
                                            update_wallet.dispatch(());
                                        }
                                    >
                                        <div>
                                            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                                                "Wallet Address"
                                            </label>
                                            <input
                                                type="text"
                                                disabled
                                                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-gray-100 dark:bg-neutral-700 text-gray-500 dark:text-gray-400 cursor-not-allowed font-mono"
                                                value=wallet.address.clone()
                                            />
                                            <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">"Wallet address cannot be changed"</p>
                                        </div>

                                        <div>
                                            <label for="label" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                                                "Wallet Label"
                                            </label>
                                            <input
                                                type="text"
                                                id="label"
                                                required
                                                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-purple-500 bg-white dark:bg-neutral-700 text-gray-900 dark:text-gray-100"
                                                prop:value=move || label.get()
                                                on:input=move |ev| {
                                                    set_label.set(event_target_value(&ev));
                                                }
                                            />
                                        </div>

                                        <div>
                                            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                                                "Wallet Type"
                                            </label>
                                            <input
                                                type="text"
                                                disabled
                                                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-gray-100 dark:bg-neutral-700 text-gray-500 dark:text-gray-400 cursor-not-allowed"
                                                value=wallet.wallet_type.clone()
                                            />
                                            <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">"Wallet type cannot be changed"</p>
                                        </div>

                                        <div>
                                            <div class="flex items-center">
                                                <input
                                                    type="checkbox"
                                                    id="is_primary"
                                                    class="h-4 w-4 text-purple-600 focus:ring-purple-500 border-gray-300 rounded"
                                                    prop:checked=move || is_primary.get()
                                                    on:change=move |ev| {
                                                        set_is_primary.set(event_target_checked(&ev));
                                                    }
                                                />
                                                <label for="is_primary" class="ml-2 block text-sm text-gray-700 dark:text-gray-300">
                                                    "Set as primary wallet"
                                                </label>
                                            </div>
                                            <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
                                                "Primary wallet will be used by default for transactions"
                                            </p>
                                        </div>

                                        {move || {
                                            if let Some(Err(e)) = update_wallet.value().get() {
                                                view! {
                                                    <div class="text-red-600 dark:text-red-400 text-sm">
                                                        "Error: " {format!("{:?}", e)}
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <div></div> }.into_any()
                                            }
                                        }}

                                        <div class="flex gap-3 pt-4">
                                            <button
                                                type="submit"
                                                disabled=move || is_loading.get()
                                                class="inline-flex items-center gap-2 px-4 py-2 bg-purple-600 text-white rounded-md hover:bg-purple-700 disabled:opacity-50 disabled:cursor-not-allowed"
                                            >
                                                <Icon icon=CHECK size="1rem" />
                                                {move || if is_loading.get() { "Saving..." } else { "Save Changes" }}
                                            </button>

                                            <button
                                                type="button"
                                                on:click={
                                                    let navigate = navigate.clone();
                                                    let wallet_id = wallet_id.clone();
                                                    move |_| {
                                                        if let Some(id) = wallet_id() {
                                                            navigate(&format!("/wallets/w/{}", id.key()), Default::default());
                                                        }
                                                    }
                                                }
                                                class="inline-flex items-center gap-2 px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded-md hover:bg-gray-300 dark:hover:bg-gray-600"
                                            >
                                                <Icon icon=X size="1rem" />
                                                "Cancel"
                                            </button>
                                        </div>
                                    </form>
                                </div>
                            }.into_any()
                        }
                        Some(Err(e)) => {
                            view! {
                                <div class="bg-white dark:bg-neutral-800 rounded-lg shadow">
                                    <div class="p-6">
                                        <div class="text-red-600 dark:text-red-400">
                                            <p>"Error loading wallet: " {e}</p>
                                            <button
                                                class="mt-2 text-sm text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
                                                on:click=move |_| wallet_resource.refetch()
                                            >
                                                "Retry"
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            }.into_any()
                        }
                        None => {
                            view! {
                                <div></div>
                            }.into_any()
                        }
                    }
                }}
            </Suspense>
        </div>
    }
}

#[server]
pub async fn get_wallet_by_id_for_edit(
    wallet_id: RecordId,
) -> Result<Wallet, leptos::server_fn::ServerFnError> {
    let user = get_user().await?;
    let wallet = Wallet::get_by_id_and_user(wallet_id, user.id.into()).await?;
    Ok(wallet)
}

#[server]
pub async fn update_wallet_data(
    wallet_id: RecordId,
    label: String,
    is_primary: bool,
) -> Result<(), leptos::server_fn::ServerFnError> {
    let user = get_user().await?;
    Wallet::update_by_id_and_user(wallet_id, user.id.into(), label, is_primary).await?;
    Ok(())
}
