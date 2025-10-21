use leptos::prelude::*;
use phosphor_leptos::Icon;

#[component]
pub fn FeatureCard(
    title: &'static str,
    description: &'static str,
    icon: phosphor_leptos::IconData,
    gradient: &'static str,
) -> impl IntoView {
    let (is_hovered, set_is_hovered) = signal(false);

    let gradient_class = match gradient {
        "purple" => "from-purple-600 to-pink-600",
        "blue" => "from-blue-600 to-cyan-600",
        "green" => "from-green-600 to-teal-600",
        "orange" => "from-orange-600 to-red-600",
        _ => "from-neutral-600 to-neutral-800",
    };

    view! {
        <div
            class="relative group cursor-pointer"
            on:mouseenter=move |_| set_is_hovered.set(true)
            on:mouseleave=move |_| set_is_hovered.set(false)
        >
            <div class=move || {
                format!("absolute -inset-0.5 rounded-2xl blur opacity-30 group-hover:opacity-60 duration-1000 group-hover:duration-200",

                )
            }></div>
            <div class="relative bg-white dark:bg-neutral-900 p-8 rounded-2xl ring-1 ring-neutral-900/5 dark:ring-neutral-100/10 duration-300 group-hover:scale-[1.02]">
                <div class=move || {
                    format!(
                        "w-16 h-16 mb-6 rounded-xl bg-gradient-to-br {} flex items-center justify-center transform duration-300 {}",
                        gradient_class,
                        if is_hovered.get() { "rotate-12 scale-110" } else { "" }
                    )
                }>
                    <Icon icon={icon} size="32px" color="white" />
                </div>
                <h3 class="text-xl font-semibold text-neutral-900 dark:text-neutral-100 mb-3">
                    {title}
                </h3>
                <p class="text-neutral-600 dark:text-neutral-400 leading-relaxed">
                    {description}
                </p>
            </div>
        </div>
    }
}

#[component]
pub fn FeatureGrid(children: Children) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            {children()}
        </div>
    }
}
