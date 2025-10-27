use crate::EmailAddress;
use crate::components::{
    Seperator,
    input::{FormField, Input, InputType},
};
use crate::user::check_username_availability;
use leptos::{prelude::*, reactive::spawn_local};

use serde::{Deserialize, Serialize};

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

    view! {
        <div class="py-20 pb-[300px]">

            <div class="bg-white dark:bg-black rounded-lg shadow-2xl p-8 mx-8 w-full max-w-md mx-auto">
                <div class="text-center mb-8">
                    <h1 class="text-3xl font-bold text-neutral-800 dark:text-neutral-100 mb-2">"Authentication"</h1>
                    <small class="text-neutral-500 dark:text-neutral-400">"Creates a new account or logs you back in."</small>
                </div>

                <WalletConnectButton />

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
