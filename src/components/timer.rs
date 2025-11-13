use leptos::prelude::*;
use std::time::Duration;

#[cfg(not(feature = "ssr"))]
use crate::Datetime;
#[cfg(feature = "ssr")]
use surrealdb::Datetime;

#[component]
pub fn Timer(
    #[prop(optional)] start_time: Option<Datetime>,
    #[prop(default = false)] is_running: bool,
) -> impl IntoView {
    // Create a reactive signal to store the elapsed seconds
    let elapsed_seconds = RwSignal::new(0i64);

    // Calculate initial elapsed time if start_time is provided and timer is running
    let initial_elapsed = if is_running {
        if let Some(start) = start_time {
            #[cfg(not(feature = "ssr"))]
            {
                let start_timestamp = start.inner().timestamp();
                let now_timestamp = chrono::Utc::now().timestamp();
                (now_timestamp - start_timestamp).max(0)
            }
            #[cfg(feature = "ssr")]
            {
                let date_str = start.to_string();
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_str) {
                    let start_timestamp = dt.timestamp();
                    let now_timestamp = chrono::Utc::now().timestamp();
                    (now_timestamp - start_timestamp).max(0)
                } else {
                    0
                }
            }
        } else {
            0
        }
    } else {
        0
    };

    // Set initial elapsed time
    elapsed_seconds.set(initial_elapsed);

    // Set up an interval to increment seconds every 1000ms
    if is_running {
        Effect::new(move |_| {
            let _ = set_interval_with_handle(
                move || {
                    elapsed_seconds.update(|n| *n += 1);
                },
                Duration::from_secs(1),
            );
        });
    }

    // Format the elapsed time
    let formatted_time = move || {
        let total_seconds = elapsed_seconds.get();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    };

    // Render the timer
    view! {
        <span class="font-mono">
            {move || if is_running { formatted_time() } else { String::new() }}
        </span>
    }
}
