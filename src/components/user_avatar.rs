use leptos::prelude::*;

#[component]
pub fn UserAvatar(
    name: Option<String>,
    image: Option<String>,
    #[prop(default = "md")] size: &'static str,
    #[prop(default = "")] class: &'static str,
) -> impl IntoView {
    let size_class = match size {
        "sm" => "w-6 h-6",
        "md" => "w-8 h-8",
        "lg" => "w-10 h-10",
        "xl" => "w-12 h-12",
        _ => "w-8 h-8", // default to md
    };

    let avatar_url = if let Some(img) = image {
        img
    } else if let Some(n) = &name {
        format!(
            "https://ui-avatars.com/api/?name={}&background=3B82F6&color=fff&size=128",
            n
        )
    } else {
        format!("https://ui-avatars.com/api/?name=U&background=6B7280&color=fff&size=128")
    };

    let default_class = "rounded-full object-cover";
    let combined_class = if !class.is_empty() {
        format!("{} {} {}", size_class, default_class, class)
    } else {
        format!("{} {}", size_class, default_class)
    };

    view! {
        <img
            src=avatar_url
            alt=name.unwrap_or_else(|| "User avatar".to_string())
            class=combined_class
        />
    }
}
