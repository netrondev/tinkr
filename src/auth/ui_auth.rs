use crate::EmailAddress;
use crate::components::{
    Seperator,
    input::{FormField, Input, InputType},
};
use crate::user::check_username_availability;
use leptos::{prelude::*, reactive::spawn_local};

use serde::{Deserialize, Serialize};

use crate::auth::oauth::OAuthProvider;
use crate::{metamask::WalletConnectButton, user::AdapterUser};

#[cfg(feature = "ssr")]
use crate::user::CreateUserData;

#[cfg(feature = "ssr")]
use crate::theme::Theme;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignInForm {
    pub email: EmailAddress,
    // add callback?
    pub callback_url: Option<String>,
}

#[server]
pub async fn get_users() -> Result<Vec<AdapterUser>, ServerFnError> {
    let user = crate::session::get_user().await?;

    match user.superadmin {
        Some(true) => {}
        _ => {
            return Err(ServerFnError::new("Unauthorized access"));
        }
    }

    let users = AdapterUser::get_all_users()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch users: {}", e)))?;

    Ok(users)
}

#[server]
pub async fn signin(input: SignInForm) -> Result<String, ServerFnError> {
    // validate email

    let user: AdapterUser =
        if let Ok(user) = AdapterUser::get_user_by_email(input.email.clone()).await {
            tracing::info!("signin get_user_by_email {:#?}", user);
            user
        } else {
            // create user if not exist
            tracing::info!("signin create_user {:#?}", input.clone());

            // Generate a unique username based on the email
            let base_name = input.email.0.split('@').next().unwrap_or("user");
            let mut username = base_name.to_string();

            let mut counter = 0;

            loop {
                let name_to_check = username.clone();

                // Check if this username is available
                let is_available = check_username_availability(name_to_check.clone())
                    .await
                    .unwrap_or(false);

                println!("is available {} for {}", is_available, name_to_check);

                if is_available {
                    username = name_to_check;
                    break;
                } else {
                    counter += 1;

                    username = format!("user_{}", counter);

                    if counter > 10 {
                        // Fallback to a UUID-based username if we can't find an available one
                        username = format!(
                            "user_{}",
                            uuid::Uuid::new_v4().to_string().split('-').next().unwrap()
                        );
                        break;
                    }
                }
            }

            AdapterUser::create_user(CreateUserData {
                email: input.email.clone(),
                email_verified: None,
                image: None,
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
            .await?
        };

    // generate token

    let token = user.new_verification_token().await?;

    if std::env::var("SERVICE_URL_DX").is_err() {
        tracing::warn!("SERVICE_URL_DX not set, using localhost:3000 as fallback");
    }

    let url = std::env::var("SERVICE_URL_DX").unwrap_or("http://localhost:3000".to_string());

    // generate link
    let link = format!(
        "{}/api/auth/callback/email?token={}&email={}&callbackUrl={}",
        url,
        token.token,
        urlencoding::encode(user.email.to_string().as_str()),
        urlencoding::encode(input.callback_url.unwrap_or_default().as_str())
    );

    let name = user.name;
    let subject = "Confirm your email to login";
    let message = format!(
        "Hi {name}! Please confirm your email to login! Click here: <a href={link}>CLICK TO CONFIRM</a>"
    );

    tracing::info!("signin message {:#?}", message);

    let _ = crate::email::send_email(user.email, subject, &message).await?;

    Ok("check your email".into())
}

#[tracing::instrument(name = "oauth_signin_internal")]
#[cfg(feature = "ssr")]
async fn oauth_signin_internal(
    provider: OAuthProvider,
    callback_url: Option<String>,
) -> Result<String, ServerFnError> {
    tracing::info!(
        "oauth_signin provider: {:?}, callback_url: {:?}",
        provider,
        callback_url,
    );
    use crate::auth::oauth::OAuthConfig;
    use oauth2::{CsrfToken, PkceCodeChallenge, Scope};

    let config = match provider {
        OAuthProvider::Github => OAuthConfig::github(),
        OAuthProvider::Google => OAuthConfig::google(),
        OAuthProvider::Discord => OAuthConfig::discord(),
    }
    .map_err(|e| ServerFnError::new(e))?;

    let client = config.build_client().map_err(|e| ServerFnError::new(e))?;

    // Generate PKCE challenge for added security (especially important for Google)
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate CSRF token
    let csrf_token = CsrfToken::new_random();

    // Build authorization URL with appropriate scopes
    let mut auth_request = client
        .authorize_url(|| csrf_token.clone())
        .set_pkce_challenge(pkce_challenge);

    // Add provider-specific scopes
    auth_request = match provider {
        OAuthProvider::Github => auth_request
            .add_scope(Scope::new("user:email".to_string()))
            .add_scope(Scope::new("read:user".to_string())),
        OAuthProvider::Google => auth_request
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string())),
        OAuthProvider::Discord => auth_request
            .add_scope(Scope::new("identify".to_string()))
            .add_scope(Scope::new("email".to_string())),
    };

    let (authorize_url, _csrf_state) = auth_request.url();

    // Store CSRF token and PKCE verifier in session for validation during callback
    // We'll use the session store for this
    use crate::auth::session::OAuthState;
    let state = OAuthState {
        csrf_token: csrf_token.secret().to_string(),
        pkce_verifier: pkce_verifier.secret().to_string(),
        callback_url: callback_url.unwrap_or_else(|| "/".to_string()),
        provider: provider.clone(),
    };

    crate::auth::session::store_oauth_state(state).await?;

    Ok(authorize_url.to_string())
}

#[server]
pub async fn oauth_signin(
    provider: OAuthProvider,
    callback_url: Option<String>,
) -> Result<String, ServerFnError> {
    tracing::info!(
        "oauth_signin server fn provider: {:?}, callback_url: {:?}",
        provider,
        callback_url,
    );

    let result = oauth_signin_internal(provider, callback_url).await;

    if result.is_err() {
        tracing::error!("oauth_signin error: {:?}", result);
    } else {
        tracing::info!("oauth_signin success");
    }

    result
}

#[component]
pub fn LoginForm() -> impl IntoView {
    let email_str = RwSignal::new(String::new());
    let (email, set_email) = signal(EmailAddress::default());
    let (is_loading, set_is_loading) = signal(false);
    let (error_message, set_error_message) = signal(Option::<String>::None);
    let (is_valid_email, set_is_valid_email) = signal(false);

    // if the user should check their email now
    let (check_email, set_check_email) = signal(false);

    // Update email and validate
    let on_email_input = move |value: String| {
        email_str.set(value.clone());
        let email_new_val = EmailAddress(value.clone());
        set_email.set(email_new_val.clone());
        set_is_valid_email.set(email_new_val.validate_email());
        if error_message.get().is_some() {
            set_error_message.set(None);
        }
    };

    // Handle form submission
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let email_value = email.get();

        if email_value.0.is_empty() {
            set_error_message.set(Some("Email is required".to_string()));
            return;
        }

        if !&email_value.validate_email() {
            set_error_message.set(Some("Please enter a valid email address".to_string()));
            return;
        }

        set_is_loading.set(true);
        set_error_message.set(None);

        spawn_local(async move {
            let _ = signin(SignInForm {
                email: email_value,
                callback_url: Some("/".to_string()),
            })
            .await;

            set_check_email.set(true);
            set_is_loading.set(false);
        });
    };

    // OAuth signin handlers
    let on_github_signin = move |_| {
        spawn_local(async move {
            match oauth_signin(OAuthProvider::Github, Some("/".to_string())).await {
                Ok(url) => {
                    window().location().set_href(&url).unwrap();
                }
                Err(e) => {
                    #[cfg(feature = "ssr")]
                    tracing::error!("GitHub OAuth error: {:?}", e);
                }
            }
        });
    };

    let on_google_signin = move |_| {
        spawn_local(async move {
            match oauth_signin(OAuthProvider::Google, Some("/".to_string())).await {
                Ok(url) => {
                    window().location().set_href(&url).unwrap();
                }
                Err(e) => {
                    #[cfg(feature = "ssr")]
                    tracing::error!("Google OAuth error: {:?}", e);
                }
            }
        });
    };

    let on_discord_signin = move |_| {
        spawn_local(async move {
            match oauth_signin(OAuthProvider::Discord, Some("/".to_string())).await {
                Ok(url) => {
                    window().location().set_href(&url).unwrap();
                }
                Err(e) => {
                    #[cfg(feature = "ssr")]
                    tracing::error!("Discord OAuth error: {:?}", e);
                }
            }
        });
    };

    view! {
        <div class="py-20 pb-[300px]">

            <div class="bg-white dark:bg-black rounded-lg shadow-2xl p-8 mx-8 w-full max-w-md mx-auto">
                <div class="text-center mb-8">
                    <h1 class="text-3xl font-bold text-neutral-800 dark:text-neutral-100 mb-2">"Authentication"</h1>
                    <small class="text-neutral-500 dark:text-neutral-400">"Creates a new account or logs you back in."</small>
                </div>

                <WalletConnectButton />

                <Seperator />

                // OAuth buttons
                <div class="flex flex-col gap-3 mb-6">
                    <button
                        type="button"
                        on:click=on_github_signin
                        class="w-full flex items-center justify-center gap-3 bg-neutral-800 hover:bg-neutral-700 dark:bg-neutral-700 dark:hover:bg-neutral-600 text-white px-4 py-3 rounded-md font-semibold duration-150"
                    >
                        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                        </svg>
                        "Continue with GitHub"
                    </button>

                    <button
                        type="button"
                        on:click=on_google_signin
                        class="w-full flex items-center justify-center gap-3 bg-white hover:bg-neutral-50 dark:bg-neutral-800 dark:hover:bg-neutral-700 text-neutral-800 dark:text-white border border-neutral-300 dark:border-neutral-600 px-4 py-3 rounded-md font-semibold duration-150"
                    >
                        <svg class="w-5 h-5" viewBox="0 0 24 24">
                            <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
                            <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
                            <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
                            <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
                        </svg>
                        "Continue with Google"
                    </button>

                    <button
                        type="button"
                        on:click=on_discord_signin
                        class="w-full flex items-center justify-center gap-3 bg-indigo-600 hover:bg-indigo-700 dark:bg-indigo-500 dark:hover:bg-indigo-600 text-white px-4 py-3 rounded-md font-semibold duration-150"
                    >
                        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028c.462-.63.874-1.295 1.226-1.994a.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10.2 10.2 0 0 0 .372-.292.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.839 19.839 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.946 2.418-2.157 2.418z"/>
                        </svg>
                        "Continue with Discord"
                    </button>
                </div>

                <Seperator />

                <form on:submit=on_submit class="flex flex-col">
                    <FormField
                        label="Email address"
                        label_for="email"
                        error=error_message.get().unwrap_or_default()
                        class="mb-6"
                    >
                        <Input
                            id="email"
                            name="email"
                            r#type=InputType::Email
                            placeholder="Enter your email"
                            value=email_str
                            on_input=Box::new(on_email_input)
                            disabled=is_loading.get()
                        />
                    </FormField>

                    <button
                        type="submit"
                        class="w-full text-neutral-100 bg-sky-600 hover:bg-sky-700 dark:bg-sky-600 dark:hover:bg-sky-500 px-4 py-3 rounded-md font-semibold duration-150 disabled:opacity-50 disabled:cursor-not-allowed"
                        disabled=move || is_loading.get() || !is_valid_email.get()
                    >
                        "LOGIN"
                    </button>

                    <Show when=move || check_email.get()>
                        <div class="text-center mt-4 text-neutral-600 dark:text-neutral-400">
                            "Check your email for the login link!"
                        </div>
                    </Show>

                </form>
            </div>
        </div>
    }
}
