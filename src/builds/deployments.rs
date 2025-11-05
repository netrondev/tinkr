use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentStatus {
    InProgress,
    Queued,
    Building,
    Success,
    Failed,
    Cancelled,
}

impl DeploymentStatus {
    pub fn color(&self) -> &'static str {
        match self {
            DeploymentStatus::Success => "text-green-500",
            DeploymentStatus::Failed => "text-red-500",
            DeploymentStatus::InProgress | DeploymentStatus::Building => "text-blue-500",
            DeploymentStatus::Queued => "text-yellow-500",
            DeploymentStatus::Cancelled => "text-neutral-500",
        }
    }

    pub fn bg_color(&self) -> &'static str {
        match self {
            DeploymentStatus::Success => "bg-green-950/30",
            DeploymentStatus::Failed => "bg-red-950/30",
            DeploymentStatus::InProgress | DeploymentStatus::Building => "bg-blue-950/30",
            DeploymentStatus::Queued => "bg-yellow-950/30",
            DeploymentStatus::Cancelled => "bg-neutral-950/30",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Deployment {
    pub id: i64,
    pub application_id: String,
    pub application_name: String,
    pub build_server_id: Option<i64>,
    pub commit: String,
    pub commit_message: Option<String>,
    pub current_process_id: Option<String>,
    pub deployment_url: String,
    pub deployment_uuid: String,
    pub destination_id: String,
    pub finished_at: Option<String>,
    pub force_rebuild: bool,
    pub git_type: Option<String>,
    pub horizon_job_id: String,
    pub horizon_job_worker: String,
    pub is_api: bool,
    pub is_webhook: bool,
    pub only_this_server: bool,
    pub pull_request_id: i64,
    pub restart_only: bool,
    pub rollback: bool,
    pub server_id: i64,
    pub server_name: String,
    pub status: DeploymentStatus,
    pub created_at: String,
    pub updated_at: String,
}

impl Deployment {
    pub fn short_commit(&self) -> String {
        self.commit.chars().take(7).collect()
    }

    pub fn trigger_type(&self) -> &'static str {
        if self.is_webhook {
            "Webhook"
        } else if self.is_api {
            "API"
        } else {
            "Manual"
        }
    }

    pub fn duration(&self) -> Option<String> {
        use chrono::{DateTime, Utc};

        let started = DateTime::parse_from_rfc3339(&self.created_at).ok()?;
        let ended = if let Some(ref finished) = self.finished_at {
            DateTime::parse_from_rfc3339(finished).ok()?
        } else {
            Utc::now().into()
        };

        let duration = ended.signed_duration_since(started);
        let minutes = duration.num_minutes();
        let seconds = duration.num_seconds() % 60;

        Some(format!("{:02}m {:02}s", minutes, seconds))
    }

    pub fn finished_ago(&self) -> Option<String> {
        use chrono::{DateTime, Utc};

        let finished = self.finished_at.as_ref()?;
        let finished_time = DateTime::parse_from_rfc3339(finished).ok()?;
        let now = Utc::now();
        let duration = now.signed_duration_since(finished_time);

        let hours = duration.num_hours();
        if hours > 0 {
            Some(format!("Finished {} hours ago", hours))
        } else {
            let minutes = duration.num_minutes();
            Some(format!("Finished {} minutes ago", minutes))
        }
    }
}

use leptos::prelude::*;

#[server]
pub async fn get_deployments() -> Result<Vec<Deployment>, ServerFnError> {
    use http::header::ACCEPT;
    use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

    let api_url = std::env::var("COOLIFY_API_URL").unwrap_or_default();
    let api_token = std::env::var("COOLIFY_API_TOKEN").unwrap_or_default();

    let client = reqwest::Client::new();
    let url = format!("{api_url}/api/v1/deployments");

    let response = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", api_token))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if response.status().is_success() {
        let deployments = response
            .json::<Vec<Deployment>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(deployments)
    } else {
        Err(ServerFnError::new(format!(
            "Failed to fetch deployments: {}",
            response.status()
        )))
    }
}

#[component]
pub fn DeploymentStatusComponent() -> impl IntoView {
    let deployments = Resource::new(|| (), |_| async { get_deployments().await });

    view! {
        <div class="space-y-4">
            <Suspense fallback=move || {
                view! { <div class="text-neutral-400">"Loading deployments..."</div> }
            }>
                {move || {
                    deployments
                        .get()
                        .map(|result| match result {
                            Ok(deployments) => {
                                view! {
                                    <div class="space-y-4">
                                        {deployments
                                            .into_iter()
                                            .take(10)
                                            .map(|deployment| {
                                                view! { <DeploymentCard deployment=deployment /> }
                                            })
                                            .collect_view()}
                                    </div>
                                }
                                    .into_any()
                            }
                            Err(e) => {
                                view! {
                                    <div class="text-red-500">
                                        "Error loading deployments: " {e.to_string()}
                                    </div>
                                }
                                    .into_any()
                            }
                        })
                }}

            </Suspense>
        </div>
    }
}

#[component]
fn DeploymentCard(deployment: Deployment) -> impl IntoView {
    let status_color = deployment.status.color();
    let bg_color = deployment.status.bg_color();
    let status_text = format!("{:?}", deployment.status);

    view! {
        <div class=format!(
            "border border-neutral-800 rounded-lg p-4 {} {}",
            bg_color,
            status_color,
        )>
            <div class="flex items-start justify-between mb-2">
                <div class="font-semibold">{status_text}</div>
            </div>

            <div class="text-sm text-neutral-400 space-y-1">
                <div>"Started: " {deployment.created_at.clone()}</div>
                {deployment
                    .finished_at
                    .as_ref()
                    .map(|finished| {
                        view! { <div>"Ended: " {finished.clone()}</div> }
                    })}

                {deployment
                    .duration()
                    .map(|dur| {
                        view! { <div>"Duration: " {dur}</div> }
                    })}

                {deployment
                    .finished_ago()
                    .map(|ago| {
                        view! { <div>{ago}</div> }
                    })}

                <div>
                    "Commit: "
                    <a
                        href=format!("https://github.com/{}", deployment.commit)
                        class="text-blue-400 hover:underline"
                        target="_blank"
                    >
                        {deployment.short_commit()}
                    </a>
                    {deployment
                        .commit_message
                        .as_ref()
                        .map(|msg| {
                            view! { <span>" - " {msg.clone()}</span> }
                        })}

                </div>

                <div>
                    <span class="inline-block px-2 py-0.5 rounded bg-neutral-800 text-xs">
                        {deployment.trigger_type()}
                    </span>
                </div>
            </div>
        </div>
    }
}

// // Example with polling
// use tokio::time::{Duration, sleep};

// async fn poll_until_complete(
//     api_url: &str,
//     api_token: &str,
//     application_id: &str,
//     deployment_id: &str,
// ) -> Result<Deployment> {
//     loop {
//         let deployment =
//             get_deployment_status(api_url, api_token, application_id, deployment_id).await?;

//         match deployment.status {
//             DeploymentStatus::Success | DeploymentStatus::Failed | DeploymentStatus::Cancelled => {
//                 return Ok(deployment);
//             }
//             _ => {
//                 println!("Status: {:?}, waiting...", deployment.status);
//                 sleep(Duration::from_secs(5)).await;
//             }
//         }
//     }
// }
