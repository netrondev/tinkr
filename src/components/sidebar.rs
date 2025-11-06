use crate::components::button::{BtnColor, BtnState, BtnVariant};
use crate::components::{Align, Tooltip};
use crate::components::{Button, button::ButtonIcon};
use leptos::callback::Callback;
use leptos::prelude::*;
use phosphor_leptos::{ARROW_LINE_LEFT, ARROW_LINE_RIGHT, SIDEBAR};
use tw_merge::tw_merge;

#[derive(Debug, Clone)]
pub struct NavLink {
    pub name: String,
    pub icon: ButtonIcon,
    pub icon_hover: Option<ButtonIcon>,
    // pub background_color: crate::colors::Color,
    pub url: String,
}

#[derive(Debug, Clone)]
pub enum NavItem {
    Link(NavLink),
    Divider,
    Gap,
}

impl NavItem {
    pub fn view_list(links: Vec<NavItem>) -> impl IntoView {
        view! {
            <NavItemList links=links />
        }
        .into_any()
    }
}

#[component]
pub fn NavItemList(links: Vec<NavItem>) -> impl IntoView {
    let location = leptos_router::hooks::use_location();
    let pathname = move || location.pathname.get();
    let nav_items = StoredValue::new(links);

    view! {
        <For
            each=move || {
                let current_path = pathname();
                nav_items
                    .get_value()
                    .into_iter()
                    .map(move |item| { (item, current_path.clone()) })
                    .collect::<Vec<_>>()
            }
            key=|(item, path)| format!("{:#?}-{}", item, path)
            children=move |(item, current_path)| {
                match item {
                    NavItem::Link(item) => {
                        {
                            let href = item.url.clone();
                            let href2 = item.url.clone();
                            let is_active = current_path == href;

                            view! {
                                <Button
                                    variant=BtnVariant::CallToAction
                                    icon=item.icon.clone()
                                    // state=if is_active {
                                    //     BtnState::Active
                                    // } else {
                                    //     BtnState::Default
                                    // }
                                    color=if is_active {
                                        BtnColor::Primary
                                    } else {
                                        BtnColor::Neutral
                                    }
                                    on:click=move |ev: web_sys::MouseEvent| {
                                        ev.prevent_default();
                                        window().location().set_href(&item.url).unwrap();
                                    }
                                    href=href2.clone()
                                >
                                    {item.name}
                                </Button>
                            }
                        }
                            .into_any()
                    }
                    NavItem::Divider => view! { <div /> }.into_any(),
                    NavItem::Gap => view! { <div class="h-2 flex-1" /> }.into_any(),
                }
            }
        />
    }
}

#[component]
pub fn SideBar(links: Vec<NavItem>) -> impl IntoView {
    let location = leptos_router::hooks::use_location();
    let pathname = move || location.pathname.get();

    let nav_items = StoredValue::new(links);
    let (is_wide, set_is_wide) = signal(false);
    let (is_mobile_open, set_is_mobile_open) = signal(false);

    view! {
        <>
            // Mobile menu toggle button - fixed at top left
            <div class="md:hidden fixed top-2 left-2 z-50">
                <Button
                    icon=ButtonIcon::Icon(SIDEBAR)
                    variant=BtnVariant::Square
                    on_click=Callback::new(move |_| set_is_mobile_open.set(!is_mobile_open.get()))
                />
            </div>

            // Mobile overlay backdrop
            <Show when=move || is_mobile_open.get()>
                <div
                    class="md:hidden fixed inset-0 bg-black bg-opacity-50 z-40"
                    on:click=move |_| set_is_mobile_open.set(false)
                />
            </Show>

            // Sidebar content
            <div class=move || {
                tw_merge!(
                    "flex flex-col h-screen justify-start p-2 gap-2 bg-white dark:bg-black z-50",
                    // Mobile styles
                    "fixed md:relative left-0 top-0 z-50 md:z-0",
                    // Mobile visibility
                    if is_mobile_open.get() { "translate-x-0" } else { "-translate-x-full md:translate-x-0" },
                    // Desktop width
                    if is_wide.get() { "w-44" } else { "w-14" }
                )
            }>

                <nav class=move || {
                    tw_merge!(
                        "flex flex-col gap-2 justify-start items-start flex-1",
                        if is_wide.get() { "w-40" } else { "w-14" }
                    )
                }>
                    <Show
                        when=move || is_wide.get()
                        fallback=move || {
                            view! {
                                <For
                                    each=move || {
                                        let current_path = pathname();
                                        nav_items
                                            .get_value()
                                            .into_iter()
                                            .map(move |item| { (item, current_path.clone()) })
                                            .collect::<Vec<_>>()
                                    }
                                    key=|(item, path)| format!("{:#?}-{}", item, path)
                                    children=move |(item, current_path)| {
                                        let is_active = match item.clone() {
                                            NavItem::Link(link) => current_path == link.url,
                                            _ => false,
                                        };
                                        match item {
                                            NavItem::Link(link) => {

                                                view! {
                                                    <Tooltip label=link.name.clone() align=Align::Right>
                                                        <Button
                                                            icon=link.icon.clone()
                                                            state=MaybeProp::from(
                                                                if is_active { BtnState::Active } else { BtnState::Default },
                                                            )
                                                            href=link.url.clone()
                                                            variant=BtnVariant::Square
                                                            on_click=Callback::new(move |_| {
                                                                set_is_mobile_open.set(false)
                                                            })
                                                        />
                                                    </Tooltip>
                                                }
                                                    .into_any()
                                            }
                                            NavItem::Divider => view! { <div /> }.into_any(),
                                            NavItem::Gap => {
                                                view! { <div class="flex-1 h-full" /> }.into_any()
                                            }
                                        }
                                    }
                                />
                            }
                        }
                    >
                        <For
                            each=move || {
                                let current_path = pathname();
                                nav_items
                                    .get_value()
                                    .into_iter()
                                    .map(move |item| { (item, current_path.clone()) })
                                    .collect::<Vec<_>>()
                            }
                            key=|(item, path)| format!("{:#?}-{}", item, path)
                            children=move |(item, current_path)| {
                                let is_active = match item.clone() {
                                    NavItem::Link(link) => current_path == link.url,
                                    _ => false,
                                };
                                match item {
                                    NavItem::Link(link) => {

                                        view! {
                                            <Button
                                                icon=link.icon.clone()
                                                state=if is_active {
                                                    BtnState::Active
                                                } else {
                                                    BtnState::Default
                                                }
                                                href=link.url.clone()
                                                variant=BtnVariant::Default
                                                class="w-full"
                                                on_click=Callback::new(move |_| {
                                                    set_is_mobile_open.set(false)
                                                })
                                            >
                                                {link.name.clone()}
                                            </Button>
                                        }
                                            .into_any()
                                    }
                                    NavItem::Divider => {

                                        view! { <div /> }
                                            .into_any()
                                    }
                                    NavItem::Gap => view! { <div class="h-2 flex-1" /> }.into_any(),
                                }
                            }
                        />
                    </Show>

                    <div class="hidden md:block">
                        <Tooltip label="toggle sidebar".to_string() align=Align::Right>
                            <Show
                                when=move || !is_wide.get()
                                fallback=move || {
                                    view! {
                                        <Button
                                            icon=ButtonIcon::Icon(SIDEBAR)
                                            icon_hover=ButtonIcon::Icon(ARROW_LINE_LEFT)
                                            variant=BtnVariant::Square
                                            on_click=Callback::new(move |_| {
                                                set_is_wide.set(!is_wide.get())
                                            })
                                        />
                                    }
                                }
                            >
                                <Button
                                    icon=ButtonIcon::Icon(SIDEBAR)
                                    icon_hover=ButtonIcon::Icon(ARROW_LINE_RIGHT)
                                    variant=BtnVariant::Square
                                    on_click=Callback::new(move |_| set_is_wide.set(!is_wide.get()))
                                />
                            </Show>
                        </Tooltip>
                    </div>

                </nav>
            </div>
        </>
    }
}
