use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use phosphor_leptos::SUN_HORIZON;
use std::time::Duration;

use crate::components::{
    Align, Button, Tooltip,
    button::{BtnVariant, ButtonIcon},
};

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    System,
    Unknown,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
            _ => "unknown",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "dark" => Theme::Dark,
            "light" => Theme::Light,
            "system" => Theme::System,
            _ => Theme::Unknown,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::System
    }
}

#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub theme: RwSignal<Theme>,
    pub system_prefers_dark: RwSignal<bool>,
    pub current_theme: RwSignal<Theme>,
}

impl ThemeContext {
    pub fn new() -> Self {
        let current_theme = RwSignal::new(Theme::Unknown);
        // Check localStorage on client side
        #[cfg(feature = "hydrate")]
        let initial_theme = {
            use web_sys::window;
            if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
                storage
                    .get_item("theme")
                    .ok()
                    .flatten()
                    .map(|s| Theme::from_str(s.as_str()))
                    .unwrap_or(Theme::System)
            } else {
                Theme::System
            }
        };

        #[cfg(not(feature = "hydrate"))]
        let initial_theme = Theme::System;

        let theme = RwSignal::new(initial_theme);
        let system_prefers_dark = RwSignal::new(false);

        // Detect system preference and listen for changes
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::JsCast;
            use wasm_bindgen::prelude::*;
            use web_sys::{Event, window};

            if let Some(window) = window() {
                if let Ok(prefers_dark_media) = window.match_media("(prefers-color-scheme: dark)") {
                    if let Some(media_query) = prefers_dark_media {
                        system_prefers_dark.set(media_query.matches());

                        // Set up listener for system theme changes
                        let system_dark_signal = system_prefers_dark;
                        let media_query_clone = media_query.clone();
                        let callback = Closure::<dyn Fn(Event)>::new(move |_event: Event| {
                            system_dark_signal.set(media_query_clone.matches());
                        });

                        let _ = media_query.add_event_listener_with_callback(
                            "change",
                            callback.as_ref().unchecked_ref(),
                        );
                        callback.forget(); // Keep the closure alive
                    }
                }
            }
        }

        // Set up effect to save theme to localStorage
        #[cfg(feature = "hydrate")]
        {
            use web_sys::window;
            let theme_effect = theme;
            let system_dark = system_prefers_dark;
            Effect::new(move |_| {
                let current_theme = theme_effect.get();
                let prefers_dark = system_dark.get();

                if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
                    let _ = storage.set_item("theme", current_theme.as_str());
                }

                // Determine effective theme
                let effective_theme = match current_theme {
                    Theme::System => {
                        if prefers_dark {
                            Theme::Dark
                        } else {
                            Theme::Light
                        }
                    }
                    _ => current_theme,
                };

                // Update document class
                if let Some(doc) = window().and_then(|w| w.document()) {
                    if let Some(doc_element) = doc.document_element() {
                        let class_list = doc_element.class_list();
                        match effective_theme {
                            Theme::Dark => {
                                let _ = class_list.add_1("dark");
                                let _ = class_list.remove_1("light");
                            }
                            Theme::Light | Theme::System => {
                                let _ = class_list.remove_1("dark");
                                let _ = class_list.add_1("light");
                            }
                            Theme::Unknown => { /* Do nothing */ }
                        }
                    }
                }
            });
        }

        Effect::new(move |_| {
            let _ = set_interval_with_handle(
                move || {
                    let asd = ThemeContext::get_current_theme();
                    current_theme.set(asd);
                },
                Duration::from_millis(100),
            );
        });

        ThemeContext {
            theme,
            system_prefers_dark,
            current_theme,
        }
    }

    pub fn toggle(&self) {
        self.theme.update(|t| {
            *t = match *t {
                Theme::Light => {
                    if self.system_prefers_dark.get() {
                        Theme::System
                    } else {
                        Theme::Dark
                    }
                }
                Theme::Dark => {
                    if self.system_prefers_dark.get() {
                        Theme::Light
                    } else {
                        Theme::System
                    }
                }
                Theme::System => {
                    if self.system_prefers_dark.get() {
                        Theme::Light
                    } else {
                        Theme::Dark
                    }
                }
                Theme::Unknown => Theme::Unknown,
            }
        });
    }

    pub fn set_theme(&self, theme: Theme) {
        self.theme.set(theme);
    }

    pub fn effective_theme(&self) -> Theme {
        match self.theme.get() {
            Theme::System => {
                if self.system_prefers_dark.get() {
                    Theme::Dark
                } else {
                    Theme::Light
                }
            }
            theme => theme,
        }
    }

    pub fn get_current_theme() -> Theme {
        #[cfg(feature = "hydrate")]
        {
            use web_sys::window;
            if let Some(doc) = window().and_then(|w| w.document()) {
                if let Some(doc_element) = doc.document_element() {
                    let class_list = doc_element.class_list();
                    if class_list.contains("dark") {
                        return Theme::Dark;
                    } else {
                        return Theme::Light;
                    }
                }
            }
        }
        Theme::Unknown
    }
}

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let theme_context = ThemeContext::new();

    provide_context(theme_context);

    children()
}

#[component]
pub fn ThemeToggle(tooltip_align: Align) -> impl IntoView {
    let theme_context =
        use_context::<ThemeContext>().expect("ThemeToggle must be used within a ThemeProvider");

    let toggle_theme = move |_| {
        theme_context.toggle();
    };

    view! {
        <Tooltip label="Toggle Theme".to_string() align=tooltip_align>
            <Button
                on:click=toggle_theme
                icon=ButtonIcon::Icon(SUN_HORIZON)
                variant=BtnVariant::Square
            />
        </Tooltip>
    }
}
