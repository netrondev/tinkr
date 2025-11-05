use crate::{
    components::{
        SectionStyled,
        alert::{Alert, AlertSeverity},
        heading::{Heading, SubHeading},
    },
    settings::{avatar_edit::AvatarSection, profile::ProfileSection},
};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::session::get_user;

#[component]
pub fn SettingsHome() -> impl IntoView {
    let _navigate = use_navigate();
    let user_resource = Resource::new(|| (), |_| get_user());

    view! {
        <div class="container mx-auto">
            <div class="">
                <div class="mb-8">
                    <Heading>"Settings"</Heading>
                    <SubHeading>"Manage your account settings and preferences"</SubHeading>
                </div>

                <Suspense fallback=move || {
                    view! {
                        <div class="bg-white shadow rounded-lg p-6">
                            <div class="animate-pulse">
                                <div class="h-4 bg-neutral-200 rounded w-1/4 mb-4"></div>
                                <div class="h-4 bg-neutral-200 rounded w-1/2"></div>
                            </div>
                        </div>
                    }
                }>
                    {move || {
                        user_resource
                            .get()
                            .map(|user| match user {
                                Ok(user) => {
                                    let usera = user.clone();
                                    let userb = user.clone();
                                    view! {
                                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
                                            <SectionStyled>
                                                <div class="flex flex-col gap-5">
                                                    <Heading>"Account"</Heading>
                                                    <AvatarSection
                                                        user=userb.clone()
                                                        on_update=move || user_resource.refetch()
                                                    />
                                                    <ProfileSection user=usera.clone() />
                                                </div>
                                            </SectionStyled>

                                            <SectionStyled>
                                                <div class="flex flex-col gap-5">
                                                    <Heading>"Delivery Details"</Heading>
                                                    <crate::auth::account_details::AccountForm />
                                                </div>
                                            </SectionStyled>
                                        </div>
                                    }
                                        .into_any()
                                }
                                Err(e) => {
                                    view! {
                                        <Alert severity=AlertSeverity::Error>
                                            "Error loading user data: " {e.to_string()}
                                        </Alert>
                                    }
                                        .into_any()
                                }
                            })
                    }}
                </Suspense>
            </div>
        </div>
    }
}
