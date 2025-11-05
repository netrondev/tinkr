use crate::apps::users::team::team::{CreateTeam, Team, TeamMember, TeamRole};
use leptos::prelude::*;
use crate::components::SubmitButton;
use phosphor_leptos::{CROWN, Icon, SHIELD_CHECK, USER, USER_PLUS, X};
use web_sys::SubmitEvent;

#[cfg(feature = "ssr")]
use crate::RecordId;
#[cfg(feature = "ssr")]
use crate::auth::user::AdapterUser;
#[cfg(feature = "ssr")]
use std::str::FromStr;

#[cfg(not(feature = "ssr"))]
use crate::auth::user::AdapterUser;

#[server]
pub async fn get_organization_teams(org_id: String) -> Result<Vec<Team>, ServerFnError> {
    let record_id =
        RecordId::from_str(&org_id).map_err(|_| ServerFnError::new("Invalid organization ID"))?;
    let teams = Team::get_organization_teams(record_id).await?;
    Ok(teams)
}

#[server]
pub async fn create_team(
    org_id: String,
    name: String,
    description: Option<String>,
) -> Result<Team, ServerFnError> {
    let user = crate::auth::session::get_user().await?;
    let org_record_id =
        RecordId::from_str(&org_id).map_err(|_| ServerFnError::new("Invalid organization ID"))?;

    let create_data = CreateTeam { name, description };

    let team = Team::create(create_data, org_record_id, user.id.into()).await?;
    Ok(team)
}

#[server]
pub async fn get_team_members(
    team_id: String,
) -> Result<Vec<(TeamMember, AdapterUser)>, ServerFnError> {
    let record_id =
        RecordId::from_str(&team_id).map_err(|_| ServerFnError::new("Invalid team ID"))?;
    let members = TeamMember::get_team_members(record_id).await?;
    Ok(members)
}

#[server]
pub async fn add_team_member(
    team_id: String,
    user_email: String,
    role: TeamRole,
) -> Result<TeamMember, ServerFnError> {
    let team_record_id =
        RecordId::from_str(&team_id).map_err(|_| ServerFnError::new("Invalid team ID"))?;

    // Find user by email
    let user = crate::auth::user::AdapterUser::get_by_email(user_email).await?;

    let member = TeamMember::add_member(team_record_id, user.id.into(), role).await?;
    Ok(member)
}

#[server]
pub async fn update_member_role(
    team_id: String,
    user_id: String,
    role: TeamRole,
) -> Result<TeamMember, ServerFnError> {
    let team_record_id =
        RecordId::from_str(&team_id).map_err(|_| ServerFnError::new("Invalid team ID"))?;
    let user_record_id =
        RecordId::from_str(&user_id).map_err(|_| ServerFnError::new("Invalid user ID"))?;

    let member = TeamMember::update_role(team_record_id, user_record_id, role).await?;
    Ok(member)
}

#[server]
pub async fn remove_team_member(team_id: String, user_id: String) -> Result<(), ServerFnError> {
    let team_record_id =
        RecordId::from_str(&team_id).map_err(|_| ServerFnError::new("Invalid team ID"))?;
    let user_record_id =
        RecordId::from_str(&user_id).map_err(|_| ServerFnError::new("Invalid user ID"))?;

    TeamMember::remove_member(team_record_id, user_record_id).await?;
    Ok(())
}

#[component]
pub fn TeamManagement(organization_id: String) -> impl IntoView {
    let org_id = organization_id.clone();
    let show_create_modal = RwSignal::new(false);
    let selected_team = RwSignal::new(None::<Team>);

    let teams_resource = Resource::new(
        move || org_id.clone(),
        |org_id| async move { get_organization_teams(org_id).await },
    );

    let create_team_action = Action::new(
        |(org_id, name, description): &(String, String, Option<String>)| {
            let org_id = org_id.clone();
            let name = name.clone();
            let description = description.clone();
            async move { create_team(org_id, name, description).await }
        },
    );

    Effect::new(move |_| {
        if let Some(Ok(_)) = create_team_action.value().get() {
            show_create_modal.set(false);
            teams_resource.refetch();
        }
    });

    view! {
        <div>
            <div class="flex justify-between items-center mb-4">
                <h2 class="text-lg font-semibold text-neutral-900 dark:text-white">"Teams"</h2>
                <button
                    on:click=move |_| show_create_modal.set(true)
                    class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 flex items-center"
                >
                    <span class="w-4 h-4 mr-2">
                        <Icon icon=USER_PLUS size="16" />
                    </span>
                    "Create Team"
                </button>
            </div>

            <Suspense fallback=move || {
                view! { <p class="text-neutral-600 dark:text-neutral-400">"Loading teams..."</p> }
            }>
                {move || {
                    teams_resource
                        .get()
                        .map(|teams_result| {
                            match teams_result {
                                Ok(teams) => {
                                    if teams.is_empty() {
                                        view! {
                                            <p class="text-neutral-600 dark:text-neutral-400">
                                                "No teams yet. Create your first team to get started."
                                            </p>
                                        }
                                            .into_any()
                                    } else {
                                        view! {
                                            <div class="space-y-2">
                                                <For
                                                    each=move || teams.clone()
                                                    key=|team| team.id.to_string()
                                                    children=move |team| {
                                                        let team_clone = team.clone();
                                                        view! {
                                                            <div
                                                                class="p-4 bg-neutral-50 dark:bg-neutral-900 rounded-lg cursor-pointer hover:bg-neutral-100 dark:hover:bg-neutral-800"
                                                                on:click=move |_| {
                                                                    selected_team.set(Some(team_clone.clone()))
                                                                }
                                                            >
                                                                <div class="flex justify-between items-center">
                                                                    <div>
                                                                        <h3 class="font-medium text-neutral-900 dark:text-white">
                                                                            {team.name.clone()}
                                                                        </h3>
                                                                        {team
                                                                            .description
                                                                            .clone()
                                                                            .map(|desc| {
                                                                                view! {
                                                                                    <p class="text-sm text-neutral-600 dark:text-neutral-400 mt-1">
                                                                                        {desc}
                                                                                    </p>
                                                                                }
                                                                            })}
                                                                    </div>
                                                                    <span class="text-sm text-neutral-500 dark:text-neutral-400">
                                                                        "Click to manage"
                                                                    </span>
                                                                </div>
                                                            </div>
                                                        }
                                                    }
                                                />
                                            </div>
                                        }
                                            .into_any()
                                    }
                                }
                                Err(err) => {
                                    view! {
                                        <p class="text-red-600 dark:text-red-400">
                                            "Error loading teams: " {err.to_string()}
                                        </p>
                                    }
                                        .into_any()
                                }
                            }
                        })
                }}
            </Suspense>

            <CreateTeamModal
                show=show_create_modal.into()
                organization_id=organization_id.clone()
                on_close=move || show_create_modal.set(false)
                on_create=move |(name, description)| {
                    create_team_action.dispatch((organization_id.clone(), name, description));
                }
            />

            {move || {
                selected_team
                    .get()
                    .map(|team| {
                        view! {
                            <TeamDetailModal
                                show=Signal::derive(move || selected_team.get().is_some())
                                team=team
                                on_close=move || selected_team.set(None)
                                on_update=move || teams_resource.refetch()
                            />
                        }
                    })
            }}
        </div>
    }
}

#[component]
fn CreateTeamModal<F, G>(
    show: Signal<bool>,
    organization_id: String,
    on_close: F,
    on_create: G,
) -> impl IntoView
where
    F: Fn() + 'static + Copy + Send + Sync,
    G: Fn((String, Option<String>)) + 'static + Send + Sync + Clone,
{
    let name = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());

    let handle_submit = {
        let on_create = on_create.clone();
        move |ev: SubmitEvent| {
            ev.prevent_default();
            let name_value = name.get();
            let desc_value = description.get();

            if !name_value.is_empty() {
                let desc = if desc_value.is_empty() {
                    None
                } else {
                    Some(desc_value)
                };
                on_create((name_value, desc));
            }
        }
    };

    view! {
        <Show when=move || show.get()>
            <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                <div class="bg-white dark:bg-neutral-800 rounded-lg p-6 max-w-md w-full">
                    <div class="flex justify-between items-center mb-4">
                        <h3 class="text-lg font-semibold text-neutral-900 dark:text-white">
                            "Create Team"
                        </h3>
                        <button
                            on:click=move |_| on_close()
                            class="text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-300"
                        >
                            <Icon icon=X size="20" />
                        </button>
                    </div>
                    <form on:submit=handle_submit class="space-y-4">
                        <div>
                            <label
                                for="team-name"
                                class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1"
                            >
                                "Team Name"
                            </label>
                            <input
                                type="text"
                                id="team-name"
                                class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-neutral-800 dark:text-white"
                                prop:value=move || name.get()
                                on:input=move |ev| name.set(event_target_value(&ev))
                                required
                            />
                        </div>

                        <div>
                            <label
                                for="team-description"
                                class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1"
                            >
                                "Description (Optional)"
                            </label>
                            <textarea
                                id="team-description"
                                rows="3"
                                class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-neutral-800 dark:text-white"
                                prop:value=move || description.get()
                                on:input=move |ev| description.set(event_target_value(&ev))
                            />
                        </div>

                        <div class="flex justify-end space-x-3 pt-4">
                            <button
                                type="button"
                                on:click=move |_| on_close()
                                class="px-4 py-2 border border-neutral-300 dark:border-neutral-600 text-neutral-700 dark:text-neutral-300 rounded-md hover:bg-neutral-50 dark:hover:bg-neutral-900 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                            >
                                "Cancel"
                            </button>
                            <SubmitButton text="Create Team" />
                        </div>
                    </form>
                </div>
            </div>
        </Show>
    }
}

#[component]
fn TeamDetailModal<F, G>(show: Signal<bool>, team: Team, on_close: F, on_update: G) -> impl IntoView
where
    F: Fn() + 'static + Copy + Send + Sync,
    G: Fn() + 'static + Send + Sync,
{
    let team_id = team.id.to_string();
    let show_add_member = RwSignal::new(false);

    let members_resource = Resource::new(
        move || team_id.clone(),
        |team_id| async move { get_team_members(team_id).await },
    );

    let add_member_action = Action::new(|(team_id, email, role): &(String, String, TeamRole)| {
        let team_id = team_id.clone();
        let email = email.clone();
        let role = role.clone();
        async move { add_team_member(team_id, email, role).await }
    });

    let remove_member_action = Action::new(|(team_id, user_id): &(String, String)| {
        let team_id = team_id.clone();
        let user_id = user_id.clone();
        async move { remove_team_member(team_id, user_id).await }
    });

    Effect::new(move |_| {
        if let Some(Ok(_)) = add_member_action.value().get() {
            show_add_member.set(false);
            members_resource.refetch();
        }
    });

    Effect::new(move |_| {
        if let Some(Ok(_)) = remove_member_action.value().get() {
            members_resource.refetch();
        }
    });

    view! {
        <Show when=move || show.get()>
            <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                <div class="bg-white dark:bg-neutral-800 rounded-lg p-6 max-w-2xl w-full max-h-[80vh] overflow-y-auto">
                    <div class="flex justify-between items-center mb-4">
                        <h3 class="text-lg font-semibold text-neutral-900 dark:text-white">
                            {team.name.clone()}
                        </h3>
                        <button
                            on:click=move |_| on_close()
                            class="text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-300"
                        >
                            <Icon icon=X size="20" />
                        </button>
                    </div>
                    <div class="space-y-6">
                        {team
                            .description
                            .clone()
                            .map(|desc| {
                                view! {
                                    <p class="text-neutral-600 dark:text-neutral-400">{desc}</p>
                                }
                            })}
                        <div>
                            <div class="flex justify-between items-center mb-4">
                                <h3 class="text-lg font-medium text-neutral-900 dark:text-white">
                                    "Team Members"
                                </h3>
                                <button
                                    on:click=move |_| show_add_member.set(true)
                                    class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 flex items-center text-sm"
                                >
                                    <span class="w-4 h-4 mr-2">
                                        <Icon icon=USER_PLUS size="16" />
                                    </span>
                                    "Add Member"
                                </button>
                            </div>

                            <Suspense fallback=move || {
                                view! { <p>"Loading members..."</p> }
                            }>
                                {move || {
                                    members_resource
                                        .get()
                                        .map(|members_result| {
                                            match members_result {
                                                Ok(members) => {
                                                    view! {
                                                        <div class="space-y-2">
                                                            <For
                                                                each=move || members.clone()
                                                                key=|(member, user)| member.id.to_string()
                                                                children={
                                                                    let team_id = team_id.clone();
                                                                    move |(member, user)| {
                                                                        let user_id = user.id.to_string();
                                                                        let team_id_clone = team_id.clone();
                                                                        view! {
                                                                            <div class="flex items-center justify-between p-3 bg-neutral-50 dark:bg-neutral-900 rounded-lg">
                                                                                <div class="flex items-center space-x-3">
                                                                                    <div class="w-10 h-10 bg-blue-500 rounded-full flex items-center justify-center">
                                                                                        <span class="text-white font-medium">
                                                                                            {user
                                                                                                .name
                                                                                                .chars()
                                                                                                .next()
                                                                                                .unwrap_or('U')
                                                                                                .to_uppercase()
                                                                                                .to_string()}
                                                                                        </span>
                                                                                    </div>
                                                                                    <div>
                                                                                        <p class="font-medium text-neutral-900 dark:text-white">
                                                                                            {user.name.clone()}
                                                                                        </p>
                                                                                        <p class="text-sm text-neutral-600 dark:text-neutral-400">
                                                                                            {user.email.to_string()}
                                                                                        </p>
                                                                                    </div>
                                                                                </div>
                                                                                <div class="flex items-center space-x-2">
                                                                                    <span class="text-sm text-neutral-600 dark:text-neutral-400 flex items-center">
                                                                                        {match member.role {
                                                                                            TeamRole::Owner => {
                                                                                                view! {
                                                                                                    <>
                                                                                                        <span class="w-4 h-4 mr-1 text-yellow-500">
                                                                                                            <Icon icon=CROWN size="16" />
                                                                                                        </span>
                                                                                                        "Owner"
                                                                                                    </>
                                                                                                }
                                                                                                    .into_any()
                                                                                            }
                                                                                            TeamRole::Admin => {
                                                                                                view! {
                                                                                                    <>
                                                                                                        <span class="w-4 h-4 mr-1 text-blue-500">
                                                                                                            <Icon icon=SHIELD_CHECK size="16" />
                                                                                                        </span>
                                                                                                        "Admin"
                                                                                                    </>
                                                                                                }
                                                                                                    .into_any()
                                                                                            }
                                                                                            TeamRole::Member => {
                                                                                                view! {
                                                                                                    <>
                                                                                                        <span class="w-4 h-4 mr-1 text-neutral-500">
                                                                                                            <Icon icon=USER size="16" />
                                                                                                        </span>
                                                                                                        "Member"
                                                                                                    </>
                                                                                                }
                                                                                                    .into_any()
                                                                                            }
                                                                                        }}
                                                                                    </span>
                                                                                    <Show when=move || member.role != TeamRole::Owner>
                                                                                        <button
                                                                                            on:click={
                                                                                                let team_id_clone = team_id_clone.clone();
                                                                                                let user_id = user_id.clone();
                                                                                                move |_| {
                                                                                                    remove_member_action
                                                                                                        .dispatch((team_id_clone.clone(), user_id.clone()));
                                                                                                }
                                                                                            }
                                                                                            class="p-1 text-red-600 hover:bg-red-100 dark:text-red-400 dark:hover:bg-red-900/20 rounded"
                                                                                        >
                                                                                            <Icon icon=X size="16" />
                                                                                        </button>
                                                                                    </Show>
                                                                                </div>
                                                                            </div>
                                                                        }
                                                                    }
                                                                }
                                                            />
                                                        </div>
                                                    }
                                                        .into_any()
                                                }
                                                Err(err) => {
                                                    view! {
                                                        <p class="text-red-600 dark:text-red-400">
                                                            "Error loading members: " {err.to_string()}
                                                        </p>
                                                    }
                                                        .into_any()
                                                }
                                            }
                                        })
                                }}
                            </Suspense>
                        </div>
                    </div>

                    <AddMemberModal
                        show=show_add_member.into()
                        team_id=team_id.clone()
                        on_close=move || show_add_member.set(false)
                        on_add=move |(email, role)| {
                            add_member_action.dispatch((team_id.clone(), email, role));
                        }
                    />
                </div>
            </div>
        </Show>
    }
}

#[component]
fn AddMemberModal<F, G>(
    show: Signal<bool>,
    team_id: String,
    on_close: F,
    on_add: G,
) -> impl IntoView
where
    F: Fn() + 'static + Copy + Send + Sync,
    G: Fn((String, TeamRole)) + 'static + Send + Sync + Clone,
{
    let email = RwSignal::new(String::new());
    let role = RwSignal::new(TeamRole::Member);

    let handle_submit = {
        let on_add = on_add.clone();
        move |ev: SubmitEvent| {
            ev.prevent_default();
            let email_value = email.get();

            if !email_value.is_empty() {
                on_add((email_value, role.get()));
            }
        }
    };

    view! {
        <Show when=move || show.get()>
            <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                <div class="bg-white dark:bg-neutral-800 rounded-lg p-6 max-w-md w-full">
                    <div class="flex justify-between items-center mb-4">
                        <h3 class="text-lg font-semibold text-neutral-900 dark:text-white">
                            "Add Team Member"
                        </h3>
                        <button
                            on:click=move |_| on_close()
                            class="text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-300"
                        >
                            <Icon icon=X size="20" />
                        </button>
                    </div>
                    <form on:submit=handle_submit class="space-y-4">
                        <div>
                            <label
                                for="member-email"
                                class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1"
                            >
                                "Email Address"
                            </label>
                            <input
                                type="email"
                                id="member-email"
                                class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-neutral-900 dark:text-white"
                                prop:value=move || email.get()
                                on:input=move |ev| email.set(event_target_value(&ev))
                                required
                            />
                        </div>

                        <div>
                            <label
                                for="member-role"
                                class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1"
                            >
                                "Role"
                            </label>
                            <select
                                id="member-role"
                                class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-neutral-900 dark:text-white"
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    role.set(
                                        match value.as_str() {
                                            "admin" => TeamRole::Admin,
                                            _ => TeamRole::Member,
                                        },
                                    );
                                }
                            >
                                <option value="member">"Member"</option>
                                <option value="admin">"Admin"</option>
                            </select>
                        </div>

                        <div class="flex justify-end space-x-3 pt-4">
                            <button
                                type="button"
                                on:click=move |_| on_close()
                                class="px-4 py-2 border border-neutral-300 dark:border-neutral-600 text-neutral-700 dark:text-neutral-300 rounded-md hover:bg-neutral-50 dark:hover:bg-neutral-900 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                            >
                                "Cancel"
                            </button>
                            <SubmitButton text="Add Member" />
                        </div>
                    </form>
                </div>
            </div>
        </Show>
    }
}
