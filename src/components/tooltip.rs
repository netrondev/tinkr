use leptos::html::Div;
use leptos::prelude::*;
use tw_merge::tw_merge;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Left,
    Top,
    Right,
    Bottom,
}

#[derive(Debug, Clone)]
pub struct TooltipData {
    pub label: String,
    pub x: f64,
    pub y: f64,
    pub align: Align,
    pub visible: bool,
}

#[component]
pub fn Tooltip<T>(
    label: T,
    #[prop(optional)] align: Option<Align>,
    children: Children,
) -> impl IntoView
where
    T: ToString + Clone + 'static,
{
    let alignment = align.unwrap_or(Align::Top);
    let label_string = label.to_string();
    let node_ref = NodeRef::<Div>::new();

    let tooltips =
        use_context::<RwSignal<Vec<TooltipData>>>().expect("LabelProvider context not found");

    let tooltip_id = RwSignal::new(None::<usize>);
    let auto_hide_handle = StoredValue::new(None::<TimeoutHandle>);

    let update_position = move || {
        if let Some(element) = node_ref.get() {
            let rect = element.get_bounding_client_rect();

            let (x, y) = match alignment {
                Align::Top => (rect.left() + rect.width() / 2.0, rect.top() - 8.0),
                Align::Bottom => (rect.left() + rect.width() / 2.0, rect.bottom() + 8.0),
                Align::Left => (rect.left() - 8.0, rect.top() + rect.height() / 2.0),
                Align::Right => (rect.right() + 8.0, rect.top() + rect.height() / 2.0),
            };

            if let Some(id) = tooltip_id.get() {
                tooltips.update(|tips| {
                    if let Some(tip) = tips.get_mut(id) {
                        tip.x = x;
                        tip.y = y;
                        tip.visible = true;
                    }
                });
            }
        }
    };

    let hide_tooltip = move || {
        // Cancel any pending auto-hide timer
        if let Some(handle) = auto_hide_handle.get_value() {
            handle.clear();
        }
        auto_hide_handle.set_value(None);

        tooltips.update(|tips| {
            tips.clear();
        });
        tooltip_id.set(None);
    };

    let show_tooltip = move || {
        // Cancel any existing auto-hide timer
        if let Some(handle) = auto_hide_handle.get_value() {
            handle.clear();
        }

        let new_tooltip = TooltipData {
            label: label_string.clone(),
            x: 0.0,
            y: 0.0,
            align: alignment,
            visible: true,
        };

        let id = tooltips.with_untracked(|tips| tips.len());
        tooltips.update(|tips| {
            tips.push(new_tooltip);
        });

        tooltip_id.set(Some(id));
        update_position();

        // Set up auto-hide timer for 3 seconds
        if let Ok(handle) = set_timeout_with_handle(
            move || {
                hide_tooltip();
            },
            std::time::Duration::from_secs(3),
        ) {
            auto_hide_handle.set_value(Some(handle));
        }
    };

    view! {
        <div
            node_ref=node_ref
            class="inline-block"
            on:mouseenter=move |_| show_tooltip()
            on:mouseleave=move |_| hide_tooltip()
        >
            {children()}
        </div>
    }
}

#[component]
pub fn LabelProvider(children: Children) -> impl IntoView {
    let tooltips = RwSignal::new(vec![] as Vec<TooltipData>);
    provide_context(tooltips);

    let tooltip_classes = "fixed z-[9999] px-2 py-1 text-sm text-white bg-neutral-900 dark:bg-neutral-700 rounded-lg shadow-lg pointer-events-none whitespace-nowrap duration-200";
    let arrow_classes = "absolute w-0 h-0 border-solid";

    view! {
        {children()}
        <For
            each=move || tooltips.get().into_iter().enumerate()
            key=|(idx, _)| *idx
            children=move |(_idx, tooltip)| {
                let transform_class = match tooltip.align {
                    Align::Top => "-translate-x-1/2 -translate-y-full",
                    Align::Bottom => "-translate-x-1/2",
                    Align::Left => "-translate-x-full -translate-y-1/2",
                    Align::Right => "-translate-y-1/2",
                };
                let arrow_position = match tooltip.align {
                    Align::Top => {
                        "bottom-[-5px] left-1/2 transform -translate-x-1/2 border-l-[5px] border-l-transparent border-r-[5px] border-r-transparent border-t-[5px] border-t-neutral-900 dark:border-t-neutral-700"
                    }
                    Align::Bottom => {
                        "top-[-5px] left-1/2 transform -translate-x-1/2 border-l-[5px] border-l-transparent border-r-[5px] border-r-transparent border-b-[5px] border-b-neutral-900 dark:border-b-neutral-700"
                    }
                    Align::Left => {
                        "right-[-5px] top-1/2 transform -translate-y-1/2 border-t-[5px] border-t-transparent border-b-[5px] border-b-transparent border-l-[5px] border-l-neutral-900 dark:border-l-neutral-700"
                    }
                    Align::Right => {
                        "left-[-5px] top-1/2 transform -translate-y-1/2 border-t-[5px] border-t-transparent border-b-[5px] border-b-transparent border-r-[5px] border-r-neutral-900 dark:border-r-neutral-700"
                    }
                };

                view! {
                    <div
                        class=tw_merge!(
                            tooltip_classes,
                            transform_class,
                            if tooltip.visible { "opacity-100" } else { "opacity-0 invisible" }
                        )
                        style:left=format!("{}px", tooltip.x)
                        style:top=format!("{}px", tooltip.y)
                    >
                        {tooltip.label.clone()}
                        <div class=tw_merge!(arrow_classes, arrow_position)></div>
                    </div>
                }
            }
        />
    }
}
