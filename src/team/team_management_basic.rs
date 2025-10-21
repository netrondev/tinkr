use crate::team::team::{Team, TeamMember, TeamRole};
use leptos::prelude::*;

use crate::user::AdapterUser;

#[cfg(feature = "ssr")]
use surrealdb::RecordId;

#[cfg(feature = "ssr")]
use std::str::FromStr;

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
    let user = crate::session::get_user().await?;
    let org_record_id =
        RecordId::from_str(&org_id).map_err(|_| ServerFnError::new("Invalid organization ID"))?;

    use crate::team::team::CreateTeam;
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

#[component]
pub fn TeamManagement(organization_id: String) -> impl IntoView {
    let org_id_for_resource = organization_id.clone();
    let teams_resource = Resource::new(
        move || org_id_for_resource.clone(),
        |org_id| async move { get_organization_teams(org_id).await },
    );

    view! {
        <div>
            <h2 class="text-lg font-semibold text-neutral-900 dark:text-white mb-4">
                "Teams"
            </h2>

            <Suspense fallback=move || view! { <p class="text-neutral-600 dark:text-neutral-400">"Loading teams..."</p> }>
                {move || {
                    teams_resource.get().map(|teams_result| {
                        match teams_result {
                            Ok(teams) => {
                                if teams.is_empty() {
                                    view! {
                                        <div>
                                            <p class="text-neutral-600 dark:text-neutral-400 mb-4">
                                                "No teams yet."
                                            </p>
                                            <CreateTeamSection organization_id=organization_id.clone() />
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div>
                                            <div class="mb-4">
                                                <CreateTeamSection organization_id=organization_id.clone() />
                                            </div>
                                            <div class="space-y-2">
                                                <For
                                                    each=move || teams.clone()
                                                    key=|team| team.id.to_string()
                                                    children=move |team| {
                                                        view! {
                                                            <TeamCard team=team />
                                                        }
                                                    }
                                                />
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                            }
                            Err(err) => view! {
                                <p class="text-red-600 dark:text-red-400">
                                    "Error loading teams: " {err.to_string()}
                                </p>
                            }.into_any()
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn CreateTeamSection(organization_id: String) -> impl IntoView {
    let create_action = Action::new(
        |(org_id, name, description): &(String, String, Option<String>)| {
            let org_id = org_id.clone();
            let name = name.clone();
            let description = description.clone();
            async move { create_team(org_id, name, description).await }
        },
    );

    let name = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());

    view! {
        <div class="bg-neutral-50 dark:bg-neutral-900 p-4 rounded-lg">
            <h3 class="font-medium text-neutral-900 dark:text-white mb-3">
                "Create New Team"
            </h3>
            <form on:submit=move |ev: web_sys::SubmitEvent| {
                ev.prevent_default();
                let name_value = name.get();
                let desc_value = description.get();
                if !name_value.is_empty() {
                    let desc = if desc_value.is_empty() { None } else { Some(desc_value) };
                    create_action.dispatch((organization_id.clone(), name_value, desc));
                }
            }>
                <div class="space-y-3">
                    <div>
                        <input
                            type="text"
                            name="name"
                            placeholder="Team name"
                            class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-neutral-800 dark:text-white"
                            prop:value=move || name.get()
                            on:input=move |ev| name.set(event_target_value(&ev))
                            required
                        />
                    </div>
                    <div>
                        <input
                            type="text"
                            name="description"
                            placeholder="Description (optional)"
                            class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-neutral-800 dark:text-white"
                            prop:value=move || description.get()
                            on:input=move |ev| description.set(event_target_value(&ev))
                        />
                    </div>
                    <button
                        type="submit"
                        class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                    >
                        "Create Team"
                    </button>
                </div>
            </form>
        </div>
    }
}

#[component]
fn TeamCard(team: Team) -> impl IntoView {
    let show_members = RwSignal::new(false);
    let team_id = team.id.to_string();

    view! {
        <div class="bg-white dark:bg-neutral-800 rounded-lg p-4 shadow">
            <div class="flex justify-between items-start">
                <div>
                    <h3 class="font-medium text-neutral-900 dark:text-white">
                        {team.name.clone()}
                    </h3>
                    {team.description.clone().map(|desc| view! {
                        <p class="text-sm text-neutral-600 dark:text-neutral-400 mt-1">
                            {desc}
                        </p>
                    })}
                </div>
                <button
                    on:click=move |_| show_members.update(|v| *v = !*v)
                    class="text-sm text-blue-600 dark:text-blue-400 hover:underline"
                >
                    {move || if show_members.get() { "Hide Members" } else { "Show Members" }}
                </button>
            </div>

            <Show when=move || show_members.get()>
                <div class="mt-4 border-t border-neutral-200 dark:border-neutral-700 pt-4">
                    <TeamMembersList team_id=team_id.clone() />
                </div>
            </Show>
        </div>
    }
}

#[component]
fn TeamMembersList(team_id: String) -> impl IntoView {
    let members_resource = Resource::new(
        move || team_id.clone(),
        |team_id| async move { get_team_members(team_id).await },
    );

    view! {
        <Suspense fallback=move || view! { <p class="text-sm text-neutral-500">"Loading members..."</p> }>
            {move || {
                members_resource.get().map(|members_result| {
                    match members_result {
                        Ok(members) => {
                            if members.is_empty() {
                                view! {
                                    <p class="text-sm text-neutral-500">
                                        "No members in this team."
                                    </p>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-2">
                                        <For
                                            each=move || members.clone()
                                            key=|(member, _)| member.id.to_string()
                                            children=move |(member, user)| {
                                                view! {
                                                    <div class="flex items-center justify-between text-sm">
                                                        <div>
                                                            <span class="font-medium text-neutral-900 dark:text-white">
                                                                {user.name.clone()}
                                                            </span>
                                                            <span class="text-neutral-600 dark:text-neutral-400 ml-2">
                                                                {user.email.to_string()}
                                                            </span>
                                                        </div>
                                                        <span class="text-xs text-neutral-500 dark:text-neutral-400">
                                                            {match member.role {
                                                                TeamRole::Owner => "Owner",
                                                                TeamRole::Admin => "Admin",
                                                                TeamRole::Member => "Member",
                                                            }}
                                                        </span>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
                                }.into_any()
                            }
                        }
                        Err(err) => view! {
                            <p class="text-sm text-red-600 dark:text-red-400">
                                "Error loading members: " {err.to_string()}
                            </p>
                        }.into_any()
                    }
                })
            }}
        </Suspense>
    }
}
