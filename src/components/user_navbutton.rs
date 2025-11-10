use crate::components::button::{BtnColor, BtnState, BtnVariant};

use crate::components::{
    Align, AvatarButton, Button, Dropdown, DropdownHeader, DropdownItem, DropdownMenu,
    DropdownSide, NavItem, Tooltip,
};

use crate::session::get_user;
use leptos::prelude::*;

#[component]
pub fn UserNavButton(#[prop()] nav_items: RwSignal<Vec<NavItem>>) -> impl IntoView {
    let user_resource = OnceResource::new(get_user());

    view! {
        <Dropdown>
            <Suspense fallback=move || {
                view! { <div class="w-8 h-8 bg-neutral-300 rounded-full animate-pulse"></div> }
            }>

                {move || {
                    match user_resource.get() {
                        Some(Ok(user)) => {
                            view! {
                                <Tooltip label=user.name.clone() align=Align::Bottom>
                                    <AvatarButton user=user.clone() />
                                </Tooltip>
                            }
                                .into_any()
                        }
                        _ => {
                            view! {
                                <Button
                                    href="/login"
                                    color=BtnColor::Primary
                                    variant=BtnVariant::CallToAction
                                >
                                    "Sign in"
                                </Button>
                            }
                                .into_any()
                        }
                    }
                }}
            </Suspense>

            <DropdownMenu side=DropdownSide::Right>
                <Suspense fallback=move || {
                    view! { <div class="p-2">"Loading..."</div> }
                }>
                    {move || {
                        match user_resource.get() {
                            Some(Ok(user)) => {
                                let location = leptos_router::hooks::use_location();
                                view! {
                                    <For
                                        each=move || nav_items.get()
                                        key=|item| format!("{:#?}", item)
                                        children=move |item| {
                                            match item {
                                                NavItem::Link(link) => {
                                                    let url = link.url.clone();

                                                    view! {
                                                        <DropdownItem href=url>
                                                            <div class="flex flex-row items-center gap-2 whitespace-nowrap">
                                                                {link.icon.display()} {link.name.clone()}
                                                            </div>
                                                        </DropdownItem>
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
                                    .into_any()
                            }
                            _ => {
                                view! {
                                    <DropdownHeader>"Error"</DropdownHeader>
                                    <DropdownItem href="/logout".into()>"Logout"</DropdownItem>
                                }
                                    .into_any()
                            }
                        }
                    }}
                </Suspense>
            </DropdownMenu>
        </Dropdown>
    }
}
