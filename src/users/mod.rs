pub mod user_list;
use leptos::prelude::{IntoView, component, view};
pub use user_list::UserList;

#[component]
pub fn UsersHeader() -> impl IntoView {
    // let nav_items = vec![
    //     // NavItem {
    //     //     name: "All Users",
    //     //     url: "/users".to_string(),
    //     // },
    //     // NavItem {
    //     //     name: "Organizations",
    //     //     url: "/users/organizations".to_string(),
    //     // },
    //     // NavItem {
    //     //     name: "Teams",
    //     //     url: "/users/teams".to_string(),
    //     // },
    // ];

    // view! { <AppHeader title="Users" nav_items=nav_items /> }
    view! { <div /> }
}

pub mod user_profile;
