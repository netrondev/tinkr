use std::str::FromStr;

use crate::organization::organization::Organization;

use crate::organization::ui_organization_form::{
    OrganizationForm, OrganizationFormData, OrganizationFormMode,
};
#[cfg(feature = "ssr")]
use crate::StorageAuthed;

use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params};
use leptos_router::params::Params;

#[cfg(feature = "ssr")]
use surrealdb::RecordId;

#[cfg(not(feature = "ssr"))]
use crate::RecordId;

#[derive(Params, PartialEq, Clone)]
struct OrganizationParams {
    org_id: Option<String>,
}

#[server]
async fn get_organization_for_edit(org_id: RecordId) -> Result<Organization, ServerFnError> {
    let org = Organization::get_by_id(org_id).await?;
    Ok(org)
}

#[server]
async fn update_organization(
    _org_id: RecordId,
    update_data: Organization,
) -> Result<Organization, ServerFnError> {
    let updated = update_data.update_self().await?;
    Ok(updated)
}

#[component]
pub fn OrganizationEdit() -> impl IntoView {
    let params = use_params::<OrganizationParams>();
    let navigate = use_navigate();

    let org_id = move || {
        params
            .get()
            .and_then(|p| Ok(p.org_id.clone().unwrap_or_default()))
            .unwrap_or_default()
    };

    let org_resource = Resource::new(org_id.clone(), |id| async move {
        if id.is_empty() {
            Err(ServerFnError::new("Invalid organization ID"))
        } else {
            let id = RecordId::from_str(&id).unwrap();
            get_organization_for_edit(id).await
        }
    });

    let update_action = Action::new(|(org_id, update_data): &(RecordId, Organization)| {
        let org_id = org_id.clone();
        let update_data = update_data.clone();

        // let org_id: RecordId = RecordId::from_str(&org_id).unwrap();
        async move { update_organization(org_id, update_data).await }
    });

    let (is_submitting, set_is_submitting) = signal(false);

    Effect::new({
        let navigate = navigate.clone();
        let org_id = org_id.clone();
        move |_| {
            if let Some(Ok(_)) = update_action.value().get() {
                navigate(
                    &format!("/users/organizations/{}", org_id()),
                    Default::default(),
                );
            }
        }
    });

    Effect::new(move |_| {
        set_is_submitting.set(update_action.pending().get());
    });

    view! {
        <div class="max-w-2xl mx-auto px-4 py-8">
            <Suspense fallback=move || view! { <p>"Loading organization..."</p> }>
                {move || {
                    org_resource.get().map(|org_result| {
                        match org_result {
                            Ok(org) => {
                                let org_id = org.id.clone();

                                let org_id_for_submit = org_id.clone();
                                let org_id_for_cancel = org_id.clone();

                                view! {
                                    <div>
                                        <div class="mb-8">
                                            <h1 class="text-2xl font-bold text-neutral-900 dark:text-white mb-2">
                                                "Edit Organization"
                                            </h1>
                                            <p class="text-neutral-600 dark:text-neutral-400">
                                                "Update your organization's information"
                                            </p>
                                        </div>

                                        <div class="bg-white dark:bg-neutral-800 rounded-lg shadow dark:shadow-neutral-700 p-6">
                                            <OrganizationForm
                                                mode=OrganizationFormMode::Edit(org)
                                                on_submit=move |form_data| {
                                                    if let OrganizationFormData::Update(update_data) = form_data {
                                                        update_action.dispatch((org_id_for_submit.clone(), update_data));
                                                    }
                                                }
                                                on_cancel={
                                                    let navigate = navigate.clone();
                                                    move || navigate(&format!("/users/organizations/{}", org_id_for_cancel), Default::default())
                                                }
                                                is_submitting=is_submitting
                                                submit_button_text="Save Changes"
                                            />

                                            // Error display
                                            {move || {
                                                if let Some(Err(e)) = update_action.value().get() {
                                                    view! {
                                                        <div class="mt-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-md p-4">
                                                            <p class="text-red-800 dark:text-red-400">
                                                                "Error updating organization: " {format!("{:?}", e)}
                                                            </p>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! {}.into_any()
                                                }
                                            }}
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
    }
}
