use leptos::prelude::*;

#[component]
pub fn Version() -> impl IntoView {
    let version = env!("CARGO_PKG_VERSION");

    view! {
        <div class="w-full justify-center mt-5 text-xs flex">
            <span class="text-xs">{version}beta</span>
        </div>
    }
}
