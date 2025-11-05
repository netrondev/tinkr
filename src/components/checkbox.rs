use leptos::prelude::*;

#[component]
pub fn Checkbox(
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional)] checked: Option<RwSignal<bool>>,
    #[prop(optional)] on_change: Option<Box<dyn Fn(bool) + 'static>>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let base_checkbox_class = r#"
        rounded border-neutral-300 dark:border-neutral-600 
        text-purple-600 dark:text-purple-400 
        focus:ring-purple-500 dark:focus:ring-purple-400 
        dark:bg-neutral-800
    "#;

    let final_checkbox_class = match class {
        Some(custom_class) => format!("{} {}", base_checkbox_class, custom_class),
        None => base_checkbox_class.to_string(),
    };

    let checkbox_view = view! {
        <input
            type="checkbox"
            id=id.clone()
            name=name
            class=final_checkbox_class
            checked=move || checked.map(|c| c.get()).unwrap_or(false)
            on:change=move |ev| {
                let is_checked = event_target_checked(&ev);
                if let Some(c) = checked {
                    c.set(is_checked);
                }
                if let Some(handler) = &on_change {
                    handler(is_checked);
                }
            }
            required=required.unwrap_or(false)
            disabled=disabled.unwrap_or(false)
        />
    };

    if label.is_some() || children.is_some() {
        view! {
            <label class="flex items-center cursor-pointer">
                {checkbox_view}
                <span class="ml-2 text-sm text-neutral-700 dark:text-neutral-300">
                    {label.unwrap_or_default()} {children.map(|c| c())}
                </span>
            </label>
        }
        .into_any()
    } else {
        checkbox_view.into_any()
    }
}
