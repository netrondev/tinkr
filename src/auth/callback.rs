use leptos::{prelude::*, reactive::spawn_local};
use leptos_router::hooks::use_query_map;
use crate::EmailAddress;
use urlencoding::decode;
use crate::auth::oauth::OAuthProvider;

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

#[component]
pub fn OAuthCallback() -> impl IntoView {
    let query = use_query_map();
    let (auth_status, set_auth_status) = signal(AuthStatus::Loading);

    let params = move || {
        let code = query.get().get("code");
        let state = query.get().get("state");
        let error = query.get().get("error");
        (code, state, error)
    };

    Effect::new(move || {
        let (code, state, error) = params();

        if let Some(error) = error {
            set_auth_status.set(AuthStatus::Error(format!("OAuth error: {}", error)));
            return;
        }

        if let (Some(code), Some(state)) = (code, state) {
            let set_auth_status = set_auth_status.clone();
            spawn_local(async move {
                let result = handle_oauth_callback(code, state).await;

                match result {
                    Ok(callback_url) => {
                        set_auth_status.set(AuthStatus::Success);
                        // Redirect to callback URL
                        window().location().set_href(&callback_url).unwrap();
                    }
                    Err(e) => {
                        set_auth_status.set(AuthStatus::Error(format!("Authentication failed: {}", e)));
                    }
                }
            });
        } else {
            set_auth_status.set(AuthStatus::Error(
                "Missing required parameters: code or state".to_string(),
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
pub async fn handle_oauth_callback(
    code: String,
    state: String,
) -> Result<String, ServerFnError> {
    use crate::auth::oauth::{OAuthConfig, OAuthUserInfo};
    use crate::auth::session::{delete_oauth_state, get_oauth_state};
    use http::header::HeaderValue;
    use leptos_axum::ResponseOptions;
    use oauth2::{AuthorizationCode, PkceCodeVerifier, TokenResponse};

    // Retrieve and validate OAuth state
    let oauth_state = get_oauth_state(state.clone()).await?;

    // Delete the state to prevent reuse
    delete_oauth_state(state).await?;

    // Get the provider config
    let config = match oauth_state.provider {
        OAuthProvider::Github => OAuthConfig::github(),
        OAuthProvider::Google => OAuthConfig::google(),
        OAuthProvider::Discord => OAuthConfig::discord(),
    }
    .map_err(|e| ServerFnError::new(e))?;

    let client = config.build_client().map_err(|e| ServerFnError::new(e))?;

    // Exchange the code for a token using async HTTP client
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(PkceCodeVerifier::new(oauth_state.pkce_verifier))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to exchange code for token: {}", e)))?;

    // Fetch user info from the provider
    let user_info = fetch_user_info(&config, token_result.access_token().secret())
        .await?;

    // Create or get user
    let user = get_or_create_user_from_oauth(&user_info, &oauth_state.provider).await?;

    // Create session
    let session = user.new_session().await?;

    // Set session cookie
    let cookie = session.build_session_cookie();
    if let Some(resp) = use_context::<ResponseOptions>() {
        resp.insert_header(
            axum::http::header::SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );
    }

    Ok(oauth_state.callback_url)
}

#[cfg(feature = "ssr")]
async fn fetch_user_info(
    config: &crate::auth::oauth::OAuthConfig,
    access_token: &str,
) -> Result<crate::auth::oauth::OAuthUserInfo, ServerFnError> {
    use crate::auth::oauth::{OAuthProvider, OAuthUserInfo};

    let http_client = reqwest::Client::new();
    let response = http_client
        .get(&config.user_info_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "Tinkr-OAuth-Client")
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch user info: {}", e)))?;

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse user info: {}", e)))?;

    // Parse based on provider
    let user_info = match config.provider {
        OAuthProvider::Github => OAuthUserInfo {
            id: json["id"].as_i64().unwrap_or(0).to_string(),
            email: json["email"].as_str().map(|s| s.to_string()),
            name: json["login"].as_str().map(|s| s.to_string()),
            avatar: json["avatar_url"].as_str().map(|s| s.to_string()),
        },
        OAuthProvider::Google => OAuthUserInfo {
            id: json["id"].as_str().unwrap_or("").to_string(),
            email: json["email"].as_str().map(|s| s.to_string()),
            name: json["name"].as_str().map(|s| s.to_string()),
            avatar: json["picture"].as_str().map(|s| s.to_string()),
        },
        OAuthProvider::Discord => OAuthUserInfo {
            id: json["id"].as_str().unwrap_or("").to_string(),
            email: json["email"].as_str().map(|s| s.to_string()),
            name: json["username"].as_str().map(|s| s.to_string()),
            avatar: json["avatar"]
                .as_str()
                .map(|avatar| format!("https://cdn.discordapp.com/avatars/{}/{}.png", json["id"].as_str().unwrap_or(""), avatar)),
        },
    };

    Ok(user_info)
}

#[cfg(feature = "ssr")]
async fn get_or_create_user_from_oauth(
    user_info: &crate::auth::oauth::OAuthUserInfo,
    provider: &OAuthProvider,
) -> Result<crate::auth::user::AdapterUser, ServerFnError> {
    use crate::user::{AdapterUser, CreateUserData};
    use crate::theme::Theme;

    // Try to find existing user by OAuth provider ID
    let existing_user = AdapterUser::get_user_by_oauth_id(&user_info.id, provider).await;

    if let Ok(user) = existing_user {
        return Ok(user);
    }

    // If user has email, try to find by email
    if let Some(ref email) = user_info.email {
        let email_addr = crate::EmailAddress(email.clone());
        if let Ok(user) = AdapterUser::get_user_by_email(email_addr).await {
            // Link OAuth account to existing user
            AdapterUser::link_oauth_account(&user.id, &user_info.id, provider).await?;
            return Ok(user);
        }
    }

    // Create new user
    let username = user_info.name.clone().unwrap_or_else(|| format!("user_{}", &user_info.id[..8]));

    let user = AdapterUser::create_user(CreateUserData {
        email: crate::EmailAddress(user_info.email.clone().unwrap_or_default()),
        email_verified: Some(surrealdb::Datetime::from(chrono::Utc::now())), // OAuth providers verify emails
        image: user_info.avatar.clone(),
        name: username,
        theme: Theme::System,
        address1: None,
        address2: None,
        address3: None,
        postcode: None,
        phone: None,
        telephone: None,
        first_name: None,
        last_name: None,
    })
    .await?;

    // Link OAuth account
    AdapterUser::link_oauth_account(&user.id, &user_info.id, provider).await?;

    Ok(user)
}
