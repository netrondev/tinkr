#[cfg(feature = "ssr")]
use crate::session::get_user;

use crate::organization::{
    organization::{CreateOrganization, Organization},
    ui_organization_form::{OrganizationForm, OrganizationFormData, OrganizationFormMode},
};

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn NewOrganizationForm() -> impl IntoView {
    let navigate = use_navigate();
    let create_org_action = Action::new(|org_data: &CreateOrganization| {
        let org_data = org_data.clone();
        async move { create_new_organization(org_data).await }
    });

    let (is_submitting, set_is_submitting) = signal(false);

    Effect::new({
        let navigate = navigate.clone();
        move |_| {
            if let Some(Ok(_org)) = create_org_action.value().get() {
                navigate("/users/organizations", Default::default());
            }
        }
    });

    Effect::new(move |_| {
        set_is_submitting.set(create_org_action.pending().get());
    });

    view! {
        <div class="max-w-2xl mx-auto px-4 py-8">
            <div class="mb-8">
                <h1 class="text-2xl font-bold text-neutral-900 dark:text-white mb-2">
                    "Create New Organization"
                </h1>
                <p class="text-neutral-600 dark:text-neutral-400">
                    "Set up a new organization to collaborate with your team"
                </p>
            </div>

            <div class="bg-white dark:bg-neutral-800 rounded-lg shadow dark:shadow-neutral-700 p-6">
                <OrganizationForm
                    mode=OrganizationFormMode::Create
                    on_submit=move |form_data| {
                        if let OrganizationFormData::Create(org_data) = form_data {
                            create_org_action.dispatch(org_data);
                        }
                    }
                    on_cancel={
                        let navigate = navigate.clone();
                        move || navigate("/users/organizations", Default::default())
                    }
                    is_submitting=is_submitting
                    submit_button_text="Create Organization"
                />

                // Error display
                {move || {
                    if let Some(Err(e)) = create_org_action.value().get() {
                        view! {
                            <div class="mt-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-md p-4">
                                <p class="text-red-800 dark:text-red-400">
                                    "Error creating organization: " {format!("{:?}", e)}
                                </p>
                            </div>
                        }
                            .into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
            </div>
        </div>
    }
}

#[server]
pub async fn create_new_organization(
    org_data: CreateOrganization,
) -> Result<Organization, leptos::server_fn::ServerFnError> {
    // Get the authenticated user
    let user = get_user().await?;

    // Create the organization with the current user as the creator
    let org = Organization::create(org_data, user.id.into()).await?;
    Ok(org)
}
