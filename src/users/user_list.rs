// use crate::{apps::users::UsersHeader, auth::user::AdapterUser};
// use app_core::ui_auth::get_users;
use crate::components::UserAvatar;
use leptos::prelude::*;

use crate::{ui_auth::get_users, user::AdapterUser, users::UsersHeader};

#[component]
pub fn UserList() -> impl IntoView {
    let users_resource = Resource::new(|| (), |_| async move { get_users().await });

    view! {
        <div>
            <UsersHeader />
            <div class="container mx-auto p-6">

            <Suspense fallback=move || view! { <p class="text-neutral-600 dark:text-neutral-400">"Loading users..."</p> }>
                {move || {
                    users_resource.get().map(|users_result| {
                        match users_result {
                            Ok(users) => view! {
                                // Mobile view: Card layout
                                <div class="block lg:hidden space-y-4">
                                    {users.clone().into_iter().map(|user| view! {
                                        <UserCard user=user />
                                    }).collect::<Vec<_>>()}
                                </div>

                                // Desktop view: Table layout
                                <div class="hidden lg:block bg-white dark:bg-neutral-800 shadow-md dark:shadow-neutral-700/50 rounded-lg overflow-hidden">
                                    <table class="min-w-full divide-y divide-neutral-200 dark:divide-neutral-700">
                                        <thead class="bg-neutral-50 dark:bg-neutral-900">
                                            <tr>
                                                <th class="px-6 py-3 text-left text-xs font-medium text-neutral-500 dark:text-neutral-400 uppercase tracking-wider">
                                                    "User"
                                                </th>
                                                <th class="px-6 py-3 text-left text-xs font-medium text-neutral-500 dark:text-neutral-400 uppercase tracking-wider">
                                                    "Email"
                                                </th>
                                                <th class="px-6 py-3 text-left text-xs font-medium text-neutral-500 dark:text-neutral-400 uppercase tracking-wider">
                                                    "Verified"
                                                </th>
                                                <th class="px-6 py-3 text-left text-xs font-medium text-neutral-500 dark:text-neutral-400 uppercase tracking-wider">
                                                    "Admin"
                                                </th>
                                            </tr>
                                        </thead>
                                        <tbody class="bg-white dark:bg-neutral-800 divide-y divide-neutral-200 dark:divide-neutral-700">
                                            {users.into_iter().map(|user| view! {
                                                <UserRow user=user />
                                            }).collect::<Vec<_>>()}
                                        </tbody>
                                    </table>
                                </div>
                            }.into_any(),
                            Err(err) => view! {
                                <div class="bg-red-100 dark:bg-red-900/20 border border-red-400 dark:border-red-600 text-red-700 dark:text-red-400 px-4 py-3 rounded">
                                    <p>"Error loading users: " {err.to_string()}</p>
                                </div>
                            }.into_any()
                        }
                    })
                }}
            </Suspense>
            </div>
        </div>
    }
}

#[component]
fn UserCard(user: AdapterUser) -> impl IntoView {
    let user_name = user.name.clone();
    let user_image = user.image.clone();

    view! {
        <div class="bg-white dark:bg-neutral-800 shadow-md dark:shadow-neutral-700/50 rounded-lg p-4 sm:p-6">
            <div class="flex items-start space-x-4">
                <div class="flex-shrink-0">
                    <UserAvatar
                        name=Some(user_name.clone())
                        image=user_image
                        size="lg"
                    />
                </div>
                <div class="flex-1 min-w-0">
                    <div class="text-base sm:text-lg font-medium text-neutral-900 dark:text-neutral-100 truncate">
                        {user_name}
                    </div>
                    <div class="text-sm text-neutral-500 dark:text-neutral-400 truncate">
                        {user.email.0}
                    </div>
                    <div class="mt-2 flex flex-wrap gap-2">
                        {if user.email_verified.is_some() {
                            view! { <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 dark:bg-green-900/30 text-green-800 dark:text-green-400">
                                "✓ Verified"
                            </span> }.into_any()
                        } else {
                            view! { <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 dark:bg-red-900/30 text-red-800 dark:text-red-400">
                                "✗ Not verified"
                            </span> }.into_any()
                        }}
                        {if user.superadmin.unwrap_or(false) {
                            view! { <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 dark:bg-blue-900/30 text-blue-800 dark:text-blue-400">
                                "Admin"
                            </span> }.into_any()
                        } else {
                            view! { <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-neutral-100 dark:bg-neutral-700 text-neutral-600 dark:text-neutral-400">
                                "User"
                            </span> }.into_any()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn UserRow(user: AdapterUser) -> impl IntoView {
    let user_name = user.name.clone();
    let user_image = user.image.clone();

    view! {
        <tr class="hover:bg-neutral-50 dark:hover:bg-neutral-700/50">
            <td class="px-6 py-4 whitespace-nowrap">
                <div class="flex items-center">
                    <div class="flex-shrink-0">
                        <UserAvatar
                            name=Some(user_name.clone())
                            image=user_image
                            size="md"
                        />
                    </div>
                    <div class="ml-4">
                        <div class="text-sm font-medium text-neutral-900 dark:text-neutral-100">
                            {user_name}
                        </div>
                    </div>
                </div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-neutral-500 dark:text-neutral-400">
                {user.email.0}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm">
                {if user.email_verified.is_some() {
                    view! { <span class="text-green-600 dark:text-green-400">"✓ Verified"</span> }.into_any()
                } else {
                    view! { <span class="text-red-600 dark:text-red-400">"✗ Not verified"</span> }.into_any()
                }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm">
                {if user.superadmin.unwrap_or(false) {
                    view! { <span class="text-blue-600 dark:text-blue-400 font-medium">"Admin"</span> }.into_any()
                } else {
                    view! { <span class="text-neutral-400 dark:text-neutral-500">"User"</span> }.into_any()
                }}
            </td>
        </tr>
    }
}
