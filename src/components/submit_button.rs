use leptos::prelude::*;
use phosphor_leptos::{Icon, IconWeightData};

#[component]
pub fn SubmitButton(
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional)] is_submitting: Option<ReadSignal<bool>>,
    #[prop(optional)] icon: Option<&'static IconWeightData>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let base_class = "px-4 py-2 bg-purple-600 dark:bg-purple-700 text-white rounded hover:bg-purple-700 dark:hover:bg-purple-600 disabled:opacity-50 disabled:cursor-not-allowed";

    let final_class = match class {
        Some(custom_class) => format!("{} {}", base_class, custom_class),
        None => base_class.to_string(),
    };

    let button_text = text.unwrap_or_else(|| "Submit".to_string());

    view! {
        <button
            type="submit"
            class=final_class
            disabled=move || is_submitting.map(|s| s.get()).unwrap_or(false)
        >
            <div class="flex items-center justify-center gap-2">
                {icon.map(|i| view! { <Icon icon=i size="20px" /> })}
                {move || {
                    let is_loading = is_submitting.map(|s| s.get()).unwrap_or(false);
                    if is_loading {
                        format!("{}...", button_text)
                    } else {
                        button_text.clone()
                    }
                }}
            </div>
        </button>
    }
}
