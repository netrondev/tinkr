use leptos::prelude::*;

use partial_struct::Partial;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::user::AdapterUser;

#[cfg(feature = "ssr")]
use crate::db_init;

#[cfg(feature = "ssr")]
use crate::AppError;

#[cfg(feature = "ssr")]
use surrealdb::{Datetime, RecordId};

#[cfg(not(feature = "ssr"))]
use crate::{Datetime, RecordId};

#[derive(Debug, Clone, Serialize, Deserialize, Partial)]
#[partial("CreateSessionData", derive(Serialize, Deserialize), omit(id))]
#[partial("UpdateSessionData", derive(Serialize, Deserialize), omit(id, user_id))]
pub struct AdapterSession {
    pub id: RecordId,
    pub session_token: String,
    pub user_id: RecordId,
    pub expires: Datetime,
}

#[cfg(feature = "ssr")]
impl AdapterSession {
    pub async fn from_string(session_token: String) -> Result<AdapterSession, AppError> {
        let client = db_init().await?;

        let mut result = client
            .query("SELECT * FROM ONLY session WHERE session_token = $session_token LIMIT 1;")
            .bind(("session_token", session_token))
            .await?;

        let token: Option<AdapterSession> = result.take(0)?;

        match token {
            Some(session) => Ok(session),
            None => Err(AppError::AuthError("Session not found".into())),
        }
    }

    pub async fn create_session(
        session_data: CreateSessionData,
    ) -> Result<AdapterSession, AppError> {
        let client = db_init().await?;
        let result: Option<AdapterSession> = client.create("session").content(session_data).await?;
        let session: AdapterSession =
            result.ok_or_else(|| AppError::AuthError("Could not create session".into()))?;
        Ok(session)
    }

    pub fn build_session_cookie(&self) -> axum_extra::extract::cookie::Cookie<'_> {
        use axum_extra::extract::cookie::Cookie;
        use time::Duration;

        let cookie: Cookie<'_> = if !cfg!(debug_assertions) {
            // release mode
            Cookie::build(("session_token", self.session_token.clone()))
                .path("/")
                .secure(true) // use only over HTTPS
                .http_only(true) // JS can't read the cookie
                .same_site(leptos_use::SameSite::Strict)
                .max_age(Duration::days(60))
                .build()
        } else {
            // debug mode
            Cookie::build(("session_token", self.session_token.clone()))
                .path("/")
                .secure(false) // use only over HTTPS
                .http_only(true) // JS can't read the cookie
                .same_site(leptos_use::SameSite::Lax)
                .max_age(Duration::days(365))
                .build()
        };

        cookie
    }

    // pub async fn get_session_and_user(
    //     &self,
    //     session_token: String,
    // ) -> Result<Option<(AdapterSession, AdapterUser)>, AppError> {
    //     let client = db_init().await?;

    //     let session_get = client
    //         .query("SELECT * FROM ONLY session WHERE sessionToken = $sessionToken LIMIT 1;")
    //         .bind(("sessionToken", session_token))
    //         .await?;

    //     println!("Session Get Result: {:?}", session_get);

    //     Ok(None)
    // }

    pub async fn update_session(
        // &self,
        data: UpdateSessionData,
    ) -> Result<Option<AdapterSession>, AppError> {
        let client = db_init().await?;

        let result = client
            .query("UPDATE session SET expires = $expires WHERE session_token = $session_token;")
            .bind(("expires", data.expires))
            .bind(("session_token", data.session_token))
            .await?;

        println!("update_session: {:?}", result);

        Ok(None)
    }

    pub async fn delete_session(session_token: String) -> Result<Option<AdapterSession>, AppError> {
        let client = db_init().await?;

        let _ = client
            .query("DELETE ONLY session WHERE session_token = $session_token RETURN BEFORE;")
            .bind(("session_token", session_token))
            .await?;

        Ok(None)
    }
}

#[server]
pub async fn get_session() -> Result<String, ServerFnError> {
    use crate::user::AdapterUser;
    let cookie_jar = leptos_axum::extract::<axum_extra::extract::CookieJar>().await?;
    let csrf_cookie = cookie_jar
        .iter()
        .filter(|cookie| cookie.name().contains("session_token"))
        .next()
        .ok_or(ServerFnError::new("Not logged in."))?;
    let user = AdapterUser::get_user_from_session(csrf_cookie.value().to_string()).await?;
    Ok(user.name)
}

#[server]
pub async fn get_user() -> Result<crate::user::AdapterUser, ServerFnError> {
    let user = get_user_option()
        .await?
        .ok_or(ServerFnError::new("Not logged in."))?;

    Ok(user)
}

/// Same as `get_user()` but wont error if no user
#[server]
pub async fn get_user_option() -> Result<Option<crate::user::AdapterUser>, ServerFnError> {
    let cookie_jar = leptos_axum::extract::<axum_extra::extract::CookieJar>().await?;

    let csrf_cookie = cookie_jar
        .iter()
        .filter(|cookie| cookie.name().contains("session_token"))
        .next();

    match csrf_cookie {
        Some(cookie) => {
            let user_from_cooki =
                AdapterUser::get_user_from_session(cookie.value().to_string()).await;

            let user = match user_from_cooki {
                Ok(user) => user,
                Err(_) => return Ok(None), // treat invalid session as no user
            };

            Ok(Some(user))
        }
        None => Ok(None),
    }
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use axum_extra::extract::cookie::Cookie;
    use http::header::HeaderValue;
    use leptos_axum::ResponseOptions;
    use time::Duration;

    let cookie_jar = leptos_axum::extract::<axum_extra::extract::CookieJar>().await?;

    // Find and delete the session from database
    if let Some(session_cookie) = cookie_jar
        .iter()
        .find(|cookie| cookie.name().contains("session_token"))
    {
        let _ = AdapterSession::delete_session(session_cookie.value().to_string()).await;
    }

    // Create the cookie to overwrite the existing session token
    // This will effectively clear the session on the client side
    // by setting an empty session token with a past expiration date
    let cookie = Cookie::build(("session_token", "".to_string()))
        .path("/")
        .secure(false) // use only over HTTPS
        .http_only(true) // JS can't read the cookie
        .same_site(leptos_use::SameSite::Lax)
        .max_age(Duration::MICROSECOND)
        .expires(time::OffsetDateTime::now_utc() - time::Duration::days(1))
        .build();

    // Set the cookie via ResponseOptions
    if let Some(resp) = use_context::<ResponseOptions>() {
        resp.insert_header(
            axum::http::header::SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );
    }

    // The cookie will be cleared on the client side after redirect
    Ok(())
}

#[component]
pub fn LogoutPage() -> impl IntoView {
    let logout_action = ServerAction::<Logout>::new();
    let (logout_triggered, set_logout_triggered) = signal(false);

    // Trigger logout immediately when component mounts
    Effect::new(move |_| {
        if !logout_triggered.get() && logout_action.value().get().is_none() {
            set_logout_triggered.set(true);
            logout_action.dispatch(Logout {});
        }
    });

    // Handle logout result
    Effect::new(move |_| {
        if let Some(Ok(_)) = logout_action.value().get() {
            // Redirect to home page
            #[cfg(not(feature = "ssr"))]
            {
                use web_sys::window;
                if let Some(window) = window() {
                    let _ = window.location().set_href("/");
                }
            }
            #[cfg(feature = "ssr")]
            {
                // On server, we can't redirect via JS, so we'll rely on client-side redirect
            }
        }
    });

    view! {
        <div class="flex items-center justify-center min-h-screen">
            <div class="text-center">
                <h2 class="text-xl font-semibold mb-2">"Logging out..."</h2>
                <p class="text-neutral-600 dark:text-neutral-400">"You will be redirected shortly."</p>
            </div>
        </div>
    }
}
