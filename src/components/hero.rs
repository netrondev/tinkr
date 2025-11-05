use crate::components::ParticleAnimation;
use leptos::prelude::*;

#[component]
pub fn Hero(title: &'static str, subtitle: &'static str, children: Children) -> impl IntoView {
    view! {
        <section
            class="relative flex-0 flex items-center justify-center overflow-hidden h-full"
            style="height: 80vh; min-height: 600px;"
        >
            // Particle animation background
            <div class="absolute inset-0 overflow-hidden">
                <ParticleAnimation />
            </div>

            // Content
            <div class="relative z-10 max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
                <h1 class="text-5xl md:text-7xl font-bold text-neutral-900 dark:text-neutral-100 mb-6 tracking-tight">
                    {title}
                </h1>
                <p class="text-xl md:text-2xl text-neutral-600 dark:text-neutral-400 mb-12 max-w-3xl mx-auto">
                    {subtitle}
                </p>
                <div class="flex flex-col sm:flex-row gap-4 justify-center mb-12">{children()}</div>
            </div>
        </section>
    }
}

#[component]
pub fn HeroButton(href: &'static str, variant: &'static str, children: Children) -> impl IntoView {
    let class = match variant {
        "primary" => {
            "bg-neutral-900 dark:bg-neutral-100 text-neutral-100 dark:text-neutral-900 hover:bg-neutral-800 dark:hover:bg-neutral-200 px-8 py-4 rounded-lg font-semibold text-lg transform hover:scale-105 shadow-lg"
        }
        "secondary" => {
            "bg-transparent border-2 border-neutral-900 dark:border-neutral-100 text-neutral-900 dark:text-neutral-100 hover:bg-neutral-900 hover:text-neutral-100 dark:hover:bg-neutral-100 dark:hover:text-neutral-900 px-8 py-4 rounded-lg font-semibold text-lg transform hover:scale-105"
        }
        _ => "",
    };

    view! {
        <a href=href class=class>
            {children()}
        </a>
    }
}
