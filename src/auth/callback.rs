use leptos::{prelude::*, reactive::spawn_local};
use leptos_router::hooks::use_query_map;
use crate::EmailAddress;
use urlencoding::decode;

#[derive(Clone, Debug)]
enum AuthStatus {
    Loading,
    Success,
    Error(String),
}

#[component]
pub fn AuthCallback() -> impl IntoView {
    let query = use_query_map();
    let (auth_status, set_auth_status) = signal(AuthStatus::Loading);

    let params = move || {
        let token = query.get().get("token");
        let callbackurl = query.get().get("callbackUrl");
        (token, callbackurl)
    };

    Effect::new(move || {
        let (token, callback_url) = params();

        #[cfg(feature = "ssr")]
        // Log the parameters for debugging
        tracing::info!(
            "AuthCallback params - Token: {:?}, Callback URL: {:?}",
            token,
            callback_url
        );

        if let Some(token) = token {
            let set_auth_status = set_auth_status.clone();
            spawn_local(async move {
                let result = verify_token_callback_get_session_token(token).await;

                match result {
                    Ok(_) => {
                        #[cfg(feature = "ssr")]
                        tracing::info!("Token verification successful");
                        set_auth_status.set(AuthStatus::Success);

                        // store_token(session_token.clone());
                        // set_cookie_session_token(Some(session_token.clone()));

                        // Redirect or update UI as needed
                        if let Some(url) = callback_url {
                            let url = match decode(url.as_str()) {
                                Ok(decoded) => decoded.to_string(),
                                Err(_e) => {
                                    #[cfg(feature = "ssr")]
                                    tracing::error!("Failed to decode callback URL: {}", _e);

                                    return;
                                }
                            };
                            // Perform redirect to callback URL
                            window().location().set_href(&url).unwrap();
                        }
                    }
                    Err(e) => {
                        #[cfg(feature = "ssr")]
                        tracing::error!("Token verification failed: {:?}", e);

                        set_auth_status
                            .set(AuthStatus::Error(format!("Authentication failed: {}", e)));
                    }
                }
            });
        } else {
            set_auth_status.set(AuthStatus::Error(
                "Missing required parameters: token or email".to_string(),
            ));
        }
    });

    view! {
        <div class="h-full flex items-center justify-center bg-neutral-50 dark:bg-neutral-900 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                {move || match auth_status.get() {
                    AuthStatus::Loading => {
                        view! {
                            <div class="bg-white dark:bg-neutral-800 rounded-lg shadow-lg p-8 text-center">
                                <div class="flex justify-center mb-4">
                                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
                                </div>
                                <p class="text-lg font-medium text-neutral-900 dark:text-neutral-100">
                                    "Processing authentication..."
                                </p>
                                <p class="mt-2 text-sm text-neutral-600 dark:text-neutral-400">
                                    "Please wait while we verify your credentials"
                                </p>
                            </div>
                        }
                            .into_any()
                    }
                    AuthStatus::Success => {
                        view! {
                            <div class="bg-white dark:bg-neutral-800 rounded-lg shadow-lg p-8 text-center">
                                <div class="flex justify-center mb-4">
                                    <div class="rounded-full bg-green-100 dark:bg-green-900/20 p-3">
                                        <svg
                                            class="h-8 w-8 text-green-600 dark:text-green-400"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M5 13l4 4L19 7"
                                            ></path>
                                        </svg>
                                    </div>
                                </div>
                                <p class="text-lg font-medium text-neutral-900 dark:text-neutral-100">
                                    "Authentication successful!"
                                </p>
                                <p class="mt-2 text-sm text-neutral-600 dark:text-neutral-400">"Redirecting..."</p>
                            </div>
                        }
                            .into_any()
                    }
                    AuthStatus::Error(error) => {
                        view! {
                            <div class="bg-white dark:bg-neutral-800 rounded-lg shadow-lg p-8">
                                <div class="flex justify-center mb-4">
                                    <div class="rounded-full bg-red-100 dark:bg-red-900/20 p-3">
                                        <svg
                                            class="h-8 w-8 text-red-600 dark:text-red-400"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M6 18L18 6M6 6l12 12"
                                            ></path>
                                        </svg>
                                    </div>
                                </div>
                                <div class="text-center">
                                    <p class="text-lg font-medium text-neutral-900 dark:text-neutral-100">
                                        "Authentication failed"
                                    </p>
                                    <p class="mt-2 text-sm text-red-600 dark:text-red-400 break-words">{error}</p>
                                    <p class="mt-4 text-sm text-neutral-600 dark:text-neutral-400">
                                        "Please try signing in again or contact support if the issue persists."
                                    </p>
                                </div>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </div>
        </div>
    }
}

#[server]
pub async fn verify_token_callback_get_session_token(
    token: String,
) -> Result<String, ServerFnError> {
    use http::header::HeaderValue;
    use leptos_axum::ResponseOptions;

    let verified = crate::token::VerificationToken::use_verification_token(token).await?;
    let user = crate::user::AdapterUser::get_user(verified.user_id).await?;

    if &verified.email == &user.email {
        user.set_verified_email().await?;
    } else {
        return Err(ServerFnError::ServerError(
            "Token email does not match user email".into(),
        ));
    }

    let session = user.new_session().await?;

    // Create the cookie
    let cookie = session.build_session_cookie();

    // Set the cookie via ResponseOptions
    if let Some(resp) = use_context::<ResponseOptions>() {
        resp.insert_header(
            axum::http::header::SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );
    }

    Ok(session.session_token)
}

#[component]
pub fn VerifyEmailCallback() -> impl IntoView {
    let query = use_query_map();
    let (verify_status, set_verify_status) = signal(AuthStatus::Loading);

    let token = move || query.get().get("token");

    Effect::new(move || {
        let token = token();

        #[cfg(feature = "ssr")]
        tracing::info!("VerifyEmailCallback - Token: {:?}", token);

        if let Some(token) = token {
            let set_verify_status = set_verify_status.clone();
            spawn_local(async move {
                let result = verify_email_with_token(token).await;

                match result {
                    Ok(_email) => {
                        #[cfg(feature = "ssr")]
                        tracing::info!("Email verification successful for: {}", _email);
                        set_verify_status.set(AuthStatus::Success);

                        // Redirect to settings page after 2 seconds
                        set_timeout(
                            move || {
                                window().location().set_href("/settings").unwrap();
                            },
                            std::time::Duration::from_secs(2),
                        );
                    }
                    Err(e) => {
                        #[cfg(feature = "ssr")]
                        tracing::error!("Email verification failed: {:?}", e);

                        set_verify_status
                            .set(AuthStatus::Error(format!("Verification failed: {}", e)));
                    }
                }
            });
        } else {
            set_verify_status.set(AuthStatus::Error("Missing verification token".to_string()));
        }
    });

    view! {
        <div class="h-full flex items-center justify-center bg-neutral-50 dark:bg-neutral-900 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                {move || match verify_status.get() {
                    AuthStatus::Loading => {
                        view! {
                            <div class="bg-white dark:bg-neutral-800 rounded-lg shadow-lg p-8 text-center">
                                <div class="flex justify-center mb-4">
                                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
                                </div>
                                <p class="text-lg font-medium text-neutral-900 dark:text-neutral-100">
                                    "Verifying your email..."
                                </p>
                                <p class="mt-2 text-sm text-neutral-600 dark:text-neutral-400">
                                    "Please wait while we confirm your email address"
                                </p>
                            </div>
                        }
                            .into_any()
                    }
                    AuthStatus::Success => {
                        view! {
                            <div class="bg-white dark:bg-neutral-800 rounded-lg shadow-lg p-8 text-center">
                                <div class="flex justify-center mb-4">
                                    <div class="rounded-full bg-green-100 dark:bg-green-900/20 p-3">
                                        <svg
                                            class="h-8 w-8 text-green-600 dark:text-green-400"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M5 13l4 4L19 7"
                                            ></path>
                                        </svg>
                                    </div>
                                </div>
                                <p class="text-lg font-medium text-neutral-900 dark:text-neutral-100">
                                    "Email verified successfully!"
                                </p>
                                <p class="mt-2 text-sm text-neutral-600 dark:text-neutral-400">
                                    "Redirecting to settings page..."
                                </p>
                            </div>
                        }
                            .into_any()
                    }
                    AuthStatus::Error(error) => {
                        view! {
                            <div class="bg-white dark:bg-neutral-800 rounded-lg shadow-lg p-8">
                                <div class="flex justify-center mb-4">
                                    <div class="rounded-full bg-red-100 dark:bg-red-900/20 p-3">
                                        <svg
                                            class="h-8 w-8 text-red-600 dark:text-red-400"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M6 18L18 6M6 6l12 12"
                                            ></path>
                                        </svg>
                                    </div>
                                </div>
                                <div class="text-center">
                                    <p class="text-lg font-medium text-neutral-900 dark:text-neutral-100">
                                        "Email verification failed"
                                    </p>
                                    <p class="mt-2 text-sm text-red-600 dark:text-red-400 break-words">{error}</p>
                                    <p class="mt-4 text-sm text-neutral-600 dark:text-neutral-400">
                                        "The verification link may have expired or is invalid. Please request a new verification email."
                                    </p>
                                    <a
                                        href="/settings"
                                        class="mt-4 inline-block text-sm text-blue-600 dark:text-blue-400 hover:underline"
                                    >
                                        "Go to Settings"
                                    </a>
                                </div>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </div>
        </div>
    }
}

#[server]
pub async fn verify_email_with_token(
    token: String,
) -> Result<EmailAddress, leptos::server_fn::ServerFnError> {
    let token = crate::token::VerificationToken::use_verification_token(token).await?;
    let user = crate::user::AdapterUser::get_user(token.user_id.clone()).await?;
    user.set_verified_email().await?;
    Ok(token.email)
}
