use crate::session::get_session;
use leptos::prelude::*;
use crate::components::NavigationBackButton;

#[component]
pub fn UserProfile() -> impl IntoView {
    let user_resource = Resource::new(|| (), |_| get_session());

    view! {
        <div>
            <div class="flex items-center space-x-3 mb-4">
                <NavigationBackButton />
                <h1 class="text-2xl font-bold">"User Info"</h1>
            </div>
            <Suspense fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {move || {
                    user_resource
                        .get()
                        .map(|res| match res {
                            Ok(name) => view! { <p>{name}</p> },
                            Err(_) => view! { <p>{"Error fetching user.".to_string()}</p> },
                        })
                }}
            </Suspense>
        </div>
    }
}
