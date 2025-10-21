use crate::components::sidebar::NavItemList;
use crate::components::user_navbutton::UserNavButton;
use crate::components::{Align, NavItem};

use leptos::prelude::*;

#[component]
pub fn Navbar<Logo, Cart, IV, IB>(
    #[prop()] logo: Logo,
    #[prop()] cart: Cart,
    #[prop()] navitems: RwSignal<Vec<NavItem>>,
    #[prop()] navitems_user: RwSignal<Vec<NavItem>>,
) -> impl IntoView
where
    Logo: Fn() -> IV,
    IV: IntoView,
    Cart: Fn() -> IB,
    IB: IntoView,
{
    view! {
        <nav class="">
            <div class="px-4">
                <div class="flex items-center h-16 flex-row gap-5">
                    {logo()}

                    {move || view! { <NavItemList links=navitems.get() /> }}

                    <div class="flex-1" />

                    <div class="flex items-center space-x-4">
                        // <OrganizationSelector />
                    </div>

                    <div class="flex items-center space-x-4">
                        <div class="hidden md:block">
                            <crate::theme::ThemeToggle tooltip_align=Align::Left />
                        </div>

                        {cart()}
                        // <WalletConnectButton />

                        <UserNavButton nav_items=navitems_user />

                    </div>
                </div>
            </div>
        </nav>
    }
}
