use leptos::prelude::*;

#[component]
pub fn FormSection(
    #[prop(into)] title: String,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let base_class = "";
    let final_class = match class {
        Some(custom_class) => format!("{} {}", base_class, custom_class),
        None => base_class.to_string(),
    };

    view! {
        <div class=final_class>
            <h3 class="text-lg font-medium text-neutral-900 dark:text-neutral-100 mb-4">
                {title}
            </h3>
            {children()}
        </div>
    }
}

#[component]
pub fn FormGrid(
    #[prop(optional)] cols: Option<u8>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let cols_class = match cols {
        Some(1) => "grid-cols-1",
        Some(2) => "grid-cols-1 md:grid-cols-2",
        Some(3) => "grid-cols-1 md:grid-cols-2 lg:grid-cols-3",
        Some(4) => "grid-cols-1 md:grid-cols-2 lg:grid-cols-4",
        _ => "grid-cols-1",
    };

    let base_class = format!("grid {} gap-4", cols_class);
    let final_class = match class {
        Some(custom_class) => format!("{} {}", base_class, custom_class),
        None => base_class.to_string(),
    };

    view! {
        <div class=final_class>
            {children()}
        </div>
    }
}

#[component]
pub fn FormActions(
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let base_class = "flex justify-between items-center pt-4 border-t dark:border-neutral-700";
    let final_class = match class {
        Some(custom_class) => format!("{} {}", base_class, custom_class),
        None => base_class.to_string(),
    };

    view! {
        <div class=final_class>
            {children()}
        </div>
    }
}
