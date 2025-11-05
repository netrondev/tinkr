#[cfg(feature = "ssr")]
use crate::session::get_user;

use crate::organization::organization::Organization;

use crate::{
    colors::Color,
    components::{Dropdown, DropdownItem, DropdownMenu, DropdownTrigger},
};
use leptos::prelude::*;
use phosphor_leptos::{CARET_DOWN, CHECK, GEAR, Icon, IconWeight};
use web_sys::window;

#[server]
pub async fn get_user_organizations() -> Result<Vec<Organization>, ServerFnError> {
    let user = get_user().await?;
    let orgs = Organization::get_user_organizations(user.id.into()).await?;
    Ok(orgs)
}

#[component]
pub fn OrganizationSelector() -> impl IntoView {
    let organizations = Resource::new(|| (), |_| get_user_organizations());
    let selected_org = RwSignal::<Option<Organization>>::new(None);
    let storage_key = "selected_organization_id";

    // Load organization from session storage on mount
    Effect::new(move |_| {
        if let Some(Ok(orgs)) = organizations.get() {
            // Try to load saved organization ID from session storage
            let saved_org_id = window()
                .and_then(|w| w.session_storage().ok())
                .and_then(|storage| storage.and_then(|s| s.get_item(storage_key).ok()))
                .flatten();

            if let Some(saved_id) = saved_org_id {
                // Find the organization with the saved ID
                if let Some(org) = orgs.iter().find(|o| o.id.to_string() == saved_id) {
                    selected_org.set(Some(org.clone()));
                } else if let Some(first_org) = orgs.first() {
                    // Fallback to first org if saved org not found
                    selected_org.set(Some(first_org.clone()));
                }
            } else if let Some(first_org) = orgs.first() {
                // No saved org, use first one
                selected_org.set(Some(first_org.clone()));
            }
        }
    });

    // Save to session storage whenever selection changes
    Effect::new(move |_| {
        if let Some(org) = selected_org.get() {
            if let Some(Ok(Some(storage))) = window().map(|w| w.session_storage()) {
                let _ = storage.set_item(storage_key, &org.id.to_string());
            }
        }
    });

    view! {
        <Dropdown class="relative">
            <DropdownTrigger>
                <Suspense fallback=move || {
                    view! {
                        <div class="flex items-center space-x-2">
                            <div class="w-5 h-5 bg-neutral-300 dark:bg-neutral-600 rounded animate-pulse"></div>
                            <span class="text-neutral-500 dark:text-neutral-400">Loading...</span>
                        </div>
                    }
                }>
                    {move || {
                        if let Some(org) = selected_org.get() {
                            view! {
                                <div class="flex items-center space-x-2">
                                    {if let Some(logo_url) = &org.logo_url {
                                        view! {
                                            <img
                                                src=logo_url
                                                alt=format!("{} logo", org.name)
                                                class="w-5 h-5 rounded object-cover"
                                            />
                                        }
                                            .into_any()
                                    } else {
                                        view! {
                                            <div class="w-5 h-5 bg-blue-500 rounded flex items-center justify-center">
                                                <span class="text-white text-xs font-bold">
                                                    {org
                                                        .name
                                                        .chars()
                                                        .next()
                                                        .unwrap_or('O')
                                                        .to_uppercase()
                                                        .to_string()}
                                                </span>
                                            </div>
                                        }
                                            .into_any()
                                    }} <span>{org.name}</span>
                                    <svg
                                        class="w-4 h-4 text-neutral-500 dark:text-neutral-400"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                        xmlns="http://www.w3.org/2000/svg"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M19 9l-7 7-7-7"
                                        ></path>
                                    </svg>
                                </div>
                            }
                                .into_any()
                        } else {
                            view! {
                                <div class="flex items-center space-x-2">
                                    <div class="w-5 h-5 bg-neutral-400 dark:bg-neutral-600 rounded"></div>
                                    <span class="text-neutral-500 dark:text-neutral-400">
                                        No Organization
                                    </span>
                                    <Icon icon=CARET_DOWN />

                                </div>
                            }
                                .into_any()
                        }
                    }}
                </Suspense>
            </DropdownTrigger>

            <DropdownMenu>
                <Suspense fallback=move || {
                    view! {
                        <div class="p-4 text-center text-neutral-500 dark:text-neutral-400">
                            Loading organizations...
                        </div>
                    }
                }>
                    {move || match organizations.get() {
                        Some(Ok(orgs)) => {
                            view! {
                                <div class="w-full">
                                    <div class="py-1 w-full">
                                        <For
                                            each=move || orgs.clone()
                                            key=|org| org.id.to_string()
                                            children=move |org: Organization| {
                                                let org_clone = org.clone();
                                                let is_selected = move || {
                                                    selected_org
                                                        .get()
                                                        .map(|s| s.id.to_string() == org.id.to_string())
                                                        .unwrap_or(false)
                                                };
                                                view! {
                                                    <DropdownItem // class="flex items-center bg-red-500 justify-between px-4 py-2 text-sm text-neutral-700 dark:text-neutral-300 hover:bg-neutral-100 dark:hover:bg-neutral-700 cursor-pointer"
                                                    on_click={
                                                        let org_clone = org_clone.clone();
                                                        Callback::from(move || {
                                                            selected_org.set(Some(org_clone.clone()));
                                                        })
                                                    }>
                                                        <div class="flex items-center space-x-2 w-full ">
                                                            {if let Some(logo_url) = &org.logo_url {
                                                                view! {
                                                                    <img
                                                                        src=logo_url
                                                                        alt=format!("{} logo", org.name)
                                                                        class="w-5 h-5 rounded object-cover"
                                                                    />
                                                                }
                                                                    .into_any()
                                                            } else {
                                                                view! {
                                                                    <div class="w-5 h-5 bg-blue-500 rounded flex items-center justify-center">
                                                                        <span class="text-white text-xs font-bold">
                                                                            {org
                                                                                .name
                                                                                .chars()
                                                                                .next()
                                                                                .unwrap_or('O')
                                                                                .to_uppercase()
                                                                                .to_string()}
                                                                        </span>
                                                                    </div>
                                                                }
                                                                    .into_any()
                                                            }} <span>{org.name.clone()}</span>
                                                            <Show when=is_selected>
                                                                <Icon
                                                                    weight=IconWeight::Bold
                                                                    icon=CHECK
                                                                    color=Color::from_tailwind("blue-500").hex
                                                                />
                                                            </Show>

                                                        </div>

                                                    </DropdownItem>
                                                }
                                            }
                                        />
                                    </div>
                                    <div class="border-t border-neutral-200 dark:border-neutral-700"></div>
                                    <div class="py-1 w-full min-w-96">
                                        <DropdownItem href="/settings/organizations".into()>
                                            <div class="flex flex-row gap-2 items-center">
                                                <div class="w-5 h-5">
                                                    <Icon icon=GEAR />
                                                </div>
                                                <span class="whitespace-nowrap">Manage Organizations</span>
                                            </div>
                                        </DropdownItem>
                                    </div>
                                </div>
                            }
                                .into_any()
                        }
                        Some(Err(_)) => {
                            view! {
                                <div class="p-4 text-center text-red-500">
                                    Error loading organizations
                                </div>
                            }
                                .into_any()
                        }
                        None => {
                            view! {
                                <div class="p-4 text-center text-neutral-500 dark:text-neutral-400">
                                    Loading...
                                </div>
                            }
                                .into_any()
                        }
                    }}
                </Suspense>
            </DropdownMenu>
        </Dropdown>
    }
}
