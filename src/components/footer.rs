use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="py-2">
            <div class="px-4 flex items-center justify-between">
                <div class="flex items-center space-x-2">

                </div>
                <small class="text-neutral-500 dark:text-neutral-500">
                    {env!("CARGO_PKG_VERSION")}
                </small>
            </div>
        </footer>
    }
}
