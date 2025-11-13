use leptos::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Page(children: Children, #[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <div class=tw_merge!(
            "bg-white dark:bg-neutral-900/50 rounded-lg w-full",
    "border border-neutral-200 dark:border-neutral-800 p-6 flex flex-col gap-5", class
        )>{children()}</div>
    }
}
