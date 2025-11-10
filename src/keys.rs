#[cfg(feature = "ssr")]
use crate::StorageAuthed;

#[cfg(not(feature = "ssr"))]
use crate::{Datetime, RecordId};

#[cfg(feature = "ssr")]
use crate::AppError;
use crate::components::{
    Button, Modal, ModalSize,
    button::{BtnColor, BtnVariant, ButtonIcon},
};
use partial_struct::Partial;
use phosphor_leptos::KEY;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use surrealdb::{Datetime, RecordId};

#[cfg(feature = "ssr")]
use crate::user::AdapterUser;

use leptos::prelude::*;

#[cfg(feature = "ssr")]
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize, Partial)]
#[partial(
    "KeyCreate",
    derive(Debug, Serialize, Deserialize, Clone),
    omit(id, created_by_user_id, created_at, updated_at, last_used)
)]

pub struct Key {
    pub id: RecordId,
    pub name: String,
    pub key_for: Option<RecordId>,
    pub key_public: Option<String>,
    pub key_private: Option<String>,
    pub key_apikey: Option<String>,
    pub key_token: Option<String>,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub created_by_user_id: RecordId,
    pub expires_at: Option<Datetime>,
    pub last_used: Option<Datetime>,
}

#[cfg(feature = "ssr")]
impl StorageAuthed<KeyCreate, Key> for Key {
    const TABLE_NAME: &'static str = "key";
}

#[cfg(feature = "ssr")]
impl Key {
    pub async fn get_user_keys_for(
        user: AdapterUser,
        key_for: RecordId,
    ) -> Result<Vec<Self>, AppError> {
        let user_keys = Self::get_by_user(user).await?;
        Ok(user_keys
            .into_iter()
            .filter(|key| match key.key_for {
                Some(ref kf) if kf == &key_for => true,
                None => false,
                _ => false,
            })
            .collect())
    }

    pub async fn get_user_firstkey_for(
        user: AdapterUser,
        key_for: RecordId,
    ) -> Result<Self, AppError> {
        let keys = Self::get_user_keys_for(user, key_for).await?;
        if keys.is_empty() {
            Err(AppError::NotFound("Key not found".to_string()))
        } else {
            Ok(keys[0].clone())
        }
    }
}

#[server]
pub async fn get_user_keys() -> Result<Vec<Key>, leptos::server_fn::ServerFnError> {
    let user = crate::session::get_user().await?;
    let keys = Key::get_by_user(user).await?;
    Ok(keys)
}

#[component]
pub fn KeyItem(key: Key) -> impl IntoView {
    let _key_id = key.id.clone();
    // let key_public_preview = if key.key_public.len() > 20 {
    //     format!("{}...", &key.key_public[..20])
    // } else {
    //     key.key_public.clone()
    // };

    let key_public_preview = "....".to_string(); // Placeholder for key public preview

    view! {
        <div class="px-4 py-3 hover:bg-neutral-50 dark:hover:bg-neutral-700">
            <div class="flex items-center justify-between">
                <div class="flex-1">
                    <div class="flex items-center gap-2">
                        <h3 class="text-sm font-medium text-neutral-900 dark:text-neutral-100">
                            {key.name.clone()}
                        </h3>
                        {key
                            .expires_at
                            .is_some()
                            .then(|| {
                                view! {
                                    <span class="text-xs text-yellow-600 dark:text-yellow-400">
                                        "(expires)"
                                    </span>
                                }
                            })}
                    </div>
                    <p class="text-xs text-neutral-500 dark:text-neutral-400 mt-1">
                        {key.description.clone()}
                    </p>
                    <div class="flex items-center gap-4 mt-2 text-xs text-neutral-600 dark:text-neutral-400">
                        <span class="font-mono">{key_public_preview}</span>
                        <span>"Created: "{key.created_at.clone()}</span>
                        {key
                            .last_used
                            .is_some()
                            .then(|| {
                                view! { <span>"Last used: recently"</span> }
                            })}
                    </div>
                </div>
                <div class="flex items-center gap-2">
                    <button class="text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300 text-sm font-medium">
                        "Delete"
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn KeyList() -> impl IntoView {
    let keys_resource = Resource::new(|| (), |_| get_user_keys());

    view! {
        <Suspense fallback=move || {
            view! {
                <div class="bg-white dark:bg-neutral-800 rounded-lg shadow">
                    <div class="p-6">
                        <div class="animate-pulse space-y-4">
                            <div class="h-4 bg-neutral-200 dark:bg-neutral-700 rounded w-3/4"></div>
                            <div class="h-4 bg-neutral-200 dark:bg-neutral-700 rounded w-1/2"></div>
                            <div class="h-4 bg-neutral-200 dark:bg-neutral-700 rounded w-5/6"></div>
                        </div>
                    </div>
                </div>
            }
        }>
            {move || {
                match keys_resource.get() {
                    Some(Ok(keys)) => {
                        if keys.is_empty() {
                            view! {
                                <div class="bg-white dark:bg-neutral-800 rounded-lg shadow">
                                    <div class="p-6">
                                        <p class="text-neutral-500 dark:text-neutral-400">
                                            "No keys found. Create your first key to get started."
                                        </p>
                                    </div>
                                </div>
                            }
                                .into_any()
                        } else {
                            view! {
                                <div class="bg-white dark:bg-neutral-800 rounded-lg shadow overflow-hidden">
                                    <div class="divide-y divide-neutral-200 dark:divide-neutral-700">
                                        {keys
                                            .into_iter()
                                            .map(|key| {
                                                view! { <KeyItem key=key /> }
                                            })
                                            .collect_view()}
                                    </div>
                                </div>
                            }
                                .into_any()
                        }
                    }
                    Some(Err(e)) => {
                        view! {
                            <div class="bg-white dark:bg-neutral-800 rounded-lg shadow">
                                <div class="p-6">
                                    <div class="text-red-600 dark:text-red-400">
                                        <p>"Error loading keys: " {format!("{:?}", e)}</p>
                                        <button
                                            class="mt-2 text-sm text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
                                            on:click=move |_| keys_resource.refetch()
                                        >
                                            "Retry"
                                        </button>
                                    </div>
                                </div>
                            </div>
                        }
                            .into_any()
                    }
                    None => view! { <div></div> }.into_any(),
                }
            }}
        </Suspense>
    }
}

#[component]
pub fn KeysControl() -> impl IntoView {
    view! {
        <div class="p-4 bg-white dark:bg-neutral-800 rounded-lg shadow">
            <h2 class="text-xl font-semibold mb-4">"Manage Keys"</h2>
            <p class="text-neutral-600 dark:text-neutral-400 mb-6">
                "Here you can manage your API keys."
            </p>
            <KeyList />
        </div>
    }
}

#[server]
pub async fn create_user_key(
    key_create: KeyCreate,
    key_for: Option<RecordId>,
) -> Result<Key, leptos::server_fn::ServerFnError> {
    use base64::{Engine as _, engine::general_purpose};
    use rand::Rng;

    let user = crate::session::get_user().await?;

    // Generate random bytes for keys - do this before any await
    let (key_private, key_public) = {
        let mut rng = rand::rng();
        let private_bytes: [u8; 32] = rng.random();
        let public_bytes: [u8; 32] = rng.random();

        // Encode to base64
        let key_private = general_purpose::STANDARD.encode(&private_bytes);
        let key_public = general_purpose::STANDARD.encode(&public_bytes);
        (key_private, key_public)
    };

    // Create the key with generated values
    let mut key_data = key_create;
    key_data.key_public = Some(key_public);
    key_data.key_private = Some(key_private);
    key_data.key_for = key_for;

    let created_key = Key::create_by_user(user, key_data).await?;
    Ok(created_key)
}

#[component]
pub fn KeysAdd(
    #[prop(optional)] key_for: Option<RecordId>,
    #[prop(optional)] on_success: Option<Callback<Key, ()>>,
    #[prop(optional)] require_apikey: bool,
    #[prop(optional)] require_token: bool,
    #[prop(optional)] require_public: bool,
    #[prop(optional)] require_private: bool,
) -> impl IntoView {
    let (show_modal, set_show_modal) = signal(false);
    let (name, set_name) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let (expires_in_days, set_expires_in_days) = signal(String::new());
    let (apikey, set_apikey) = signal(String::new());
    let (token, set_token) = signal(String::new());
    let (is_creating, set_is_creating) = signal(false);
    let (error_message, set_error_message) = signal(Option::<String>::None);

    let create_action = Action::new(move |_: &()| {
        let name = name.get();
        let description = description.get();
        let expires_in_days = expires_in_days.get();
        let apikey = apikey.get();
        let token = token.get();
        let key_for = key_for.clone();

        async move {
            if name.is_empty() {
                return Err("Key name is required".to_string());
            }

            let expires_at = if !expires_in_days.is_empty() {
                match expires_in_days.parse::<i64>() {
                    Ok(days) if days > 0 => {
                        #[cfg(feature = "ssr")]
                        {
                            let expiry = Utc::now() + chrono::Duration::days(days);
                            Some(surrealdb::Datetime::from(expiry))
                        }
                        #[cfg(not(feature = "ssr"))]
                        {
                            None
                        }
                    }
                    _ => return Err("Invalid expiration days".to_string()),
                }
            } else {
                None
            };

            let key_create = KeyCreate {
                name,
                description,
                key_for: None,
                key_public: None,
                key_private: None,
                key_apikey: if apikey.is_empty() {
                    None
                } else {
                    Some(apikey)
                },
                key_token: if token.is_empty() { None } else { Some(token) },
                expires_at,
            };

            create_user_key(key_create, key_for)
                .await
                .map_err(|e| e.to_string())
        }
    });

    Effect::new(move || {
        if let Some(result) = create_action.value().get() {
            match result {
                Ok(key) => {
                    // Clear form
                    set_name.set(String::new());
                    set_description.set(String::new());
                    set_expires_in_days.set(String::new());
                    set_apikey.set(String::new());
                    set_token.set(String::new());
                    set_error_message.set(None);
                    set_is_creating.set(false);
                    set_show_modal.set(false);

                    // Call success callback if provided
                    if let Some(callback) = on_success {
                        callback.run(key);
                    }
                }
                Err(e) => {
                    set_error_message.set(Some(e));
                    set_is_creating.set(false);
                }
            }
        }
    });

    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_is_creating.set(true);
        set_error_message.set(None);
        create_action.dispatch(());
    };

    let close_modal = Callback::new(move |_| {
        set_show_modal.set(false);
        // Clear form on close
        set_name.set(String::new());
        set_description.set(String::new());
        set_expires_in_days.set(String::new());
        set_apikey.set(String::new());
        set_token.set(String::new());
        set_error_message.set(None);
    });

    // Determine which fields to show
    let show_apikey = require_apikey;
    let show_token = require_token;
    let show_public_private = require_public || require_private;

    view! {
        <>
            <Button
                on_click=Callback::new(move |_| set_show_modal.set(true))
                variant=BtnVariant::CallToAction
                color=BtnColor::Primary
                icon=ButtonIcon::Icon(KEY)
            >
                "Create New Key"
            </Button>

            <Modal
                show=show_modal.into()
                on_close=close_modal
                title="Create New Key".to_string()
                size=ModalSize::Medium
            >
                <form on:submit=submit>
                    <div class="space-y-4">
                        <div>
                            <label
                                for="key-name"
                                class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1"
                            >
                                "Name"
                                <span class="text-red-500">"*"</span>
                            </label>
                            <input
                                type="text"
                                id="key-name"
                                class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 bg-white dark:bg-neutral-700 text-neutral-900 dark:text-white rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                prop:value=move || name.get()
                                on:input=move |e| set_name.set(event_target_value(&e))
                                required
                                disabled=move || is_creating.get()
                            />
                        </div>

                        <div>
                            <label
                                for="key-description"
                                class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1"
                            >
                                "Description"
                            </label>
                            <textarea
                                id="key-description"
                                rows=3
                                class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 bg-white dark:bg-neutral-700 text-neutral-900 dark:text-white rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                prop:value=move || description.get()
                                on:input=move |e| set_description.set(event_target_value(&e))
                                disabled=move || is_creating.get()
                            />
                        </div>

                        {move || {
                            if show_apikey {
                                view! {
                                    <div>
                                        <label
                                            for="key-apikey"
                                            class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1"
                                        >
                                            "API Key"
                                        </label>
                                        <input
                                            type="text"
                                            id="key-apikey"
                                            class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 bg-white dark:bg-neutral-700 text-neutral-900 dark:text-white rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                            placeholder="Enter API key"
                                            prop:value=move || apikey.get()
                                            on:input=move |e| set_apikey.set(event_target_value(&e))
                                            disabled=move || is_creating.get()
                                        />
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! { <></> }.into_any()
                            }
                        }}

                        {move || {
                            if show_token {
                                view! {
                                    <div>
                                        <label
                                            for="key-token"
                                            class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1"
                                        >
                                            "Token"
                                        </label>
                                        <input
                                            type="text"
                                            id="key-token"
                                            class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 bg-white dark:bg-neutral-700 text-neutral-900 dark:text-white rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                            placeholder="Enter token"
                                            prop:value=move || token.get()
                                            on:input=move |e| set_token.set(event_target_value(&e))
                                            disabled=move || is_creating.get()
                                        />
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! { <></> }.into_any()
                            }
                        }}

                        {move || {
                            if show_public_private {
                                view! {
                                    <div class="p-3 bg-neutral-100 dark:bg-neutral-700 rounded-md">
                                        <p class="text-sm text-neutral-600 dark:text-neutral-400">
                                            "Public and private keys will be automatically generated when you create this key."
                                        </p>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! { <></> }.into_any()
                            }
                        }}

                        // <div>
                        // <label for="key-expires" class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">
                        // "Expires in (days)"
                        // </label>
                        // <input
                        // type="number"
                        // id="key-expires"
                        // min="1"
                        // class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 bg-white dark:bg-neutral-700 text-neutral-900 dark:text-white rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                        // placeholder="Leave empty for no expiration"
                        // prop:value=move || expires_in_days.get()
                        // on:input=move |e| set_expires_in_days.set(event_target_value(&e))
                        // disabled=move || is_creating.get()
                        // />
                        // </div>

                        {move || {
                            error_message
                                .get()
                                .map(|msg| {
                                    view! {
                                        <div class="text-red-600 dark:text-red-400 text-sm">
                                            {msg}
                                        </div>
                                    }
                                })
                        }}

                        <div class="flex gap-4 pt-2">
                            <button
                                type="submit"
                                class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-neutral-800 disabled:opacity-50 disabled:cursor-not-allowed"
                                disabled=move || is_creating.get()
                            >
                                {move || {
                                    if is_creating.get() { "Creating..." } else { "Create Key" }
                                }}
                            </button>
                            <button
                                type="button"
                                class="px-4 py-2 bg-neutral-200 dark:bg-neutral-700 text-neutral-700 dark:text-neutral-300 rounded-md hover:bg-neutral-300 dark:hover:bg-neutral-600 focus:outline-none focus:ring-2 focus:ring-neutral-500 focus:ring-offset-2 dark:focus:ring-offset-neutral-800"
                                on:click=move |_| close_modal.run(())
                                disabled=move || is_creating.get()
                            >
                                "Cancel"
                            </button>
                        </div>
                    </div>
                </form>
            </Modal>
        </>
    }
}
