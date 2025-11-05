use leptos::prelude::*;

#[component]
pub fn Label(children: Children) -> impl IntoView {
    view! {
        <label class="block text-sm font-medium text-neutral-500 dark:text-neutral-500 mb-1">
            {children()}
        </label>
    }
    .into_view()
}
