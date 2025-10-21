use leptos::prelude::*;

#[component]
pub fn ProgressBar(progress: RwSignal<i64>) -> impl IntoView {
    // let (progress, set_progress) = signal(0);
    // let (is_uploading, set_is_uploading) = signal(false);

    // let start_upload = move |_| {
    //     if !is_uploading.get() {
    //         set_is_uploading.set(true);
    //         set_progress.set(0);
    //         // Simulate upload progress
    //         set_progress.set(100);
    //     }
    // };

    view! {
        <div class="space-y-4">


            <div class="relative h-2 bg-neutral-700 rounded-full overflow-hidden">
                <div
                    class=move || format!(
                        "absolute inset-y-0 left-0 bg-gradient-to-r from-blue-500 to-cyan-500 duration-200",

                    )
                    style:width=move || format!("{}%", progress.get())
                ></div>
            </div>

            <div class="text-neutral-400 text-sm">
                Progress: {move || progress.get()}%
            </div>
        </div>
    }
}
