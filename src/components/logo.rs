use leptos::prelude::*;
use phosphor_leptos::{Icon, CODEPEN_LOGO};

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <div class="group relative size-10 antialiased">
            <div class="absolute -top-[0px] left-0 size-10 text-blue-500/50 antialiased duration-1000 group-hover:-top-[6px]  dark:text-blue-600">
                <Icon size="32px" icon=CODEPEN_LOGO />
            </div>
            <div class="absolute left-0 top-[0px] size-10 text-orange-500/50 duration-1000 group-hover:top-[6px] dark:text-orange-600">
                <Icon size="32px" icon=CODEPEN_LOGO />
            </div>
            <div class="absolute left-0 top-0 size-10 text-neutral-200 dark:text-white">
                <Icon size="32px" color="#777777" icon=CODEPEN_LOGO />
            </div>
        </div>
    }
}
