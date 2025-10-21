pub mod avatar_edit;
pub mod home;
pub mod profile;
pub mod upload;

use leptos::prelude::*;

use leptos_router::{
    components::{Route, Routes},
    path,
};

#[cfg(feature = "ssr")]
pub mod upload_ssr;

pub use home::SettingsHome;

use crate::{
    keys::KeysControl,
    organization::{
        organization_component_list::OrganizationList, ui_organization_new::NewOrganizationForm,
    },
};

#[component]
pub fn SettingsRouter() -> impl IntoView {
    view! {<div>

            <Routes fallback=|| "Page not found.".into_view()>
                <Route path=path!("/settings") view=SettingsHome />
                <Route
                    path=path!("/settings/keys")
                    view=KeysControl
                />
                <Route
                    path=path!("/settings/organizations")
                    view=OrganizationList
                />
                <Route
                    path=path!("/users/organizations/new")
                    view=NewOrganizationForm
                />
            </Routes>
        </div>
    }
}
