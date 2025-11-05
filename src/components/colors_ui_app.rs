use crate::colors::Color;
use leptos::{IntoView, component, prelude::*, view};

#[component]
pub fn ColorsApp() -> impl IntoView {
    let color_families = vec![
        "red", "orange", "amber", "yellow", "lime", "green", "emerald", "teal", "cyan", "sky",
        "blue", "indigo", "violet", "purple", "fuchsia", "pink", "rose", "slate", "gray", "zinc",
        "neutral", "stone",
    ];

    let shades = vec![
        "50", "100", "200", "300", "400", "500", "600", "700", "800", "900", "950",
    ];

    let copy_message = RwSignal::new(None::<String>);

    let copy_to_clipboard = move |_value: String| {
        // #[cfg(not(feature = "ssr"))]
        // {
        //     let _ = window().navigator().clipboard().write_text(&value);
        //     copy_message.set(Some(format!("Copied: {}", value)));
        //     set_timeout(
        //         move || copy_message.set(None),
        //         std::time::Duration::from_millis(2000),
        //     );
        // }
    };

    view! {
        <div class="bg-gray-900 min-h-screen p-8">
            <div class="max-w-7xl mx-auto">
                <h1 class="text-3xl font-bold text-white mb-8">"Tailwind Color Palette"</h1>

                <div class="space-y-4">
                    {color_families
                        .into_iter()
                        .map(|family| {
                            view! {
                                <div class="flex items-center gap-4">
                                    <div class="w-20 text-gray-300 text-sm font-medium capitalize">
                                        {family}
                                    </div>
                                    <div class="flex gap-2">
                                        {shades
                                            .clone()
                                            .into_iter()
                                            .map(|shade| {
                                                let color_name = format!("{}-{}", family, shade);
                                                let color = Color::from_tailwind(&color_name);
                                                let hex_value = color.hex.clone();
                                                let hex_for_click = hex_value.clone();

                                                view! {
                                                    <button
                                                        class="w-12 h-12 rounded-lg hover:scale-110 cursor-pointer shadow-lg"
                                                        style=format!("background-color: {}", hex_value)
                                                        title=format!("{} - {}", color_name, hex_value)
                                                        on:click=move |_| copy_to_clipboard(hex_for_click.clone())
                                                    />
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>

                <div class="mt-12 text-center text-gray-400 text-sm">
                    "Click to copy the OKLCH value or shift+click to copy the nearest hex value."
                </div>

                {move || {
                    copy_message
                        .get()
                        .map(|msg| {
                            view! {
                                <div class="fixed bottom-4 right-4 bg-gray-800 text-white px-4 py-2 rounded-lg shadow-lg">
                                    {msg}
                                </div>
                            }
                        })
                }}
            </div>
        </div>
    }
}
