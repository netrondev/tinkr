use leptos::context::Provider;
use leptos::prelude::*;
use tw_merge::tw_merge;

use crate::boring_avatars::{Avatar, AvatarVariants};
use crate::components::Button;
use crate::components::button::{BtnVariant, ButtonIcon};
use crate::user::AdapterUser;
#[component]
pub fn Dropdown(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let dropdown_visible = RwSignal::new(false);
    let default_class = "relative cursor-pointer";
    let final_class = tw_merge!(default_class, class);

    view! {
        <div class=final_class on:click=move |_| dropdown_visible.set(false)>
            <Provider value=dropdown_visible>
                <div on:click=move |e| e.stop_propagation()>{children()}</div>
            </Provider>
        </div>
    }
}

#[component]
pub fn DropdownTrigger(
    #[prop(optional)] icon: Option<ButtonIcon>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] variant: Option<BtnVariant>,
) -> impl IntoView {
    let dropdown_visible = expect_context::<RwSignal<bool>>();

    view! {
        <Button
            // type="button"
            icon=icon.unwrap()
            variant=variant.unwrap_or(BtnVariant::Default)
            on:click=move |e| {
                e.stop_propagation();
                dropdown_visible.update(|v| *v = !*v);
            }
            on:touchstart=move |e| {
                e.stop_propagation();
            }
            on:touchend=move |e| {
                e.prevent_default();
                e.stop_propagation();
                dropdown_visible.update(|v| *v = !*v);
            }
        >
            {match children {
                Some(children) => children(),
                None => view! {}.into_any(),
            }}
        </Button>
    }
}

pub enum DropdownSide {
    Left,
    Right,
}

#[component]
pub fn DropdownMenu(
    children: ChildrenFn,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] side: Option<DropdownSide>,
) -> impl IntoView {
    let dropdown_visible = expect_context::<RwSignal<bool>>();
    // let default_class = "absolute right-0 top-14 mt-2 max-w-96 w-full bg-white dark:bg-neutral-800 rounded-md shadow-lg ring-1 ring-black ring-opacity-5 dark:ring-neutral-700 z-50";
    // let default_class = "absolute right-0 top-14 mt-2 max-w-96 w-full dark:bg-sky-500 bg-sky-500 rounded-md shadow-lg ring-1 ring-black ring-opacity-5 dark:ring-neutral-700 z-50";

    let default_class = "absolute top-full mt-1 w-auto flex flex-col bg-white dark:bg-neutral-800 rounded-md shadow-lg ring-1 ring-black ring-opacity-5 dark:ring-neutral-700 z-50";
    let final_class = match side {
        Some(DropdownSide::Left) => {
            format!("{} left-0", class.unwrap_or(default_class))
        }
        Some(DropdownSide::Right) => {
            format!("{} right-0", class.unwrap_or(default_class))
        }
        None => {
            format!("{} left-0", class.unwrap_or(default_class))
        }
    };

    view! {
        <Show when=move || dropdown_visible.get()>
            <div
                class="fixed top-0 left-0 w-screen h-screen opacity-0 cursor-default"
                on:click=move |_| dropdown_visible.set(false)
            />
            <div class=final_class.clone() on:click=move |e| e.stop_propagation()>
                <div class="py-1 w-full flex flex-col">{children()}</div>
            </div>
        </Show>
    }
}

#[component]
pub fn DropdownItem(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] href: Option<String>,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let dropdown_visible = expect_context::<RwSignal<bool>>();
    let default_class = "block w-full text-left px-4 py-2 text-sm text-neutral-700 dark:text-neutral-300 hover:bg-neutral-100 dark:hover:bg-neutral-700";
    let final_class = class.unwrap_or(default_class.into());

    let handle_click = move |_| {
        dropdown_visible.set(false);
        if let Some(callback) = on_click {
            callback.run(());
        }
    };

    if let Some(href_val) = href {
        view! {
            <leptos_router::components::A
                href=href_val
                attr:class=final_class
                on:click=handle_click
            >
                {children()}
            </leptos_router::components::A>
        }
        .into_any()
    } else {
        view! {
            <button type="button" class=final_class on:click=handle_click>
                {children()}
            </button>
        }
        .into_any()
    }
}

#[component]
pub fn DropdownHeader(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let default_class = "px-4 py-2 text-sm text-neutral-700 dark:text-neutral-300 border-b dark:border-neutral-700 whitespace-nowrap";
    let final_class = class.unwrap_or(default_class);

    view! { <div class=final_class>{children()}</div> }
}

#[component]
pub fn AvatarButton(user: AdapterUser) -> impl IntoView {
    if let Some(image) = user.image.clone() {
        return view! { <DropdownTrigger icon=ButtonIcon::ImageCover(image) variant=BtnVariant::Round /> }
        .into_any();
    };

    return view! {
        <DropdownTrigger
            // icon=ButtonIcon::Icon(phosphor_leptos::USER_CIRCLE_DASHED)
            icon=ButtonIcon::Avatar(user.name)
            variant=BtnVariant::Round
        />
    }
    .into_any();
}
