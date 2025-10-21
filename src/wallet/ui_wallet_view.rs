use std::str::FromStr;

use crate::wallet::Wallet;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use crate::components::{
    Button,
    button::{BtnColor, ButtonIcon},
};
use phosphor_leptos::{COPY, Icon, IconWeight, PENCIL_SIMPLE, WALLET};

#[cfg(feature = "ssr")]
use surrealdb::RecordId;

#[cfg(not(feature = "ssr"))]
use crate::RecordId;

#[component]
pub fn WalletView() -> impl IntoView {
    let params = use_params_map();
    let wallet_id = move || {
        let id_str = params.read().get("wallet_id").unwrap_or_default();
        if id_str.is_empty() {
            None
        } else {
            let record = RecordId::from_str(&id_str).unwrap();
            Some(record)
        }
    };

    let wallet_resource = Resource::new(wallet_id, |id_opt| async move {
        match id_opt {
            Some(id) => get_wallet_by_id(id.clone())
                .await
                .map_err(|e| format!("Problem: {:?} {:?}", e, id)),
            None => Err("Invalid wallet ID".to_string()),
        }
    });

    let copy_address = move |address: String| {
        let _ = window().navigator().clipboard().write_text(&address);
    };

    view! {
        <div class="p-6">
            <Suspense fallback=move || view! {
                <div class="bg-white dark:bg-neutral-800 rounded-lg shadow">
                    <div class="p-6">
                        <div class="animate-pulse space-y-4">
                            <div class="h-6 bg-gray-200 dark:bg-gray-700 rounded w-1/3"></div>
                            <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-1/2"></div>
                            <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-2/3"></div>
                        </div>
                    </div>
                </div>
            }>
                {move || {
                    match wallet_resource.get() {
                        Some(Ok(wallet)) => {
                            // let wallet_address = wallet.address.clone();
                            let wallet_address_clone = wallet.address.clone();
                            let wallet_address_clone_b = wallet.address.clone();
                            view! {
                                <div class="bg-white dark:bg-neutral-800 rounded-lg shadow">
                                    <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
                                        <div class="flex items-center justify-between">
                                            <div class="flex items-center gap-3">
                                                <Icon icon=WALLET color="purple" weight=IconWeight::Duotone size="2rem" />
                                                <div>
                                                    <h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">
                                                        {wallet.label.clone()}
                                                        {if wallet.is_primary {
                                                            view! { <span class="ml-2 text-sm bg-purple-100 dark:bg-purple-900 text-purple-800 dark:text-purple-200 px-2 py-0.5 rounded-full">"Primary"</span> }.into_any()
                                                        } else {
                                                            view! { <span></span> }.into_any()
                                                        }}
                                                    </h1>
                                                    <p class="text-sm text-gray-500 dark:text-gray-400">{wallet.wallet_type.clone()} " Wallet"</p>
                                                </div>
                                            </div>
                                            <a
                                                href=format!("/wallets/w/{}/edit", wallet.id.key())
                                                class="inline-flex items-center gap-2 px-4 py-2 bg-purple-600 text-white rounded-md hover:bg-purple-700"
                                            >
                                                <Icon icon=PENCIL_SIMPLE size="1rem" />
                                                "Edit"
                                            </a>
                                        </div>
                                    </div>

                                    <div class="p-6 space-y-6">
                                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                            <div>
                                                <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3">"Wallet Details"</h3>
                                                <dl class="space-y-3">
                                                    <div>
                                                        <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">"Label"</dt>
                                                        <dd class="text-sm text-gray-900 dark:text-gray-100">{wallet.label.clone()}</dd>
                                                    </div>
                                                    <div>
                                                        <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">"Wallet Type"</dt>
                                                        <dd class="text-sm text-gray-900 dark:text-gray-100">{wallet.wallet_type.clone()}</dd>
                                                    </div>
                                                    <div>
                                                        <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">"Address"</dt>
                                                        <dd class="flex items-center gap-2">
                                                            <span class="text-sm text-gray-900 dark:text-gray-100 font-mono">
                                                                <Button
                                                                    href=format!("/wallets/a/{}", wallet_address_clone.clone())
                                                                    color=BtnColor::Primary
                                                                    icon=ButtonIcon::Icon(phosphor_leptos::WALLET)
                                                                >
                                                                    {wallet_address_clone_b.clone()}
                                                                </Button>
                                                            </span>
                                                            <button
                                                                class="text-gray-400 hover:text-gray-600 dark:text-gray-500 dark:hover:text-gray-300 flex gap-2 items-center"
                                                                title="Copy address"
                                                                on:click=move |_| copy_address(wallet_address_clone.clone())
                                                            >
                                                                <Icon icon=COPY /> Copy
                                                            </button>
                                                        </dd>
                                                    </div>
                                                </dl>
                                            </div>

                                            <div>
                                                <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3">"Wallet Status"</h3>
                                                <div class="space-y-3">
                                                    <div>
                                                        <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">"Connection Status"</dt>
                                                        <dd class="text-sm mt-1">
                                                            <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200">
                                                                "Connected"
                                                            </span>
                                                        </dd>
                                                    </div>
                                                    <div>
                                                        <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">"Primary Wallet"</dt>
                                                        <dd class="text-sm text-gray-900 dark:text-gray-100 mt-1">
                                                            {if wallet.clone().is_primary { "Yes" } else { "No" }}
                                                        </dd>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
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
pub async fn get_wallet_by_id(
    wallet_id: RecordId,
) -> Result<Wallet, leptos::server_fn::ServerFnError> {
    let user = crate::session::get_user().await?;
    let wallet = Wallet::get_by_id_and_user(wallet_id.into(), user.id.into()).await?;
    Ok(wallet)
}
