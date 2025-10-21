use leptos::prelude::*;

#[component]
pub fn ColorPicker(
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(String) + 'static>>,
    #[prop(optional)] on_change: Option<Box<dyn Fn(String) + 'static>>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let base_class = r#"
        w-full h-10 px-1 py-1 
        border border-neutral-300 dark:border-neutral-600 
        dark:bg-neutral-800 
        rounded-md
        cursor-pointer
    "#;

    let final_class = match class {
        Some(custom_class) => format!("{} {}", base_class, custom_class),
        None => base_class.to_string(),
    };

    view! {
        <input
            type="color"
            id=id
            name=name
            class=final_class
            value=move || value.map(|v| v.get()).unwrap_or_else(|| "#000000".to_string())
            on:input=move |ev| {
                let val = event_target_value(&ev);
                if let Some(v) = value {
                    v.set(val.clone());
                }
                if let Some(handler) = &on_input {
                    handler(val);
                }
            }
            on:change=move |ev| {
                if let Some(handler) = &on_change {
                    handler(event_target_value(&ev));
                }
            }
            required=required.unwrap_or(false)
            disabled=disabled.unwrap_or(false)
        />
    }
}
