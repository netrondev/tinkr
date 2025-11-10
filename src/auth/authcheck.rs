use crate::components::loading::LoadingIndicator;
use crate::session::{get_session, get_user};
use leptos::prelude::*;

#[component]
pub fn AuthCheck<F, IV>(unauthed: F, children: ChildrenFn) -> impl IntoView
where
    F: Fn() -> IV + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    let user_resource = Resource::new(|| (), |_| get_session());

    view! {
        <Suspense fallback=|| {
            view! { <LoadingIndicator /> }
        }>
            {move || {
                match user_resource.get() {
                    Some(Ok(_)) => children().into_any(),
                    Some(Err(_)) => unauthed().into_any(),
                    None => view! { <LoadingIndicator /> }.into_any(),
                }
            }}
        </Suspense>
    }
}

#[component]
pub fn UnauthedMessage() -> impl IntoView {
    view! {
        <div class="min-h-screen w-full flex items-center justify-center">
            <div class="text-center p-6 bg-white dark:bg-neutral-900 rounded-lg shadow-md">
                <h2 class="text-2xl font-semibold text-neutral-800 dark:text-neutral-200 mb-4">
                    "Access Denied"
                </h2>
                <p class="text-neutral-600 dark:text-neutral-400 mb-6">
                    "You do not have permission to view this page."
                </p>
                <a href="/login" class="text-blue-500 hover:underline">
                    "Go to Login"
                </a>
            </div>
        </div>
    }
}

#[component]
pub fn AuthCheckAdmin(children: ChildrenFn) -> impl IntoView {
    let user_resource = Resource::new(|| (), |_| get_user());

    view! {
        <Suspense fallback=|| {
            view! { <LoadingIndicator /> }
        }>
            {move || {
                match user_resource.get() {
                    Some(Ok(user)) => {
                        match user.is_admin {
                            Some(true) => children().into_any(),
                            _ => UnauthedMessage().into_any(),
                        }
                    }
                    Some(Err(_)) => UnauthedMessage().into_any(),
                    None => view! { <LoadingIndicator /> }.into_any(),
                }
            }}
        </Suspense>
    }
}

#[component]
pub fn AuthCheckAdminCustom<F, IV>(unauthed: F, children: ChildrenFn) -> impl IntoView
where
    F: Fn() -> IV + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    let user_resource = Resource::new(|| (), |_| get_user());

    view! {
        <Suspense fallback=|| {
            view! { <LoadingIndicator /> }
        }>
            {move || {
                match user_resource.get() {
                    Some(Ok(user)) => {
                        match user.is_admin {
                            Some(true) => children().into_any(),
                            _ => unauthed().into_any(),
                        }
                    }
                    Some(Err(_)) => unauthed().into_any(),
                    None => view! { <LoadingIndicator /> }.into_any(),
                }
            }}
        </Suspense>
    }
}
