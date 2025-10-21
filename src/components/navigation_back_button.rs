use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use phosphor_leptos::CARET_LEFT;

use crate::components::button::{BtnVariant, ButtonIcon};
use crate::components::Button;

#[component]
pub fn NavigationBackButton(#[prop(optional)] class: Option<&'static str>) -> impl IntoView {
    let navigate = use_navigate();

    let on_click = Callback::new(move |_| {
        // Try to use browser's history.back() first
        #[cfg(not(feature = "ssr"))]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(history) = window.history() {
                    // Check if we can go back
                    if let Ok(length) = history.length() {
                        if length > 1 {
                            // We have history, go back
                            let _ = history.back();
                            return;
                        }
                    }
                }
            }
        }

        // Fallback: navigate to home if history.back() is not available
        // This happens when user lands directly on a page or history is not available
        let navigate = navigate.clone();
        navigate("/", Default::default());
    });

    view! {
        <Button
            variant=BtnVariant::Square
            icon=ButtonIcon::Icon(&CARET_LEFT)
            on_click=on_click
            class=class.unwrap_or("")
        />
    }
}
