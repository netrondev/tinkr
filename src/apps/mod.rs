use phosphor_leptos::{
    BRIEFCASE, CHART_LINE_UP, CHAT_CIRCLE_DOTS, CLOCK_USER, CUBE, CURRENCY_ETH, GAUGE, GEAR, GLOBE,
    INVOICE, PLAY, SWATCHES, TARGET, USERS, VIDEO, WALLET,
};
pub mod settings;
pub mod users;
use crate::{
    colors::Color,
    components::{
        button::ButtonIcon,
        sidebar::{NavBarLink, SidebarItem},
    },
};
use leptos::prelude::*;

pub fn get_navbar_list() -> Vec<SidebarItem> {
    vec![
        SidebarItem::Link(NavBarLink {
            name: "Trading".to_string(),
            icon: ButtonIcon::Icon(CHART_LINE_UP),
            icon_hover: None,
            background_color: Color::from_tailwind("amber-600"),
            url: "/trading".to_string(),
        }),
        SidebarItem::Link(NavBarLink {
            name: "Wallets".to_string(),
            icon: ButtonIcon::Icon(WALLET),
            icon_hover: None,
            background_color: Color::from_tailwind("purple-600"),
            url: "/wallets".to_string(),
        }),
        SidebarItem::Gap,
        SidebarItem::Divider,
        SidebarItem::Link(NavBarLink {
            name: "Settings".to_string(),
            icon: ButtonIcon::Icon(GEAR),
            icon_hover: None,
            background_color: Color::from_tailwind("teal-600"),
            url: "/settings".to_string(),
        }),
    ]
}
