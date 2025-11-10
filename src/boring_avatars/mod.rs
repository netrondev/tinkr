pub mod components;
pub mod playground;
pub mod utilities;

pub use playground::AvatarPlayground;

use components::{
    BoringAvatarBauhaus, BoringAvatarBeam, BoringAvatarMarble, BoringAvatarPixel,
    BoringAvatarRing, BoringAvatarSunset,
};
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarVariants {
    Pixel,
    Bauhaus,
    Ring,
    Beam,
    Sunset,
    Marble,
    Geometric, // Deprecated, use 'beam'
    Abstract,  // Deprecated, use 'bauhaus'
}

#[component]
pub fn Avatar(
    name: String,
    variant: AvatarVariants,
    #[prop(default = vec![
        "#92A1C6".to_string(),
        "#146A7C".to_string(),
        "#F0AB3D".to_string(),
        "#C271B4".to_string(),
        "#C20D90".to_string(),
    ])]
    colors: Vec<String>,
    #[prop(default = 40)]
    size: u32,
    #[prop(default = false)]
    square: bool,
    #[prop(default = false)]
    title: bool,
) -> AnyView {
    match variant {
        AvatarVariants::Pixel => view! { <BoringAvatarPixel name=name colors=colors size=size square=square title=title /> }
        .into_any(),
        AvatarVariants::Bauhaus => view! { <BoringAvatarBauhaus name=name colors=colors size=size square=square title=title /> }
        .into_any(),
        AvatarVariants::Ring => view! { <BoringAvatarRing name=name colors=colors size=size square=square title=title /> }
        .into_any(),
        AvatarVariants::Beam | AvatarVariants::Geometric => view! { <BoringAvatarBeam name=name colors=colors size=size square=square title=title /> }
        .into_any(),
        AvatarVariants::Sunset => view! { <BoringAvatarSunset name=name colors=colors size=size square=square title=title /> }
        .into_any(),
        AvatarVariants::Marble => view! { <BoringAvatarMarble name=name colors=colors size=size square=square title=title /> }
        .into_any(),
        AvatarVariants::Abstract => view! { <BoringAvatarBauhaus name=name colors=colors size=size square=square title=title /> }
        .into_any(),
    }
}
