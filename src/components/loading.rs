use leptos::prelude::*;
use phosphor_leptos::{Icon, CIRCLE_NOTCH};
use tw_merge::*;
pub struct Loading {}

#[derive(TwClass)]
// Optional base class
#[tw(class = "flex")]
struct LoadingProps {
    size: LoadingIndicatorSize,
}

#[derive(TwVariant)]
pub enum LoadingIndicatorSize {
    #[tw(default, class = "w-4 h-4")]
    Small,
    #[tw(class = "w-6 h-6")]
    Medium,
    #[tw(class = "w-8 h-8")]
    Large,
    #[tw(class = "w-20 h-20")]
    Huge,
}

#[component]
pub fn LoadingIndicator(#[prop(optional)] size: Option<LoadingIndicatorSize>) -> impl IntoView {
    let class_text = move || {
        LoadingProps {
            size: size.unwrap_or_default(),
        }
        .to_class()
    };

    let default_class =
        "animate-spin text-neutral-600 dark:text-neutral-400 flex items-center justify-center rounded-full";

    let final_class = move || tw_merge!(default_class, class_text());

    view! {
        <div>
            <div class=final_class>
                <Icon icon=CIRCLE_NOTCH size="100%" />
            </div>
        </div>
    }
}
