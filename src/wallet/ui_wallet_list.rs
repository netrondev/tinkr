use crate::wallet::Wallet;
use leptos::prelude::*;
use crate::{
    AppError,
    components::{
        Button,
        button::{BtnColor, ButtonIcon},
    },
};

use phosphor_leptos::{Icon, WALLET};

#[component]
pub fn WalletItem(wallet: Wallet) -> impl IntoView {
    view! {
        <div class="p-2 rounded-lg bg-neutral-100 dark:bg-neutral-800 hover:bg-neutral-200 dark:hover:bg-neutral-800">
            <div class="flex items-center gap-2 pl-2">
                <Icon icon=WALLET />
                <div class="text-sm font-medium text-left text-gray-900 dark:text-gray-100 flex-1">
                    {wallet.label.clone()}
                    {if wallet.is_primary {
                        view! { <span class="ml-2 text-xs bg-purple-100 dark:bg-purple-900 text-purple-800 dark:text-purple-200 px-2 py-0.5 rounded-full">"Primary"</span> }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400">
                    {wallet.wallet_type.clone()}
                    {wallet.chain_type.as_ref().map(|chain| {
                        view! {
                            <span class="ml-1">
                                "("{chain.clone()}")"
                            </span>
                        }
                    })}
                </div>
                <div class="text-xs text-gray-400 dark:text-gray-500 font-mono">
                    {wallet.address.chars().take(6).collect::<String>() + "..." + &wallet.address.chars().rev().take(4).collect::<String>().chars().rev().collect::<String>()}
                </div>

                <div class="flex items-center space-x-2">
                    <Button
                        color=BtnColor::Neutral
                        href={format!("/wallets/a/{}", wallet.address.clone())}
                        icon=ButtonIcon::Icon(phosphor_leptos::WALLET)
                    >
                        "View"
                    </Button>

                    <Button
                        icon=ButtonIcon::Icon(phosphor_leptos::GEAR)
                        color=BtnColor::Neutral
                        href={format!("/wallets/w/{}", wallet.id)}
                    >
                        "Manage"
                    </Button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn WalletList() -> impl IntoView {
    let wallets_resource = Resource::new(|| (), |_| get_user_wallets());

    view! {
        <Suspense fallback=move || view! {
            <div class="bg-white dark:bg-neutral-800 rounded-lg shadow">
                <div class="p-6">
                    <div class="animate-pulse space-y-4">
                        <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-3/4"></div>
                        <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-1/2"></div>
                        <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-5/6"></div>
                    </div>
                </div>
            </div>
        }>
            {move || {
                match wallets_resource.get() {
                    Some(Ok(wallets)) => {
                        if wallets.is_empty() {
                            view! {
                                <div class="bg-white dark:bg-neutral-800 rounded-lg shadow">
                                    <div class="p-6">
                                        <p class="text-gray-500 dark:text-gray-400">"No wallets connected. Connect your first wallet to get started."</p>
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="">
                                    <div class="gap-2 flex flex-col">
                                        {wallets
                                            .into_iter()
                                            .map(|wallet| {
                                                view! {
                                                    <WalletItem wallet=wallet />
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }
                    Some(Err(e)) => {
                        view! {
                            <div class="bg-white dark:bg-neutral-800 rounded-lg shadow">
                                <div class="p-6">
                                    <div class="text-red-600 dark:text-red-400">
                                        <p>"Error loading wallets: " {format!("{:?}", e)}</p>
                                        <button
                                            class="mt-2 text-sm text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
                                            on:click=move |_| wallets_resource.refetch()
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
    }
}

#[server]
pub async fn get_user_wallets() -> Result<Vec<Wallet>, leptos::server_fn::ServerFnError> {
    let user = crate::session::get_user().await?;
    let wallets = Wallet::get_by_user(user.id.into()).await?;
    Ok(wallets)
}

#[server]
pub async fn get_user_wallet() -> Result<Option<Wallet>, AppError> {
    let user = crate::session::get_user().await?;

    let wallets = Wallet::get_by_user(user.id.into()).await;

    let firstwallet: Option<Wallet> = match wallets {
        Ok(wallets) => {
            if wallets.is_empty() {
                return Ok(None);
            }
            wallets.first().cloned()
        }
        Err(_e) => None,
    };

    Ok(firstwallet)
}
