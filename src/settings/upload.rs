use crate::{
    components::button::{BtnColor, BtnVariant, Button, ButtonIcon},
    session::get_user,
    user::AdapterUser,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use phosphor_leptos::UPLOAD_SIMPLE;
use wasm_bindgen::prelude::*;
use web_sys::{Event, File, FormData, HtmlInputElement, ProgressEvent, XmlHttpRequest};

#[server]
async fn update_user_avatar(
    image_url: String,
) -> Result<AdapterUser, leptos::server_fn::ServerFnError> {
    let user = get_user().await?;
    let updated_user = user.update_user_image(image_url).await?;
    Ok(updated_user)
}

#[component]
pub fn AvatarUpload(on_upload: impl Fn(String) + Clone + 'static) -> impl IntoView {
    let (uploading, set_uploading) = signal(false);
    let (progress, set_progress) = signal(0.0);
    let (error, set_error) = signal(Option::<String>::None);

    let file_input_ref = NodeRef::<leptos::html::Input>::new();

    let handle_file_select = move |ev: Event| {
        let target = ev.target().unwrap();
        let input = target.dyn_into::<HtmlInputElement>().unwrap();

        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                // Validate file type
                let file_type = file.type_();
                if !file_type.starts_with("image/") {
                    set_error.set(Some("Please select an image file".to_string()));
                    return;
                }

                // Validate file size (5MB max)
                if file.size() > 5.0 * 1024.0 * 1024.0 {
                    set_error.set(Some("File size must be less than 5MB".to_string()));
                    return;
                }

                // Upload file
                upload_file(
                    file,
                    set_uploading,
                    set_progress,
                    set_error,
                    on_upload.clone(),
                );
            }
        }
    };

    view! {
        <div>
            <input
                type="file"
                accept="image/*"
                class="hidden"
                node_ref=file_input_ref
                on:change=handle_file_select
            />

            <Button
                color=BtnColor::Neutral
                on_click=Callback::new(move |_| {
                    if let Some(input) = file_input_ref.get() {
                        input.click();
                    }
                })
                icon=ButtonIcon::Icon(UPLOAD_SIMPLE)
                variant=BtnVariant::CallToAction
                disabled=uploading.get()
            >
                {move || {
                    if uploading.get() {
                        format!("Uploading... {}%", (progress.get() * 100.0) as i32)
                    } else {
                        "Change Avatar".to_string()
                    }
                }}
            </Button>

            // Error display
            <Show when=move || error.get().is_some()>
                <p class="mt-2 text-sm text-red-600">{move || error.get().unwrap_or_default()}</p>
            </Show>
        </div>
    }
}

fn upload_file(
    file: File,
    set_uploading: WriteSignal<bool>,
    set_progress: WriteSignal<f64>,
    set_error: WriteSignal<Option<String>>,
    on_upload: impl Fn(String) + Clone + 'static,
) {
    set_uploading.set(true);
    set_progress.set(0.0);
    set_error.set(None);

    let xhr = XmlHttpRequest::new().expect("Failed to create XMLHttpRequest");

    // Set up progress tracking
    let progress_callback = {
        let set_progress = set_progress.clone();
        Closure::<dyn Fn(ProgressEvent)>::new(move |e: ProgressEvent| {
            if e.length_computable() {
                let percentage = e.loaded() as f64 / e.total() as f64;
                set_progress.set(percentage);
            }
        })
    };

    xhr.upload()
        .unwrap()
        .set_onprogress(Some(progress_callback.as_ref().unchecked_ref()));
    progress_callback.forget();

    // Set up completion handler
    let xhr_clone = xhr.clone();
    let onload_callback = {
        let set_uploading = set_uploading.clone();
        let set_error = set_error.clone();
        let on_upload = on_upload.clone();

        Closure::<dyn Fn()>::new(move || {
            set_uploading.set(false);

            if xhr_clone.status().unwrap() == 200 {
                if let Ok(response_text) = xhr_clone.response_text() {
                    if let Some(response) = response_text {
                        // Parse response to get uploaded file URL
                        #[derive(serde::Deserialize)]
                        struct UploadResponse {
                            path: String,
                        }

                        if let Ok(upload_response) =
                            serde_json::from_str::<UploadResponse>(&response)
                        {
                            let image_url = format!("/{}", upload_response.path);

                            // Update user avatar in database
                            let on_upload_clone = on_upload.clone();
                            let set_error_clone = set_error.clone();
                            spawn_local(async move {
                                match update_user_avatar(image_url.clone()).await {
                                    Ok(_) => {
                                        on_upload_clone(image_url);
                                    }
                                    Err(e) => {
                                        set_error_clone
                                            .set(Some(format!("Failed to update avatar: {}", e)));
                                    }
                                }
                            });
                        } else {
                            set_error.set(Some("Invalid response from server".to_string()));
                        }
                    }
                } else {
                    set_error.set(Some("Failed to get response".to_string()));
                }
            } else {
                set_error.set(Some(format!(
                    "Upload failed with status: {}",
                    xhr_clone.status().unwrap()
                )));
            }
        })
    };

    xhr.set_onload(Some(onload_callback.as_ref().unchecked_ref()));
    onload_callback.forget();

    // Set up error handler
    let onerror_callback = {
        let set_uploading = set_uploading.clone();
        let set_error = set_error.clone();

        Closure::<dyn Fn()>::new(move || {
            set_uploading.set(false);
            set_error.set(Some("Network error occurred during upload".to_string()));
        })
    };

    xhr.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    // Prepare and send request
    match xhr.open("POST", "/api/upload-avatar") {
        Ok(_) => {
            let form_data = FormData::new().expect("Failed to create FormData");
            match form_data.append_with_blob("file", &file) {
                Ok(_) => match xhr.send_with_opt_form_data(Some(&form_data)) {
                    Ok(_) => {}
                    Err(_) => {
                        set_uploading.set(false);
                        set_error.set(Some("Failed to send upload request".to_string()));
                    }
                },
                Err(_) => {
                    set_uploading.set(false);
                    set_error.set(Some("Failed to append file to form data".to_string()));
                }
            }
        }
        Err(_) => {
            set_uploading.set(false);
            set_error.set(Some("Failed to open upload connection".to_string()));
        }
    }
}
