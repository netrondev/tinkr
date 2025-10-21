use leptos::prelude::*;

#[component]
pub fn Section(
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let base_class = "py-24 px-4 sm:px-6 lg:px-8";
    let combined_class = match class {
        Some(c) => format!("{} {}", base_class, c),
        None => base_class.to_string(),
    };

    view! {
        <section id={id} class={combined_class}>
            <div class="">
                {children()}
            </div>
        </section>
    }
}

#[component]
pub fn SectionStyled(children: Children) -> impl IntoView {
    view! {
        <section class="bg-white dark:bg-neutral-800 rounded-lg shadow-sm border border-neutral-200 dark:border-neutral-700 p-6">
            {children()}
        </section>
    }
}

#[component]
pub fn SectionHeader(
    title: &'static str,
    #[prop(optional)] subtitle: Option<&'static str>,
    #[prop(optional)] centered: bool,
) -> impl IntoView {
    let alignment_class = if centered { "text-center" } else { "" };

    view! {
        <div class={format!("mb-16 {}", alignment_class)}>
            <h2 class="text-4xl md:text-5xl font-bold text-neutral-900 dark:text-neutral-100 mb-4">
                {title}
            </h2>
            {subtitle.map(|s| view! {
                <p class="text-xl text-neutral-600 dark:text-neutral-400">
                    {s}
                </p>
            })}
        </div>
    }
}
