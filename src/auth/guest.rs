use crate::components::loading::LoadingIndicator;
use crate::{session::get_user_option, user::AdapterUser};
use leptos::prelude::*;

#[server]
pub async fn get_user_or_guest() -> Result<AdapterUser, ServerFnError> {
    let user = get_user_option().await?;

    match user {
        Some(user) => Ok(user),
        None => Ok(AdapterUser::new_guest().await?),
    }
}

#[server]
pub async fn ensure_guest_session() -> Result<AdapterUser, ServerFnError> {
    use http::header::HeaderValue;
    use leptos_axum::ResponseOptions;

    // Check if user is already logged in
    let user = get_user_option().await?;

    let guest_user = match user {
        Some(user) => user,
        None => {
            // No session exists, create a guest user
            let guest = AdapterUser::new_guest().await?;

            // Create a new session for the guest
            let session = guest.new_session().await?;

            // Set the session cookie
            let cookie = session.build_session_cookie();

            if let Some(resp) = use_context::<ResponseOptions>() {
                resp.insert_header(
                    axum::http::header::SET_COOKIE,
                    HeaderValue::from_str(&cookie.to_string()).unwrap(),
                );
            }

            guest
        }
    };

    Ok(guest_user)
}

#[component]
pub fn ForceUserAccount() -> impl IntoView {
    view! {
        <div class="p-4 bg-yellow-100 text-yellow-800 rounded-md border border-yellow-300">
            "You are currently browsing as a guest. Please "
            <a href="/login" class="text-blue-600 underline">
                "log in"
            </a> " or " <a href="/signup" class="text-blue-600 underline">
                "create an account"
            </a> " to access all features."
        </div>
    }
}

#[component]
pub fn ForcedGuest(children: ChildrenFn) -> impl IntoView {
    let user_resource = Resource::new(|| (), |_| get_user_option());
    let ensure_guest_action = ServerAction::<EnsureGuestSession>::new();
    let (guest_setup_triggered, set_guest_setup_triggered) = signal(false);
    let (needs_reload, set_needs_reload) = signal(false);

    // // Check if we need to set up a guest session
    Effect::new(move |_| {
        if let Some(Ok(None)) = user_resource.get() {
            // No user session exists, trigger guest creation
            if !guest_setup_triggered.get() && ensure_guest_action.value().get().is_none() {
                set_guest_setup_triggered.set(true);
                ensure_guest_action.dispatch(EnsureGuestSession {});
            }
        }
    });

    // // After guest session is created, trigger a reload to get the cookie
    Effect::new(move |_| {
        if let Some(Ok(_)) = ensure_guest_action.value().get() {
            if !needs_reload.get() {
                set_needs_reload.set(true);
                #[cfg(not(feature = "ssr"))]
                {
                    use web_sys::window;
                    if let Some(window) = window() {
                        let _ = window.location().reload();
                    }
                }
            }
        }
    });

    view! { {children()} }

    // view! {
    //     <Suspense fallback=|| {
    //         view! { <div class="p-2"><LoadingIndicator /></div> }
    //     }>
    //         {move || {
    //             match user_resource.get() {
    //                 Some(Ok(Some(_))) => children().into_any(),
    //                 _ => view! { <div class="p-2"><LoadingIndicator /></div> }.into_any(),
    //             }
    //         }}
    //         {children()}
    //     </Suspense>
    // }
}
