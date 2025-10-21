use crate::{
    components::{
        Button, FormField, Input, InputType,
        alert::{Alert, AlertSeverity},
        button::{BtnColor, BtnVariant, ButtonIcon},
    },
    user::{
        check_email_availability, check_username_availability, send_verification_email,
        update_user_profile,
    },
};
use leptos::prelude::*;

use crate::user::AdapterUser;

#[component]
pub fn ProfileSection(user: AdapterUser) -> impl IntoView {
    let original_name = std::sync::Arc::new(user.name.clone());
    let original_email = std::sync::Arc::new(user.email.0.clone());

    let name = RwSignal::new(user.name.clone());
    let email = RwSignal::new(user.email.0.clone());
    let loading = RwSignal::new(false);
    let (error, set_error) = signal(Option::<String>::None);
    let (success, set_success) = signal(false);

    // Username validation states
    let (checking_username, set_checking_username) = signal(false);
    let (username_available, set_username_available) = signal(Option::<bool>::None);
    let (username_error, set_username_error) = signal(Option::<String>::None);

    // Email validation states
    let (checking_email, set_checking_email) = signal(false);
    let (email_available, set_email_available) = signal(Option::<bool>::None);
    let (email_error, set_email_error) = signal(Option::<String>::None);
    let (email_format_valid, set_email_format_valid) = signal(true);

    // Email verification states
    let (sending_verification, set_sending_verification) = signal(false);
    let (verification_sent, set_verification_sent) = signal(false);
    let (verification_error, set_verification_error) = signal(Option::<String>::None);

    // Debounce timer for username check
    let original_name_clone = original_name.clone();
    let username_check = Action::new(move |username: &String| {
        let username = username.clone();
        let original = (*original_name_clone).clone();
        async move {
            // Skip check if it's the original username
            if username == original {
                set_username_available.set(Some(true));
                set_username_error.set(None);
                return;
            }

            if username.is_empty() {
                set_username_available.set(None);
                set_username_error.set(Some("Username cannot be empty".to_string()));
                return;
            }

            set_checking_username.set(true);
            set_username_error.set(None);

            match check_username_availability(username).await {
                Ok(available) => {
                    set_username_available.set(Some(available));
                    if !available {
                        set_username_error.set(Some("Username is already taken".to_string()));
                    }
                }
                Err(e) => {
                    set_username_error.set(Some(format!("Error checking username: {}", e)));
                    set_username_available.set(None);
                }
            }
            set_checking_username.set(false);
        }
    });

    // Debounce timer for email check
    let original_email_clone = original_email.clone();
    let email_check = Action::new(move |email_str: &String| {
        let email_str = email_str.clone();
        let original = (*original_email_clone).clone();
        async move {
            // Basic email format validation
            use crate::EmailAddress;
            use std::str::FromStr;

            if EmailAddress::from_str(&email_str).is_err() {
                set_email_format_valid.set(false);
                set_email_available.set(None);
                set_email_error.set(Some("Invalid email format".to_string()));
                return;
            }

            set_email_format_valid.set(true);

            // Skip check if it's the original email
            if email_str == original {
                set_email_available.set(Some(true));
                set_email_error.set(None);
                return;
            }

            set_checking_email.set(true);
            set_email_error.set(None);

            match check_email_availability(email_str).await {
                Ok(available) => {
                    set_email_available.set(Some(available));
                    if !available {
                        set_email_error.set(Some("Email is already in use".to_string()));
                    }
                }
                Err(e) => {
                    set_email_error.set(Some(format!("Error checking email: {}", e)));
                    set_email_available.set(None);
                }
            }
            set_checking_email.set(false);
        }
    });

    // Send verification email action
    let send_verification = Action::new(move |_| async move {
        set_sending_verification.set(true);
        set_verification_error.set(None);
        set_verification_sent.set(false);

        match send_verification_email().await {
            Ok(_) => {
                set_verification_sent.set(true);
            }
            Err(e) => {
                set_verification_error.set(Some(e.to_string()));
            }
        }
        set_sending_verification.set(false);
    });

    // Create effects for debounced validation
    Effect::new(move |_| {
        let current_name = name.get();
        // Use a timeout for debouncing
        set_timeout(
            move || {
                username_check.dispatch(current_name);
            },
            std::time::Duration::from_millis(500),
        );
    });

    Effect::new(move |_| {
        let current_email = email.get();
        // Use a timeout for debouncing
        set_timeout(
            move || {
                email_check.dispatch(current_email);
            },
            std::time::Duration::from_millis(500),
        );
    });

    let submit_action = Action::new(move |_| {
        let name_val = name.get();
        let email_val = email.get();

        async move {
            loading.set(true);
            set_error.set(None);
            set_success.set(false);

            match update_user_profile(name_val, email_val).await {
                Ok(_) => {
                    set_success.set(true);
                    loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e.to_string()));
                    loading.set(false);
                }
            }
        }
    });

    let original_name_clone2 = original_name.clone();
    let original_email_clone2 = original_email.clone();
    let submit_handler = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        // Check if validations pass
        let username_ok =
            username_available.get().unwrap_or(false) || name.get() == **original_name_clone2;
        let email_ok = (email_available.get().unwrap_or(false)
            || email.get() == **original_email_clone2)
            && email_format_valid.get();

        if username_ok && email_ok && !checking_username.get() && !checking_email.get() {
            submit_action.dispatch(());
        }
    };

    view! {
        <div class="">
            <form on:submit=submit_handler class="space-y-4">
                // Name field
                <FormField label="Name" label_for="name">
                    <div class="relative">
                        <Input
                            id="name"
                            r#type=InputType::Text
                            value=name
                            required=true
                        />
                        {
                            let original_name_view = original_name.clone();
                            move || {
                                if checking_username.get() {
                                    view! {
                                        <div class="absolute right-2 top-1/2 -translate-y-1/2">
                                            <div class="animate-spin h-4 w-4 border-2 border-neutral-300 border-t-neutral-600 rounded-full"></div>
                                        </div>
                                    }.into_any()
                                } else if let Some(available) = username_available.get() {
                                    if available && name.get() != **original_name_view {
                                    view! {
                                        <div class="absolute right-2 top-1/2 -translate-y-1/2">
                                            <svg class="h-5 w-5 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                                            </svg>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            } else {
                                view! { <div></div> }.into_any()
                            }
                        }}
                    </div>
                    {
                        let original_name_view2 = original_name.clone();
                        move || {
                            if let Some(error) = username_error.get() {
                                view! {
                                    <p class="mt-1 text-sm text-red-600">{error}</p>
                                }.into_any()
                            } else if username_available.get() == Some(true) && name.get() != **original_name_view2 {
                            view! {
                                <p class="mt-1 text-sm text-green-600">"Username is available"</p>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                </FormField>

                // Email field
                <FormField label="Email" label_for="email">
                    <div class="relative">
                        <Input
                            id="email"
                            r#type=InputType::Email
                            value=email
                            required=true
                        />
                        {
                            let original_email_view = original_email.clone();
                            move || {
                                if checking_email.get() {
                                    view! {
                                        <div class="absolute right-2 top-1/2 -translate-y-1/2">
                                            <div class="animate-spin h-4 w-4 border-2 border-neutral-300 border-t-neutral-600 rounded-full"></div>
                                        </div>
                                    }.into_any()
                                } else if let Some(available) = email_available.get() {
                                    if available && email.get() != **original_email_view && email_format_valid.get() {
                                    view! {
                                        <div class="absolute right-2 top-1/2 -translate-y-1/2">
                                            <svg class="h-5 w-5 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                                            </svg>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            } else {
                                view! { <div></div> }.into_any()
                            }
                        }}
                    </div>
                    {
                        let original_email_view2 = original_email.clone();
                        move || {
                            if let Some(error) = email_error.get() {
                                if error == "Email is already in use" {
                                    view! {
                                        <Alert severity=AlertSeverity::Error>
                                            {error}
                                        </Alert>
                                        <Button
                                            variant=BtnVariant::CallToAction
                                            color=BtnColor::Primary
                                        href="/login">"Login instead"</Button>

                                    }.into_any()
                                } else {
                                    view! {<span class="text-sm text-red-600">{error}</span>}.into_any()
                                }
                            } else if email_available.get() == Some(true) && email.get() != **original_email_view2 && email_format_valid.get() {
                            view! {
                                <p class="mt-1 text-sm text-green-600">"Email is available"</p>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}

                     {match user.email_verified {
                         Some(_) => view! {
                            <Alert severity=AlertSeverity::Success>
                                "Email verified"
                            </Alert>
                         }.into_any(),
                         None => view! {
                            <div class="space-y-2">
                                <Alert severity=AlertSeverity::Warning>
                                    "Email not verified"
                                </Alert>
                                <div class="flex items-center gap-2">
                                    <button
                                        type="button"
                                        on:click=move |_| { send_verification.dispatch(()); }
                                        disabled=sending_verification.get()
                                        class="text-sm px-3 py-1.5 bg-neutral-600 hover:bg-neutral-700 text-white rounded-md disabled:opacity-50 disabled:cursor-not-allowed"
                                    >
                                        {move || if sending_verification.get() { "Sending..." } else { "Send Verification Email" }}
                                    </button>
                                    {move || {
                                        if verification_sent.get() {
                                            view! {
                                                <span class="text-sm text-green-600">"Verification email sent!"</span>
                                            }.into_any()
                                        } else if let Some(error) = verification_error.get() {
                                            view! {
                                                <span class="text-sm text-red-600">{error}</span>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    }}
                                </div>
                            </div>
                         }.into_any()
                     }}
                </FormField>



                // Success message
                <Show when=move || success.get()>
                     <Alert severity=AlertSeverity::Success>
                        "Profile updated successfully"
                    </Alert>
                </Show>

                // Error message
                <Show when=move || error.get().is_some()>
                    <div class="rounded-md bg-red-50 p-4">
                        <div class="flex">
                            <div class="flex-shrink-0">
                                <svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
                                </svg>
                            </div>
                            <div class="ml-3">
                                <p class="text-sm font-medium text-red-800">
                                    {move || error.get().unwrap_or_default()}
                                </p>
                            </div>
                        </div>
                    </div>
                </Show>

                // Submit button
                <div class="flex justify-end">
                    <Button
                        button_type="submit"
                        disabled={
                            let is_loading = loading.get();
                            let is_checking = checking_username.get() || checking_email.get();
                            let username_invalid = username_error.get().is_some();
                            let email_invalid = email_error.get().is_some();
                            is_loading || is_checking || username_invalid || email_invalid
                        }
                        color=BtnColor::Primary
                        variant=BtnVariant::CallToAction
                        icon=ButtonIcon::Icon(phosphor_leptos::FLOPPY_DISK)
                    >
                        {move || {
                            if loading.get() {
                                "Saving..."
                            } else if checking_username.get() || checking_email.get() {
                                "Checking availability..."
                            } else {
                                "Save Changes"
                            }
                        }}
                    </Button>
                </div>

            </form>
        </div>
    }
}
