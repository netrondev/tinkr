use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn Label(
    #[prop(into)] for_id: String,
    children: Children,
    #[prop(optional)] required: bool,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let default_class = "block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1";
    let final_class = class.unwrap_or(default_class);

    view! {
        <label for=for_id class=final_class>
            {children()}
            {if required {
                view! { <span class="text-red-500">" *"</span> }.into_any()
            } else {
                view! {}.into_any()
            }}
        </label>
    }
}

#[component]
pub fn Input(
    #[prop(into)] id: String,
    #[prop(into)] r#type: String,
    value: Signal<String>,
    on_input: impl Fn(String) + 'static,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] required: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let default_class = "w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 bg-white dark:bg-neutral-800 text-neutral-900 dark:text-white rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 disabled:opacity-50 disabled:cursor-not-allowed";
    let final_class = class.unwrap_or(default_class);

    view! {
        <input
            type=r#type
            id=id.clone()
            name=id
            class=final_class
            prop:value=move || value.get()
            on:input=move |e| on_input(event_target_value(&e))
            placeholder=placeholder.unwrap_or("")
            required=required
            disabled=disabled
        />
    }
}

#[component]
pub fn Textarea(
    #[prop(into)] id: String,
    value: Signal<String>,
    on_input: impl Fn(String) + 'static,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] rows: Option<i32>,
    #[prop(optional)] required: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let default_class = "w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 bg-white dark:bg-neutral-800 text-neutral-900 dark:text-white rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 disabled:opacity-50 disabled:cursor-not-allowed";
    let final_class = class.unwrap_or(default_class);

    view! {
        <textarea
            id=id.clone()
            name=id
            class=final_class
            rows=rows.unwrap_or(3)
            prop:value=move || value.get()
            on:input=move |e| on_input(event_target_value(&e))
            placeholder=placeholder.unwrap_or("")
            required=required
            disabled=disabled
        />
    }
}

#[component]
pub fn FormButton(
    children: Children,
    #[prop(optional)] r#type: Option<&'static str>,
    #[prop(optional)] variant: Option<&'static str>,
    #[prop(optional)] disabled: Option<Signal<bool>>,
    #[prop(optional)] on_click: Option<Box<dyn Fn(ev::MouseEvent)>>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let button_type = r#type.unwrap_or("button");
    let variant = variant.unwrap_or("primary");

    let base_class = "px-4 py-2 rounded-md focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed";

    let variant_class = match variant {
        "primary" => "bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500 dark:focus:ring-offset-neutral-800",
        "secondary" => "bg-neutral-200 dark:bg-neutral-700 text-neutral-700 dark:text-neutral-300 hover:bg-neutral-300 dark:hover:bg-neutral-600 focus:ring-neutral-500 dark:focus:ring-offset-neutral-800",
        _ => "bg-neutral-200 dark:bg-neutral-700 text-neutral-700 dark:text-neutral-300 hover:bg-neutral-300 dark:hover:bg-neutral-600 focus:ring-neutral-500 dark:focus:ring-offset-neutral-800",
    };

    let final_class = if let Some(custom_class) = class {
        format!("{} {} {}", base_class, variant_class, custom_class)
    } else {
        format!("{} {}", base_class, variant_class)
    };

    view! {
        <button
            type=button_type
            class=final_class
            disabled=move || disabled.map(|d| d.get()).unwrap_or(false)
            on:click=move |e| {
                if let Some(handler) = &on_click {
                    handler(e);
                }
            }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn FormGroup(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let default_class = "space-y-4";
    let final_class = class.unwrap_or(default_class);

    view! {
        <div class=final_class>
            {children()}
        </div>
    }
}

#[component]
pub fn FormField(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let default_class = "";
    let final_class = class.unwrap_or(default_class);

    view! {
        <div class=final_class>
            {children()}
        </div>
    }
}

#[component]
pub fn FormActions(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let default_class = "flex gap-4 pt-4";
    let final_class = class.unwrap_or(default_class);

    view! {
        <div class=final_class>
            {children()}
        </div>
    }
}
