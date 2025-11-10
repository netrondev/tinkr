use leptos::ev::Event;
use leptos::prelude::*;
use tw_merge::*;

#[component]
pub fn Select(
    children: Children,
    // #[prop(optional = true, default = false, into)] disabled: bool,
) -> impl IntoView {
    view! {
        <select class=tw_merge!(
            "rounded-lg border-2 border-neutral-200 bg-white px-3 py-2 text-sm font-medium text-neutral-700",
                "dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-300 w-full",
                "disabled:cursor-not-allowed disabled:opacity-50",
                "hover:border-neutral-300 hover:bg-neutral-50 dark:hover:border-neutral-600 dark:hover:bg-neutral-800"
        )>
            // disabled=disabled
            {children()}
        </select>
    }
    .into_any()
}
