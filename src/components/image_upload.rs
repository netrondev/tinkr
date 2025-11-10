use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{Event, File, FormData, HtmlInputElement, ProgressEvent, XmlHttpRequest};

#[component]
pub fn ImageUpload(
    #[prop(optional)] button_text: Option<&'static str>,
    #[prop(optional)] upload_endpoint: Option<&'static str>,
    #[prop(optional)] current_image_url: Option<Signal<Option<String>>>,
    on_upload: impl Fn(String) + Clone + 'static,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let (uploading, set_uploading) = signal(false);
    let (progress, set_progress) = signal(0.0);
    let (error, set_error) = signal(Option::<String>::None);

    let file_input_ref = NodeRef::<leptos::html::Input>::new();
    let button_text = button_text.unwrap_or("Upload Image");
    let upload_endpoint = upload_endpoint.unwrap_or("/api/upload-image");

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
                    upload_endpoint,
                    set_uploading,
                    set_progress,
                    set_error,
                    on_upload.clone(),
                );
            }
        }
    };

    let default_button_class = "inline-flex items-center px-4 py-2 border border-neutral-300 dark:border-neutral-600 shadow-sm text-sm font-medium rounded-md text-neutral-700 dark:text-neutral-300 bg-white dark:bg-neutral-800 hover:bg-neutral-50 dark:hover:bg-neutral-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 dark:focus:ring-offset-neutral-800 disabled:opacity-50 disabled:cursor-not-allowed";
    let final_class = class.unwrap_or(default_button_class);

    view! {
        <div>
            <input
                type="file"
                accept="image/*"
                class="hidden"
                node_ref=file_input_ref
                on:change=handle_file_select
            />

            {move || {
                if let Some(current_url_signal) = current_image_url {
                    if let Some(url) = current_url_signal.get() {
                        view! {
                            <div class="mb-4">
                                <img
                                    src=url
                                    alt="Current image"
                                    class="h-32 w-32 object-cover rounded-lg"
                                />
                            </div>
                        }
                            .into_any()
                    } else {
                        view! {}.into_any()
                    }
                } else {
                    view! {}.into_any()
                }
            }}

            <button
                type="button"
                on:click=move |_| {
                    if let Some(input) = file_input_ref.get() {
                        input.click();
                    }
                }
                disabled=move || uploading.get()
                class=final_class
            >
                {move || {
                    if uploading.get() {
                        format!("Uploading... {}%", (progress.get() * 100.0) as i32)
                    } else {
                        button_text.to_string()
                    }
                }}
            </button>

            // Error display
            <Show when=move || error.get().is_some()>
                <p class="mt-2 text-sm text-red-600 dark:text-red-400">
                    {move || error.get().unwrap_or_default()}
                </p>
            </Show>
        </div>
    }
}

fn upload_file(
    file: File,
    endpoint: &'static str,
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
                            on_upload(image_url);
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
    match xhr.open("POST", endpoint) {
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
