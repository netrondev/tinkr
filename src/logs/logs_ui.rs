use crate::{
    AppError,
    components::{
        Button, SectionHeader, SectionStyled,
        button::{BtnColor, BtnVariant, ButtonIcon},
    },
    date_utils::FormatDatetime,
};
use leptos::prelude::*;

use super::tracing_layer::LogEvent;

use phosphor_leptos::{CARET_RIGHT, Icon};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogsFilter {
    pub level: Option<String>,
    pub target: Option<String>,
    pub search: Option<String>,
    pub limit: u32,
    pub offset: u32,
}

impl Default for LogsFilter {
    fn default() -> Self {
        Self {
            level: None,
            target: None,
            search: None,
            limit: 50,
            offset: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogsResponse {
    pub logs: Vec<LogEvent>,
    pub total_count: u64,
    pub has_more: bool,
}

#[server]
pub async fn fetch_logs(filter: LogsFilter) -> Result<LogsResponse, AppError> {
    use crate::db_init;

    let db = db_init().await?;

    // Build the query with filters
    let mut query = "SELECT * FROM log_events".to_string();
    let mut conditions = Vec::new();

    if let Some(level) = &filter.level {
        if !level.is_empty() {
            conditions.push(format!("level = '{}'", level));
        }
    }

    if let Some(target) = &filter.target {
        if !target.is_empty() {
            conditions.push(format!("target CONTAINS '{}'", target));
        }
    }

    if let Some(search) = &filter.search {
        if !search.is_empty() {
            conditions.push(format!("message CONTAINS '{}'", search));
        }
    }

    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }

    query.push_str(" ORDER BY timestamp DESC");
    query.push_str(&format!(" LIMIT {} START {}", filter.limit, filter.offset));

    let mut result = db.query(&query).await?;
    let logs: Vec<LogEvent> = result.take(0)?;

    // Get total count for pagination
    let count_query = if conditions.is_empty() {
        "SELECT count() FROM log_events GROUP ALL".to_string()
    } else {
        format!(
            "SELECT count() FROM log_events WHERE {} GROUP ALL",
            conditions.join(" AND ")
        )
    };

    let mut count_result = db.query(&count_query).await?;
    let total_count: Option<u64> = count_result.take("count")?;
    let total_count = total_count.unwrap_or(0);

    let has_more = (filter.offset + filter.limit as u32) < total_count as u32;

    Ok(LogsResponse {
        logs,
        total_count,
        has_more,
    })
}

#[server]
pub async fn get_log_stats() -> Result<Vec<LogStats>, AppError> {
    use crate::db_init;

    let db = db_init().await?;

    let stats_query = r#"
        SELECT
            level,
            count() as count
        FROM log_events
        WHERE timestamp > time::now() - 1d
        GROUP BY level
        ORDER BY count DESC
    "#;

    let mut result = db.query(stats_query).await?;
    let stats: Vec<LogStats> = result.take(0)?;

    Ok(stats)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogStats {
    level: String,
    count: u64,
}

trait LogEventColors {
    fn level_color(&self) -> &'static str;
    fn level_bg_color(&self) -> &'static str;
}

impl LogEventColors for LogEvent {
    fn level_color(&self) -> &'static str {
        match self.level.as_str() {
            "ERROR" => "text-red-500",
            "WARN" => "text-yellow-500",
            "INFO" => "text-blue-500",
            "DEBUG" => "text-neutral-500",
            "TRACE" => "text-purple-500",
            _ => "text-neutral-400",
        }
    }

    fn level_bg_color(&self) -> &'static str {
        match self.level.as_str() {
            "ERROR" => "bg-red-100 dark:bg-red-900/20",
            "WARN" => "bg-yellow-100 dark:bg-yellow-900/20",
            "INFO" => "bg-blue-100 dark:bg-blue-900/20",
            "DEBUG" => "bg-neutral-100 dark:bg-neutral-800/20",
            "TRACE" => "bg-purple-100 dark:bg-purple-900/20",
            _ => "bg-neutral-100 dark:bg-neutral-800/20",
        }
    }
}

#[component]
pub fn LogsAdmin() -> impl IntoView {
    let (filter, set_filter) = signal(LogsFilter::default());
    let (selected_log, set_selected_log) = signal(None::<LogEvent>);

    // Fetch stats resource
    let stats_resource = LocalResource::new(get_log_stats);

    // Fetch logs resource
    let logs_resource = Resource::new(
        move || filter.get(),
        |filter| async move { fetch_logs(filter).await },
    );

    let refresh_logs = move || {
        logs_resource.refetch();
        stats_resource.refetch();
    };

    let update_filter = move |new_filter: LogsFilter| {
        set_filter.set(new_filter);
    };

    let load_more = move || {
        let current = filter.get();
        let new_filter = LogsFilter {
            offset: current.offset + current.limit,
            ..current
        };
        set_filter.set(new_filter);
    };

    view! {
        <SectionStyled>
            <SectionHeader title="Log Management" subtitle="View and analyze application logs" />

            <Suspense fallback=move || {
                view! {
                    <div class="bg-white dark:bg-neutral-800 rounded-lg shadow p-6">
                        <div class="animate-pulse">
                            <div class="h-8 bg-neutral-300 dark:bg-neutral-600 rounded w-1/3 mb-6"></div>
                            <div class="space-y-4">
                                <div class="h-10 bg-neutral-200 dark:bg-neutral-700 rounded"></div>
                                <div class="h-24 bg-neutral-200 dark:bg-neutral-700 rounded"></div>
                                <div class="h-10 bg-neutral-200 dark:bg-neutral-700 rounded"></div>
                            </div>
                        </div>
                    </div>
                }
            }>

                {move || {
                    match stats_resource.get() {
                        Some(Ok(stats_data)) => {
                            view! {
                                <div class="grid grid-cols-2 md:grid-cols-5 gap-4 mb-6">
                                    {stats_data
                                        .iter()
                                        .map(|stat| {
                                            let level = stat.level.as_str();
                                            let count = stat.count;
                                            let color_class = match level {
                                                "ERROR" => "text-red-500 bg-red-100 dark:bg-red-900/20",
                                                "WARN" => {
                                                    "text-yellow-500 bg-yellow-100 dark:bg-yellow-900/20"
                                                }
                                                "INFO" => "text-blue-500 bg-blue-100 dark:bg-blue-900/20",
                                                "DEBUG" => {
                                                    "text-neutral-500 bg-neutral-100 dark:bg-neutral-800/20"
                                                }
                                                "TRACE" => {
                                                    "text-purple-500 bg-purple-100 dark:bg-purple-900/20"
                                                }
                                                _ => {
                                                    "text-neutral-400 bg-neutral-100 dark:bg-neutral-800/20"
                                                }
                                            };

                                            view! {
                                                <div class=format!("rounded-lg p-4 {}", color_class)>
                                                    <div class="text-2xl font-bold">{count}</div>
                                                    <div class="text-sm">{level}</div>
                                                </div>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            }
                                .into_any()
                        }
                        Some(Err(e)) => {
                            view! {
                                <div class="text-red-500 p-4 rounded-lg bg-red-100 dark:bg-red-900/20">
                                    "Error loading stats: " {e.to_string()}
                                </div>
                            }
                                .into_any()
                        }
                        None => {
                            view! {
                                <div class="text-center py-4">
                                    <div class="text-neutral-500 dark:text-neutral-400">
                                        "Loading stats..."
                                    </div>
                                </div>
                            }
                                .into_any()
                        }
                    }
                }}
                <LogsFilter filter=filter set_filter=update_filter on_refresh=refresh_logs />
                {move || {
                    match logs_resource.get() {
                        Some(Ok(logs_response)) => {
                            if logs_response.logs.is_empty() {
                                view! {
                                    <div class="text-center py-12">
                                        <div class="text-neutral-500 dark:text-neutral-400">
                                            "No logs found matching your criteria"
                                        </div>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <div>
                                        <LogsList
                                            logs=logs_response.logs.clone()
                                            on_select_log=set_selected_log
                                        />

                                        {if logs_response.has_more {
                                            view! {
                                                <div class="text-center mt-6">
                                                    <Button
                                                        variant=BtnVariant::Default
                                                        color=BtnColor::Neutral
                                                        on:click=move |_| load_more()
                                                    >
                                                        "Load More"
                                                    </Button>
                                                </div>
                                            }
                                                .into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }}

                                        <div class="mt-4 text-sm text-neutral-500 dark:text-neutral-400">
                                            "Showing " {logs_response.logs.len()} " of "
                                            {logs_response.total_count} " total logs"
                                        </div>
                                    </div>
                                }
                                    .into_any()
                            }
                        }
                        Some(Err(e)) => {
                            view! {
                                <div class="text-red-500 p-4 rounded-lg bg-red-100 dark:bg-red-900/20">
                                    "Error loading logs: " {e.to_string()}
                                </div>
                            }
                                .into_any()
                        }
                        None => {
                            view! {
                                <div class="text-center py-12">
                                    <div class="text-neutral-500 dark:text-neutral-400">
                                        "Loading logs..."
                                    </div>
                                </div>
                            }
                                .into_any()
                        }
                    }
                }}

            </Suspense>

            <LogDetailModal log=selected_log set_log=set_selected_log />
        </SectionStyled>
    }
}

#[component]
fn LogsFilter(
    filter: ReadSignal<LogsFilter>,
    set_filter: impl Fn(LogsFilter) + Copy + 'static,
    on_refresh: impl Fn() + Copy + 'static,
) -> impl IntoView {
    let (level_filter, set_level_filter) = signal(String::new());
    let (target_filter, set_target_filter) = signal(String::new());
    let (search_filter, set_search_filter) = signal(String::new());

    let apply_filters = move || {
        let new_filter = LogsFilter {
            level: if level_filter.get().is_empty() {
                None
            } else {
                Some(level_filter.get())
            },
            target: if target_filter.get().is_empty() {
                None
            } else {
                Some(target_filter.get())
            },
            search: if search_filter.get().is_empty() {
                None
            } else {
                Some(search_filter.get())
            },
            offset: 0, // Reset offset when applying new filters
            ..filter.get()
        };
        set_filter(new_filter);
    };

    let clear_filters = move || {
        set_level_filter.set(String::new());
        set_target_filter.set(String::new());
        set_search_filter.set(String::new());
        set_filter(LogsFilter::default());
    };

    view! {
        <div class="bg-neutral-50 dark:bg-neutral-800 rounded-lg p-4 mb-6">
            <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-4">
                <div>
                    <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">
                        "Log Level"
                    </label>
                    <select
                        class="w-full px-3 py-2  rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100"
                        on:change=move |ev| {
                            set_level_filter.set(event_target_value(&ev));
                        }
                        prop:value=move || level_filter.get()
                    >
                        <option value="">"All Levels"</option>
                        <option value="ERROR">"ERROR"</option>
                        <option value="WARN">"WARN"</option>
                        <option value="INFO">"INFO"</option>
                        <option value="DEBUG">"DEBUG"</option>
                        <option value="TRACE">"TRACE"</option>
                    </select>
                </div>

                <div>
                    <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">
                        "Target/Module"
                    </label>
                    <input
                        class="w-full px-3 py-2  rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100"
                        placeholder="e.g., app_web, app_trading"
                        prop:value=move || target_filter.get()
                        on:input=move |ev| set_target_filter.set(event_target_value(&ev))
                    />
                </div>

                <div>
                    <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">
                        "Search Message"
                    </label>
                    <input
                        class="w-full px-3 py-2  rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100"
                        placeholder="Search in log messages..."
                        prop:value=move || search_filter.get()
                        on:input=move |ev| set_search_filter.set(event_target_value(&ev))
                    />
                </div>

                <div class="flex items-end gap-2">
                    <Button
                        variant=BtnVariant::CallToAction
                        icon=ButtonIcon::Icon(phosphor_leptos::MAGNIFYING_GLASS)
                        on:click=move |_| apply_filters()
                    >
                        "Filter"
                    </Button>

                    <Button
                        variant=BtnVariant::Default
                        color=BtnColor::Neutral
                        icon=ButtonIcon::Icon(phosphor_leptos::X)
                        on:click=move |_| clear_filters()
                    >
                        "Clear"
                    </Button>
                </div>
            </div>

            <div class="flex gap-2">
                <Button
                    variant=BtnVariant::Default
                    color=BtnColor::Neutral
                    icon=ButtonIcon::Icon(phosphor_leptos::ARROW_CLOCKWISE)
                    on:click=move |_| on_refresh()
                >
                    "Refresh"
                </Button>
            </div>
        </div>
    }
}

#[component]
fn LogsList(logs: Vec<LogEvent>, on_select_log: WriteSignal<Option<LogEvent>>) -> impl IntoView {
    view! {
        <div class="space-y-2">
            {logs
                .into_iter()
                .map(|log| {
                    view! {
                        <LogEntryCard
                            log=log.clone()
                            on_select=move |log_entry: LogEvent| on_select_log.set(Some(log_entry))
                        />
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn LogEntryCard(log: LogEvent, on_select: impl Fn(LogEvent) + 'static) -> impl IntoView {
    let timestamp_str = log.timestamp.ago();
    let location = if let (Some(file), Some(line)) = (&log.file, log.line) {
        format!("{}:{}", file, line)
    } else if let Some(module) = &log.module_path {
        module.clone()
    } else {
        "unknown".to_string()
    };

    view! {
        <div
            class=format!(
                "p-4 rounded-lg cursor-pointer hover:bg-neutral-200 hover:dark:bg-neutral-900 {}",
                log.level_bg_color(),
            )
            on:click={
                let log_clone = log.clone();
                move |_| on_select(log_clone.clone())
            }
        >
            <div class="flex items-start justify-between">
                <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-2 mb-1">
                        <span class=format!(
                            "text-xs px-2 py-1 rounded font-medium {}",
                            log.level_color(),
                        )>{log.level.clone()}</span>
                        <span class="text-xs text-neutral-500 dark:text-neutral-400">
                            {timestamp_str}
                        </span>
                        <span class="text-xs text-neutral-500 dark:text-neutral-400">
                            {location}
                        </span>
                    </div>

                    <div class="text-sm text-neutral-600 dark:text-neutral-400 mb-1">
                        {log.target.clone()}
                    </div>

                    <div class="text-sm text-neutral-900 dark:text-neutral-100 break-words">
                        {if log.message.len() > 200 {
                            format!("{}...", &log.message[..200])
                        } else {
                            log.message.clone()
                        }}
                    </div>
                </div>

                <div class="ml-2">
                    <Icon icon=CARET_RIGHT size="16px" />
                </div>
            </div>
        </div>
    }
}

#[component]
fn LogDetailModal(
    log: ReadSignal<Option<LogEvent>>,
    set_log: WriteSignal<Option<LogEvent>>,
) -> impl IntoView {
    view! {
        {move || {
            log.get()
                .map(|log_entry| {
                    let close = move || set_log.set(None);
                    let timestamp_str = log_entry.timestamp.ago();

                    view! {
                        <div class="fixed inset-0 z-50 flex items-center justify-center">
                            <div
                                class="fixed inset-0 bg-black bg-opacity-50"
                                on:click=move |_| close()
                            ></div>

                            <div class="relative bg-white dark:bg-neutral-800 rounded-lg shadow-xl max-w-4xl w-full mx-4 max-h-[90vh] overflow-hidden">
                                <div class="flex items-center justify-between p-4 border-b border-neutral-200 dark:border-neutral-700">
                                    <h3 class="text-lg font-semibold text-neutral-900 dark:text-neutral-100">
                                        "Log Details"
                                    </h3>
                                    <Button
                                        variant=BtnVariant::Default
                                        color=BtnColor::Neutral
                                        icon=ButtonIcon::Icon(phosphor_leptos::X)
                                        on:click=move |_| close()
                                    >
                                        ""
                                    </Button>
                                </div>

                                <div class="p-4 overflow-y-auto max-h-[calc(90vh-120px)]">
                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                                        <div>
                                            <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300">
                                                "Timestamp"
                                            </label>
                                            <div class="mt-1 text-sm text-neutral-900 dark:text-neutral-100">
                                                {timestamp_str}
                                            </div>
                                        </div>

                                        <div>
                                            <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300">
                                                "Level"
                                            </label>
                                            <div class=format!(
                                                "mt-1 text-sm font-medium {}",
                                                log_entry.level_color(),
                                            )>{log_entry.level.clone()}</div>
                                        </div>

                                        <div>
                                            <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300">
                                                "Target"
                                            </label>
                                            <div class="mt-1 text-sm text-neutral-900 dark:text-neutral-100">
                                                {log_entry.target.clone()}
                                            </div>
                                        </div>

                                        <div>
                                            <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300">
                                                "Location"
                                            </label>
                                            <div class="mt-1 text-sm text-neutral-900 dark:text-neutral-100">
                                                {if let (Some(file), Some(line)) = (
                                                    &log_entry.file,
                                                    log_entry.line,
                                                ) {
                                                    format!("{}:{}", file, line)
                                                } else if let Some(module) = &log_entry.module_path {
                                                    module.clone()
                                                } else {
                                                    "unknown".to_string()
                                                }}
                                            </div>
                                        </div>
                                    </div>

                                    <div class="mb-4">
                                        <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-2">
                                            "Message"
                                        </label>
                                        <div class="bg-neutral-50 dark:bg-neutral-900 rounded-md p-4 text-sm text-neutral-900 dark:text-neutral-100 font-mono whitespace-pre-wrap">
                                            {log_entry.message.clone()}
                                        </div>
                                    </div>

                                    {if !log_entry.fields.is_null()
                                        && log_entry
                                            .fields
                                            .as_object()
                                            .map_or(false, |obj| !obj.is_empty())
                                    {
                                        view! {
                                            <div>
                                                <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-2">
                                                    "Additional Fields"
                                                </label>
                                                <div class="bg-neutral-50 dark:bg-neutral-900 rounded-md p-4 text-sm text-neutral-900 dark:text-neutral-100 font-mono">
                                                    <pre>
                                                        {serde_json::to_string_pretty(&log_entry.fields)
                                                            .unwrap_or_default()}
                                                    </pre>
                                                </div>
                                            </div>
                                        }
                                            .into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }}
                                </div>
                            </div>
                        </div>
                    }
                })
        }}
    }
}
