pub mod team;
pub mod team_management_basic;

use crate::users::UsersHeader;
use leptos::prelude::*;

#[component]
pub fn TeamList() -> impl IntoView {
    view! {
        <div>
            <UsersHeader />
            <div class="container mx-auto p-6">
                <p>"Team list coming soon..."</p>
            </div>
        </div>
    }
}

#[component]
pub fn TeamDetail() -> impl IntoView {
    view! {
        <div>
            <UsersHeader />
            <div class="container mx-auto p-6">
                <p>"Team details coming soon..."</p>
            </div>
        </div>
    }
}
