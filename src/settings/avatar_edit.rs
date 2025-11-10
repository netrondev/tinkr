use crate::user::AdapterUser;
use leptos::prelude::*;

#[component]
pub fn AvatarSection(user: AdapterUser, on_update: impl Fn() + Clone + 'static) -> impl IntoView {
    let (_uploading, _set_uploading) = signal(false);
    let (error, _set_error) = signal(Option::<String>::None);

    view! {
        <div class="">
            <div class="">
                <div class="flex items-center">
                    <div class="flex-shrink-0">
                        <img
                            class="h-24 w-24 rounded-full object-cover bg-neutral-300"
                            src=user
                                .image
                                .clone()
                                .unwrap_or_else(|| {
                                    "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='%23999'%3E%3Cpath d='M24 20.993V24H0v-2.996A14.977 14.977 0 0112.004 15c4.904 0 9.26 2.354 11.996 5.993zM16.002 8.999a4 4 0 11-8 0 4 4 0 018 0z'/%3E%3C/svg%3E"
                                        .to_string()
                                })
                            alt="Profile"
                        />
                    </div>
                    <div class="ml-6">
                        <super::upload::AvatarUpload on_upload={
                            let on_update = on_update.clone();
                            move |_image_url| {
                                on_update();
                            }
                        } />
                        <p class="mt-2 text-sm text-neutral-500">
                            "JPG, GIF or PNG. Max size of 5MB."
                        </p>
                    </div>
                </div>

                // Error message
                <Show when=move || error.get().is_some()>
                    <div class="mt-4 rounded-md bg-red-50 p-4">
                        <div class="flex">
                            <div class="flex-shrink-0">
                                <svg
                                    class="h-5 w-5 text-red-400"
                                    viewBox="0 0 20 20"
                                    fill="currentColor"
                                >
                                    <path
                                        fill-rule="evenodd"
                                        d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                                        clip-rule="evenodd"
                                    />
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
            </div>
        </div>
    }
}
