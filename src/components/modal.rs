use leptos::prelude::*;
use tw_merge::tw_merge;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModalSize {
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
    FullWidth,
}

#[derive(Clone, Copy)]
pub struct ModalState {
    pub visible: RwSignal<bool>,
}

#[component]
pub fn Modal(
    show: Signal<bool>,
    #[prop(optional)] on_close: Option<Callback<()>>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] size: ModalSize,
    children: ChildrenFn,
) -> impl IntoView {
    let modals = use_context::<RwSignal<Vec<(usize, ModalState)>>>()
        .expect("ModalProvider context not found");

    let modal_id = StoredValue::new(None::<usize>);
    let visible = RwSignal::new(false);
    let next_id = StoredValue::new(0_usize);

    Effect::new(move || {
        let is_visible = show.get();

        if is_visible {
            if modal_id.get_value().is_none() {
                let id = next_id.get_value();
                next_id.set_value(id + 1);

                let state = ModalState { visible };
                modals.update(|modals| {
                    modals.push((id, state));
                });
                modal_id.set_value(Some(id));
            }
            visible.set(true);
        } else {
            visible.set(false);
        }
    });

    let size_class = match size {
        ModalSize::Small => "max-w-md",
        ModalSize::Medium => "max-w-lg",
        ModalSize::Large => "max-w-2xl",
        ModalSize::ExtraLarge => "max-w-4xl",
        ModalSize::FullWidth => "max-w-7xl",
    };

    let close_modal = move |_| {
        if let Some(callback) = on_close {
            callback.run(());
        }
        visible.set(false);
    };

    view! {
        <Show when=move || visible.get() fallback=|| ()>
            <div
                class="fixed inset-0 z-50 overflow-y-auto"
                aria-labelledby="modal-title"
                role="dialog"
                aria-modal="true"
            >
                <div class="flex items-center justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
                    <div
                        class="fixed inset-0 bg-neutral-500/30 backdrop-blur-lg transition-opacity"
                        aria-hidden="true"
                        on:click=close_modal
                    ></div>

                    <div class=tw_merge!(
                        "inline-block align-top bg-white dark:bg-neutral-800 rounded-lg container text-left overflow-hidden shadow-xl transform sm:my-8 sm:align-middle {} w-full shadow-xl", size_class
                    )>
                        <div class="px-4 pt-5 pb-4 sm:p-6 sm:pb-4 relative dark:bg-black bg-white rounded-md">
                            {title
                                .as_ref()
                                .map(|t| {
                                    view! {
                                        <h3
                                            class="text-lg leading-6 font-medium text-neutral-900 dark:text-neutral-100 mb-4"
                                            id="modal-title"
                                        >
                                            {t.clone()}
                                        </h3>
                                    }
                                })} {children()}
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}

#[component]
pub fn ModalProvider(children: Children) -> impl IntoView {
    let modals = RwSignal::new(vec![] as Vec<(usize, ModalState)>);
    provide_context(modals);

    view! { {children()} }
}
