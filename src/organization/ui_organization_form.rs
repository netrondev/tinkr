use super::organization::{CreateOrganization, Organization};
// use crate::components::form::{
//     FormActions, FormButton, FormField, FormGroup, Input, Label, Textarea,
// };
// use crate::components::image_upload::ImageUpload;

use leptos::ev;
use leptos::prelude::*;
use crate::components::form::FormActions;
use crate::components::form::FormButton;
use crate::components::form::FormField;
use crate::components::form::FormGroup;
use crate::components::form::Input;
use crate::components::form::Label;
use crate::components::form::Textarea;
use crate::components::image_upload::ImageUpload;

#[derive(Clone, Debug)]
pub enum OrganizationFormMode {
    Create,
    Edit(Organization),
}

#[derive(Clone)]
pub enum OrganizationFormData {
    Create(CreateOrganization),
    Update(Organization),
}

#[component]
pub fn OrganizationForm<F>(
    mode: OrganizationFormMode,
    on_submit: F,
    on_cancel: impl Fn() + 'static + Send,
    is_submitting: ReadSignal<bool>,
    submit_button_text: &'static str,
) -> impl IntoView
where
    F: Fn(OrganizationFormData) + 'static,
{
    // Initialize form fields based on mode
    let (initial_name, initial_description, initial_logo_url, initial_website) = match &mode {
        OrganizationFormMode::Create => {
            (String::new(), String::new(), String::new(), String::new())
        }
        OrganizationFormMode::Edit(org) => (
            org.name.clone(),
            org.description.clone().unwrap_or_default(),
            org.logo_url.clone().unwrap_or_default(),
            org.website.clone().unwrap_or_default(),
        ),
    };

    // Form state
    let name = RwSignal::new(initial_name);
    let description = RwSignal::new(initial_description);
    let logo_url = RwSignal::new(initial_logo_url);
    let website = RwSignal::new(initial_website);

    // Form validation
    let is_valid = move || !name.get().trim().is_empty();

    let handle_submit = move |e: ev::SubmitEvent| {
        e.prevent_default();

        if !is_valid() {
            return;
        }

        let form_data = match &mode {
            OrganizationFormMode::Create => OrganizationFormData::Create(CreateOrganization {
                name: name.get().trim().to_string(),
                description: if description.get().trim().is_empty() {
                    None
                } else {
                    Some(description.get().trim().to_string())
                },
                logo_url: if logo_url.get().trim().is_empty() {
                    None
                } else {
                    Some(logo_url.get().trim().to_string())
                },
                website: if website.get().trim().is_empty() {
                    None
                } else {
                    Some(website.get().trim().to_string())
                },
            }),
            OrganizationFormMode::Edit(org) => {
                let mut orgedit = org.clone();

                orgedit.name = name.get().trim().to_string();
                orgedit.description = if description.get().trim().is_empty() {
                    None
                } else {
                    Some(description.get().trim().to_string())
                };

                orgedit.logo_url = if logo_url.get().trim().is_empty() {
                    None
                } else {
                    Some(logo_url.get().trim().to_string())
                };

                orgedit.website = if website.get().trim().is_empty() {
                    None
                } else {
                    Some(website.get().trim().to_string())
                };

                let output = OrganizationFormData::Update(orgedit);

                output
            }
        };

        on_submit(form_data);
    };

    view! {
        <form on:submit=handle_submit>
            <FormGroup>
                // Organization Name
                <FormField>
                    <Label for_id="name" required=true>
                        "Organization Name"
                    </Label>
                    <Input
                        id="name"
                        r#type="text"
                        value=name.into()
                        on_input=move |value| name.set(value)
                        placeholder="Enter organization name"
                        required=true
                    />
                </FormField>

                // Description
                <FormField>
                    <Label for_id="description">
                        "Description"
                    </Label>
                    <Textarea
                        id="description"
                        value=description.into()
                        on_input=move |value| description.set(value)
                        placeholder="Brief description of your organization"
                        rows=3
                    />
                </FormField>

                // Logo
                <FormField>
                    <Label for_id="logo">
                        "Organization Logo"
                    </Label>





                    <ImageUpload
                        button_text="Upload Logo"
                        upload_endpoint="/api/upload-avatar"
                        current_image_url=Signal::derive(move || {
                            let url = logo_url.get();
                            if url.trim().is_empty() {
                                None
                            } else {
                                Some(url)
                            }
                        })
                        on_upload=move |url| logo_url.set(url)
                    />

                </FormField>

                // Website
                <FormField>
                    <Label for_id="website">
                        "Website"
                    </Label>
                    <Input
                        id="website"
                        r#type="url"
                        value=website.into()
                        on_input=move |value| website.set(value)
                        placeholder="https://example.com"
                    />
                </FormField>

                // Form Actions
                <FormActions>
                    <FormButton
                        r#type="submit"
                        variant="primary"
                        disabled=Signal::derive(move || is_submitting.get() || !is_valid())
                    >
                        {move || if is_submitting.get() { "Saving..." } else { submit_button_text }}
                    </FormButton>
                    <FormButton
                        variant="secondary"
                        on_click=Box::new(move |_: ev::MouseEvent| on_cancel())
                        disabled=is_submitting.into()
                    >
                        "Cancel"
                    </FormButton>
                </FormActions>
            </FormGroup>
        </form>
    }
}
