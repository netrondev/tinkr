use leptos::prelude::*;

use crate::components::{
    button::{BtnState, BtnVariant},
    Button,
};

#[component]
pub fn TabButton<F>(active: F, children: Children) -> impl IntoView
where
    F: Fn() -> bool + Send + Sync + 'static,
{
    let state = Signal::derive(move || {
        if active() {
            BtnState::TabActive
        } else {
            BtnState::Default
        }
    });

    view! {
        <Button variant=BtnVariant::Tab class="px-4" state=state>
            {children()}
        </Button>
    }
}

#[component]
pub fn TabNavGroup(children: Children) -> impl IntoView {
    view! {
        <div class="border-b border-gray-200 dark:border-gray-700 my-5">
            <nav class="flex justify-left">{children()}</nav>
        </div>
    }
}
