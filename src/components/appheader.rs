use crate::{
    components::{
        Align,
        button::{BtnState, BtnVariant, Button, ButtonIcon},
        sidebar::NavItem,
        user_navbutton::UserNavButton,
    },
    theme::ThemeToggle,
};
use leptos::prelude::*;
use phosphor_leptos::{LIST, X};

#[component]
pub fn AppHeader<Logo, Cart, IV, IB>(
    #[prop()] logo: Logo,
    #[prop()] cart: Cart,
    #[prop()] nav_items: RwSignal<Vec<NavItem>>,
    #[prop()] nav_items_user: RwSignal<Vec<NavItem>>,
) -> impl IntoView
where
    Logo: Fn() -> IV,
    IV: IntoView,
    Cart: Fn() -> IB,
    IB: IntoView,
{
    let location = leptos_router::hooks::use_location();
    let pathname = move || location.pathname.get();
    let (is_mobile_menu_open, set_mobile_menu_open) = signal(false);

    view! {
        <header class="">
            <div class="">
                <div class="flex items-center h-16 gap-2 md:gap-5">

                    // Mobile navigation area
                    <div class="md:hidden flex items-center">
                        {move || {
                            view! {
                                <Button
                                    variant=BtnVariant::Square
                                    icon=match is_mobile_menu_open.get() {
                                        true => ButtonIcon::Icon(&X),
                                        false => ButtonIcon::Icon(&LIST),
                                    }
                                    on_click=Callback::new(move |_| {
                                        set_mobile_menu_open.update(|v| *v = !*v)
                                    })
                                />
                            }
                        }}
                    </div>

                    <div class="flex items-center">{logo()}</div>

                    // Desktop navigation
                    <nav class="hidden md:flex items-center space-x-4">
                        <For
                            each=move || {
                                let current_path = pathname();
                                nav_items
                                    .get()
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
                                                    state=if is_active {
                                                        BtnState::Active
                                                    } else {
                                                        BtnState::Default
                                                    }
                                                    icon=item.icon.clone()
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
                    </nav>

                    <div class="flex-1" />

                    <div class="hidden md:block">
                        <ThemeToggle tooltip_align=Align::Bottom />
                    </div>

                    {cart()}

                    <UserNavButton nav_items=nav_items_user />
                </div>

                // Mobile menu dropdown
                <Show when=move || is_mobile_menu_open.get()>
                    <nav class="md:hidden pb-4">
                        <div class="flex flex-col space-y-1">
                            <For
                                each=move || nav_items.get()
                                key=|item| format!("{:#?}", item)
                                children=move |item| {
                                    match item {
                                        NavItem::Link(item) => {
                                            {
                                                let href = item.url.clone();
                                                let is_active = move || pathname() == href;

                                                view! {
                                                    <Button
                                                        variant=BtnVariant::Default
                                                        state=if is_active() {
                                                            BtnState::Active
                                                        } else {
                                                            BtnState::Default
                                                        }
                                                        on_click=Callback::new(move |_| {
                                                            set_mobile_menu_open.set(false);
                                                            window().location().set_href(&item.url).unwrap();
                                                        })
                                                        class="w-full text-left"
                                                        icon=item.icon.clone()
                                                    >
                                                        {item.name}
                                                    </Button>
                                                }
                                            }
                                                .into_any()
                                        }
                                        NavItem::Divider => view! { <div /> }.into_any(),
                                        NavItem::Gap => {
                                            view! { <div class="h-2 flex-1" /> }.into_any()
                                        }
                                    }
                                }
                            />

                            // <ThemeToggle tooltip_align=Align::Right />

                        </div>
                    </nav>
                </Show>
            </div>
        </header>
    }
}
