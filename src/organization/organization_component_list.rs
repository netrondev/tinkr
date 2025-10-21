#[cfg(feature = "ssr")]
use std::str::FromStr;

use super::organization::Organization;

#[cfg(feature = "ssr")]
use crate::session::get_user;

use crate::team::team_management_basic::TeamManagement;
use crate::users::UsersHeader;
use crate::components::button::{BtnColor, BtnVariant, ButtonIcon};
use crate::components::Button;

// #[cfg(not(feature = "ssr"))]
// use crate::RecordId;

#[cfg(feature = "ssr")]
use surrealdb::RecordId;

use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params};
use leptos_router::params::Params;
use phosphor_leptos::USERS;
use web_sys::MouseEvent;

#[derive(Params, PartialEq, Clone)]
struct OrganizationParams {
    org_id: Option<String>,
}

#[server]
async fn get_organization(org_id: String) -> Result<Organization, ServerFnError> {
    let record_id =
        RecordId::from_str(&org_id).map_err(|_| ServerFnError::new("Invalid organization ID"))?;
    let org = Organization::get_by_id(record_id.into()).await?;
    Ok(org)
}

#[component]
pub fn OrganizationDetail() -> impl IntoView {
    let params = use_params::<OrganizationParams>();
    let navigate = use_navigate();

    let org_resource = Resource::new(
        move || {
            params
                .get()
                .and_then(|p| Ok(p.org_id.clone().unwrap_or_default()))
                .unwrap_or_default()
        },
        |org_id| async move {
            if org_id.is_empty() {
                Err(ServerFnError::new("Invalid organization ID"))
            } else {
                get_organization(org_id).await
            }
        },
    );

    view! {
        <div>
            <UsersHeader />
            <div class="container mx-auto p-6">
                <Suspense fallback=move || view! { <p>"Loading organization..."</p> }>
                    {move || {
                        org_resource.get().map(|org_result| {
                            match org_result {
                                Ok(org) => {
                                    let org_clone = org.clone();
                                    let org_clone_for_closure = org_clone.clone();
                                    view! {
                                        <div class="max-w-4xl mx-auto">
                                            <div class="bg-white dark:bg-neutral-800 shadow rounded-lg p-6">
                                                <div class="flex justify-between items-start mb-6">
                                                    <div class="flex items-start space-x-4">
                                                        {move || {
                                                            if let Some(logo_url) = &org_clone_for_closure.logo_url {
                                                                view! {
                                                                    <img
                                                                        src=logo_url.clone()
                                                                        alt="Organization logo"
                                                                        class="w-16 h-16 rounded-lg object-cover"
                                                                    />
                                                                }.into_any()
                                                            } else {
                                                                view! {
                                                                    <div class="w-16 h-16 bg-blue-500 rounded-lg flex items-center justify-center">
                                                                        <span class="text-white text-2xl font-bold">
                                                                            {org_clone_for_closure.name.chars().next().unwrap_or('O').to_uppercase().to_string()}
                                                                        </span>
                                                                    </div>
                                                                }.into_any()
                                                            }
                                                        }}
                                                        <div>
                                                            <h1 class="text-2xl font-bold text-neutral-900 dark:text-white">
                                                                {org_clone.name.clone()}
                                                            </h1>
                                                            {org_clone.website.clone().map(|website| view! {
                                                                <a
                                                                    href=website.clone()
                                                                    target="_blank"
                                                                    rel="noopener noreferrer"
                                                                    class="text-blue-600 dark:text-blue-400 hover:underline"
                                                                >
                                                                    {website.clone()}
                                                                </a>
                                                            })}
                                                        </div>
                                                    </div>
                                                    <div>
                                                        <Button
                                                            on:click={
                                                                let navigate = navigate.clone();
                                                                let org_id = org.id.to_string();
                                                                move |_| {
                                                                    navigate(&format!("/users/organizations/{}/edit", org_id), Default::default());
                                                                }
                                                            }
                                                            href={format!("/users/organizations/{}/edit", org.id.to_string())}
                                                            color=BtnColor::Default
                                                            variant=BtnVariant::CallToAction
                                                            icon=ButtonIcon::Icon(USERS)
                                                            class="w-min whitespace-nowrap"
                                                        >
                                                            "Edit Organization"
                                                        </Button>
                                                    </div>
                                                </div>

                                                {org_clone.description.map(|desc| view! {
                                                    <div class="mb-6">
                                                        <h2 class="text-lg font-semibold text-neutral-900 dark:text-white mb-2">
                                                            "Description"
                                                        </h2>
                                                        <p class="text-neutral-600 dark:text-neutral-400">
                                                            {desc}
                                                        </p>
                                                    </div>
                                                })}

                                                <div class="border-t border-neutral-200 dark:border-neutral-700 pt-6">
                                                    <TeamManagement
                                                        organization_id=org.id.to_string()
                                                    />
                                                </div>
                                            </div>
                                        </div>
                                    }.into_any()
                                },
                                Err(err) => view! {
                                    <div class="bg-red-100 dark:bg-red-900/20 border border-red-400 dark:border-red-800 text-red-700 dark:text-red-400 px-4 py-3 rounded">
                                        <p>"Error loading organization: " {err.to_string()}</p>
                                    </div>
                                }.into_any()
                            }
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}

// Server functions
#[server]
pub async fn get_user_organizations() -> Result<Vec<Organization>, ServerFnError> {
    let user = get_user().await?;
    let orgs = Organization::get_user_organizations(user.id.into()).await?;
    Ok(orgs)
}

#[component]
fn OrganizationCard(organization: Organization) -> impl IntoView {
    let navigate = use_navigate();
    let org_clone_for_closure = organization.clone();
    let org_id = organization.id.to_string();
    let org_idc = organization.id.to_string();
    let org_clone = organization.clone();

    view! {
        <div class="bg-white dark:bg-neutral-800 shadow rounded-lg p-6">
            <div class="flex items-start space-x-4">
                 <a on:click=move |ev:MouseEvent| {
                    ev.prevent_default();
                    ev.stop_propagation();
                    navigate(&format!("/users/organizations/{}",  org_id), Default::default());
                }
                     href=move || format!("/users/organizations/{}", org_idc)
                 >
     {move || {
                    if let Some(logo_url) = &org_clone_for_closure.logo_url {
                        view! {
                            <img
                                src=logo_url.clone()
                                alt="Organization logo"
                                class="w-16 h-16 rounded-lg object-cover"
                            />
                        }.into_any()
                    } else {
                        view! {
                            <div class="w-16 h-16 bg-blue-500 rounded-lg flex items-center justify-center">
                                <span class="text-white text-2xl font-bold">
                                    {org_clone_for_closure.name.chars().next().unwrap_or('O').to_uppercase().to_string()}
                                </span>
                            </div>
                        }.into_any()
                    }
                }}
            </a>


                <div>
                    <h1 class="text-2xl font-bold text-neutral-900 dark:text-white">
                        {org_clone.name.clone()}
                    </h1>
                    {org_clone.website.clone().map(|website| view! {
                        <a
                            href=website.clone()
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-blue-600 dark:text-blue-400 hover:underline"
                        >
                            {website.clone()}
                        </a>
                    })}
                </div>
            </div>


        </div>
    }
}

// Organization list view
#[component]
pub fn OrganizationList() -> impl IntoView {
    let navigate = use_navigate();
    let orgs_resource = Resource::new(|| (), |_| async move { get_user_organizations().await });

    view! {
        <div>
            <UsersHeader />
            <div class="container mx-auto p-6">
                <div class="mb-6 flex justify-between items-center w-full">
                    <h2 class="text-2xl font-bold text-neutral-800 dark:text-white">"My Organizations"</h2>
                    <div>
                        <Button
                            on:click=move |_| {
                                navigate("/users/organizations/new", Default::default());
                            }
                            icon=ButtonIcon::Icon(USERS)
                            variant=BtnVariant::Default
                            color=BtnColor::Primary
                        >
                            "Create Organization"
                        </Button>
                    </div>
                </div>

                <Suspense fallback=move || view! { <p>"Loading organizations..."</p> }>
                    {move || {
                        orgs_resource.get().map(|orgs_result| {
                            match orgs_result {
                                Ok(orgs) => view! {
                                    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
                                        {orgs.into_iter().map(|org| view! {
                                            <OrganizationCard organization=org />
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any(),
                                Err(err) => view! {
                                    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
                                        <p>"Error loading organizations: " {err.to_string()}</p>
                                    </div>
                                }.into_any()
                            }
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}
