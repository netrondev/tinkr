use std::str::FromStr;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use tw_merge::tw_merge;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Error,
    Warning,
    Info,
    Success,
}

impl Default for AlertSeverity {
    fn default() -> Self {
        AlertSeverity::Info
    }
}

impl ToString for AlertSeverity {
    fn to_string(&self) -> String {
        match self {
            AlertSeverity::Error => "Error".to_string(),
            AlertSeverity::Warning => "Warning".to_string(),
            AlertSeverity::Info => "Info".to_string(),
            AlertSeverity::Success => "Success".to_string(),
        }
    }
}

impl FromStr for AlertSeverity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Error" => Ok(AlertSeverity::Error),
            "Warning" => Ok(AlertSeverity::Warning),
            "Info" => Ok(AlertSeverity::Info),
            "Success" => Ok(AlertSeverity::Success),
            _ => Err(()),
        }
    }
}

#[component]
pub fn Alert(
    #[prop(optional)] children: Option<Children>,
    severity: AlertSeverity,
    #[prop(default = "")] class: &'static str,
) -> impl IntoView {
    let (bg_color, text_color, icon_color, icon_svg) = match severity {
        AlertSeverity::Success => (
            "bg-green-50 dark:bg-green-900/20 dark:border dark:border-green-800/30",
            "text-green-800 dark:text-green-100",
            "text-green-400 dark:text-green-400",
            view! {
                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                    <path
                        fill-rule="evenodd"
                        d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                        clip-rule="evenodd"
                    />
                </svg>
            },
        ),
        AlertSeverity::Error => (
            "bg-red-50 dark:bg-red-900/20 dark:border dark:border-red-800/30",
            "text-red-800 dark:text-red-100",
            "text-red-400 dark:text-red-400",
            view! {
                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                    <path
                        fill-rule="evenodd"
                        d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                        clip-rule="evenodd"
                    />
                </svg>
            },
        ),
        AlertSeverity::Warning => (
            "bg-yellow-50 dark:bg-yellow-900/20 dark:border dark:border-yellow-800/30",
            "text-yellow-800 dark:text-yellow-100",
            "text-yellow-400 dark:text-yellow-400",
            view! {
                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                    <path
                        fill-rule="evenodd"
                        d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                        clip-rule="evenodd"
                    />
                </svg>
            },
        ),
        AlertSeverity::Info => (
            "bg-blue-50 dark:bg-blue-900/20 dark:border dark:border-blue-800/30",
            "text-blue-800 dark:text-blue-100",
            "text-blue-400 dark:text-blue-400",
            view! {
                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                    <path
                        fill-rule="evenodd"
                        d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
                        clip-rule="evenodd"
                    />
                </svg>
            },
        ),
    };

    let combined_class = tw_merge!("rounded-md p-4 w-full", bg_color, class);

    view! {
        <div class=combined_class>
            <div class="flex">
                <div class="flex-shrink-0">
                    <div class=format!("{icon_color}")>{icon_svg}</div>
                </div>
                <div class="ml-3">
                    <p class=tw_merge!(
                        "text-sm font-medium {}", text_color
                    )>
                        {match children {
                            Some(children) => children(),
                            None => view! {}.into_any(),
                        }}
                    </p>
                </div>
            </div>
        </div>
    }
}
