use leptos::prelude::*;
use tw_merge::*;

use phosphor_leptos::{Icon, IconWeightData};

#[derive(TwVariant)]
pub enum HeadingSize {
    #[tw(class = "text-3xl font-bold text-neutral-600 dark:text-white")]
    H1,
    #[tw(class = "text-2xl font-medium text-neutral-600 dark:text-white")]
    H2,
    #[tw(
        default,
        class = "text-xl font-medium text-neutral-600 dark:text-white"
    )]
    H3,
    #[tw(class = "text-lg font-bold text-neutral-600 dark:text-white")]
    H4,
    #[tw(class = "text-lg font-light text-neutral-600 dark:text-white")]
    H5,
    #[tw(class = "text-md font-medium text-neutral-900 dark:text-white")]
    H6,
}

#[derive(TwClass)]
// Optional base class
#[tw(class = "")]
struct HeadingPropsCustom {
    variant: HeadingSize,
}

#[component]
pub fn Heading(
    children: Children,
    #[prop(optional)] variant: Option<HeadingSize>,
) -> impl IntoView {
    let classtext = move || {
        HeadingPropsCustom {
            variant: variant.unwrap_or(HeadingSize::H3),
        }
        .to_class()
    };

    view! { <h1 class=classtext>{children()}</h1> }
}

#[component]
pub fn SubHeading(children: Children) -> impl IntoView {
    view! { <p class="mt-1 text-md text-neutral-500 dark:text-neutral-400 mb-2">{children()}</p> }
}
