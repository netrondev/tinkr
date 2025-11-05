use leptos::prelude::*;
use tw_merge::*;

use phosphor_leptos::{Icon, IconWeightData};

#[derive(Debug, Clone)]
pub enum ButtonIcon {
    Image(String),
    ImageCover(String),
    Icon(&'static IconWeightData),
    View(fn() -> AnyView),
}

impl ButtonIcon {
    pub fn display(&self) -> AnyView {
        match self {
            ButtonIcon::Image(url) => {
                view! { <img src=url alt="" class="size-[20px]" /> }.into_any()
            }
            ButtonIcon::ImageCover(url) => {
                view! { <img src=url alt="" class="size-8 rounded-full object-cover" /> }.into_any()
            }
            ButtonIcon::Icon(icon) => view! { <Icon icon=icon size="20px" /> }.into_any(),
            ButtonIcon::View(view_fn) => {
                view! { <div class="size-[20px] flex">{view_fn()}</div> }.into_any()
            }
        }
    }
}

// Your Component Type
#[derive(TwClass)]
// Optional base class
#[tw(class = "flex")]
struct ButtonsProps {
    variant: BtnVariant,
    color: BtnColor,
    state: BtnState,
}

// Variant for size
#[derive(TwVariant)]
pub enum BtnVariant {
    #[tw(default, class = "h-10 rounded-md p-4")]
    Default,
    #[tw(class = "h-10 w-full rounded-md p-4")]
    Wide,
    #[tw(class = "w-10 h-10 rounded-md")]
    Square,
    #[tw(class = "w-10 h-10 rounded-full")]
    Round,
    #[tw(class = "h-10 rounded-md px-3")]
    CallToAction,
    #[tw(
        class = "border-b-2 w-min px-4 rounded-none border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:border-gray-300 dark:hover:border-gray-600 py-2 px-1 text-sm font-medium"
    )]
    Tab,
}

// Variant for color
#[derive(TwVariant)]
pub enum BtnColor {
    #[tw(
        default,
        class = "text-neutral-700 bg-neutral-100 dark:bg-neutral-800 dark:text-neutral-300 hover:bg-neutral-200 dark:hover:bg-neutral-700 active:bg-blue-500 dark:active:bg-blue-600"
    )]
    Default,
    #[tw(
        class = "text-sky-900 dark:text-sky-100 bg-sky-200 dark:bg-sky-700 hover:bg-sky-300 dark:hover:bg-sky-600"
    )]
    Primary,
    #[tw(
        class = "text-white dark:text-emerald-100 bg-emerald-600 dark:bg-emerald-700 hover:bg-emerald-300 dark:hover:bg-emerald-600"
    )]
    Success,
    #[tw(
        class = "text-rose-900 dark:text-rose-100 bg-rose-200 dark:bg-rose-700 hover:bg-rose-300 dark:hover:bg-rose-600"
    )]
    Error,
    #[tw(
        class = "bg-neutral-200 dark:bg-neutral-900 text-neutral-700 dark:text-neutral-300 hover:bg-neutral-600 dark:hover:bg-neutral-700"
    )]
    Neutral,
    #[tw(
        class = "rounded-lg border-2 border-neutral-200 bg-white px-3 py-2 text-sm font-medium text-neutral-700 hover:border-neutral-300 hover:bg-neutral-50 dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-300 dark:hover:border-neutral-600 dark:hover:bg-neutral-800"
    )]
    Outlined,
    #[tw(
        class = "rounded-lg border-2 border-blue-500 bg-blue-50 px-3 py-2 text-sm font-medium text-blue-700 dark:border-blue-400 dark:bg-blue-900/30 dark:text-blue-300"
    )]
    OutlinedActive,
}

#[derive(TwVariant)]
pub enum BtnState {
    #[tw(default, class = "")]
    Default,
    #[tw(class = "bg-neutral-200 dark:bg-neutral-800")]
    Active,
    #[tw(
        class = "border-b-2 hover:border-sky-600 hover:text-sky-500 border-sky-500 hover:dark:border-sky-300 hover:dark:text-sky-300 dark:border-sky-400 text-sky-600 dark:text-sky-400 py-2 px-1 text-sm font-medium"
    )]
    TabActive,
}

// pub struct Flare {
//     pub text: String,
// }

#[component]
pub fn Button(
    #[prop(optional)] variant: Option<BtnVariant>,
    #[prop(optional)] _label: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] state: MaybeProp<BtnState>,
    #[prop(optional)] icon: Option<ButtonIcon>,
    #[prop(optional)] icon_hover: Option<ButtonIcon>,
    // #[prop(optional)] flare: Option<Flare>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] color: Option<BtnColor>,
    // #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] button_type: Option<&'static str>,
) -> impl IntoView {
    let classtext = move || {
        ButtonsProps {
            variant: variant.unwrap_or(BtnVariant::Default),
            color: color.unwrap_or(BtnColor::Default),
            state: state.get().unwrap_or(BtnState::Default),
        }
        .to_class()
    };

    // let is_disabled = move || disabled.get().unwrap_or(false);

    let default_class = "flex relative items-center rounded-md justify-left w-min text-left text-sm group cursor-pointer whitespace-nowrap";
    let disabled_class = move || {
        if disabled {
            "opacity-50 cursor-not-allowed pointer-events-none"
        } else {
            ""
        }
    };

    let overrides = if icon.is_some() { "pl-0" } else { "" };

    let final_class = move || {
        tw_merge!(
            default_class,
            classtext(),
            disabled_class(),
            class,
            overrides
        )
    };

    let handle_click = move |_| {
        if !disabled {
            if let Some(callback) = on_click {
                callback.run(());
            }
        }
    };

    let button_contents = match variant {
        Some(BtnVariant::Square) => view! {
            <div class="w-full flex items-center justify-center relative">
                {match (&icon, &icon_hover) {
                    (Some(normal_icon), Some(hover_icon)) => view! {
                        <>
                            <div class="group-hover:hidden">
                                {normal_icon.display()}
                            </div>
                            <div class="hidden group-hover:block">
                                {hover_icon.display()}
                            </div>
                        </>
                    }.into_any(),
                    (Some(normal_icon), None) => normal_icon.display(),
                    (None, _) => view! { <div /> }.into_any(),
                }}
                {match children {
                    Some(children) => children(),
                    None => view! {  }.into_any(),
                }}
            </div>
        }
        .into_any(),
        Some(BtnVariant::Round) => view! {
            <div class="w-full flex items-center justify-center relative">
                {match (&icon, &icon_hover) {
                    (Some(normal_icon), Some(hover_icon)) => view! {
                        <>
                            <div class="group-hover:hidden">
                                {normal_icon.display()}
                            </div>
                            <div class="hidden group-hover:block">
                                {hover_icon.display()}
                            </div>
                        </>
                    }.into_any(),
                    (Some(normal_icon), None) => normal_icon.display(),
                    (None, _) => view! { <div /> }.into_any(),
                }}
                {match children {
                    Some(children) => children(),
                    None => view! {  }.into_any(),
                }}
            </div>
        }
        .into_any(),
        _ => view! {
            <div class="w-full flex items-center justify-left gap-2">

                {match (&icon, &icon_hover) {
                    (Some(normal_icon), Some(hover_icon)) => view! {
                        <div class="w-10 flex items-center justify-center">
                            <div class="group-hover:hidden">
                                {normal_icon.display()}
                            </div>
                            <div class="hidden group-hover:block">
                                {hover_icon.display()}
                            </div>
                        </div>
                    }.into_any(),
                    (Some(normal_icon), None) => view! { <div class="w-10 flex items-center justify-center">{normal_icon.display()}</div> }.into_any(),
                    (None, _) => view! {  }.into_any(),
                }}

                <span>{match children {
                    Some(children) => children(),
                    None => view! {  }.into_any(),
                }}</span>
            </div>
        }
        .into_any(),
    };

    // let button_contents = view! {
    //     <div class="w-full flex flex-row items-center gap-2">
    //         <div class="size-[20px]">
    //             <div class="w-full h-full flex items-center justify-center text-white text-2xl font-bold">
    //                 {match icon {
    //                     Some(ButtonIcon::Image(url)) => view! { <img src=url alt="" class="size-[20px]" /> }.into_any(),
    //                     Some(ButtonIcon::Icon(icon)) => view! { <Icon icon=icon size="20px" /> }.into_any(),
    //                     None => view! { <div /> }.into_any(),
    //                 }}
    //             </div>
    //         </div>
    //         <div class="text-md text-center font-normal max-w-16 leading-tight text-shadow ">
    //             {
    //                 match children {
    //                     Some(children) => children(),
    //                     None => view! {  }.into_any(),
    //                 }
    //             }
    //         </div>
    //     </div>
    // };

    if let Some(href_val) = href {
        view! {
            <leptos_router::components::A
                href=href_val
                attr:class=move || final_class()
                on:click=handle_click
            >
                {button_contents}
            </leptos_router::components::A>
        }
        .into_any()
    } else {
        view! {
            <button
                type=button_type.unwrap_or("button")
                class=move || final_class()
                on:click=handle_click
                disabled=disabled
            >
                {button_contents}
            </button>
        }
        .into_any()
    }
}
