use super::{Avatar, AvatarVariants};
use leptos::prelude::*;

// Example names from https://notablewomen.withgoogle.com/all
const EXAMPLE_NAMES: &[&str] = &[
    "Mary Baker",
    "Amelia Earhart",
    "Mary Roebling",
    "Sarah Winnemucca",
    "Margaret Brent",
    "Lucy Stone",
    "Mary Edwards",
    "Margaret Chase",
    "Mahalia Jackson",
    "Maya Angelou",
    "Margaret Bourke",
    "Eunice Kennedy",
    "Carrie Chapman",
    "Elizabeth Peratrovich",
    "Alicia Dickerson",
    "Daisy Gatson",
    "Emma Willard",
    "Amelia Boynton",
    "Maria Mitchell",
    "Sojourner Truth",
    "Willa Cather",
    "Coretta Scott",
    "Harriet Tubman",
    "Fabiola Cabeza",
    "Sacagawea",
    "Esther Martinez",
    "Elizabeth Cady",
    "Bessie Coleman",
    "Ma Rainey",
    "Julia Ward",
    "Irene Morgan",
    "Babe Didrikson",
    "Lyda Conley",
    "Annie Dodge",
    "Maud Nathan",
    "Betty Ford",
    "Rosa Parks",
    "Susan La",
    "Gertrude Stein",
    "Wilma Mankiller",
    "Grace Hopper",
    "Jane Addams",
    "Katharine Graham",
    "Florence Chadwick",
    "Zora Neale",
    "Wilma Rudolph",
    "Annie Jump",
    "Mother Frances",
    "Jovita Id�r",
    "Maggie L",
];

// Load color palettes from JSON
const COLOR_PALETTES_JSON: &str = include_str!("color_palettes.json");

fn get_color_palettes() -> &'static Vec<Vec<String>> {
    use std::sync::OnceLock;
    static COLOR_PALETTES: OnceLock<Vec<Vec<String>>> = OnceLock::new();
    COLOR_PALETTES.get_or_init(|| {
        serde_json::from_str::<Vec<Vec<String>>>(COLOR_PALETTES_JSON).unwrap_or_else(|_| {
            vec![vec![
                "#92A1C6".to_string(),
                "#146A7C".to_string(),
                "#F0AB3D".to_string(),
                "#C271B4".to_string(),
                "#C20D90".to_string(),
            ]]
        })
    })
}

fn get_random_palette_index() -> usize {
    #[cfg(not(feature = "ssr"))]
    return (js_sys::Math::random() * (get_color_palettes().len() as f64)).round() as usize;

    // Use timestamp-based randomness for cross-platform compatibility
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    (now as usize) % get_color_palettes().len()
}

#[component]
fn AvatarItem(
    name: RwSignal<String>,
    #[prop(into)] colors: Signal<Vec<String>>,
    #[prop(into)] size: Signal<u32>,
    #[prop(into)] square: Signal<bool>,
    #[prop(into)] variant: Signal<AvatarVariants>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-2 p-4">
            <div class="flex justify-center items-center">
                {move || {
                    view! {
                        <Avatar
                            name=name.get()
                            variant=variant.get()
                            colors=colors.get()
                            size=size.get()
                            square=square.get()
                        />
                    }
                }}
            </div>
            <input
                class="w-full text-center p-2 border border-transparent rounded-full bg-transparent hover:border-gray-300 focus:border-blue-500 focus:outline-none transition-colors"
                type="text"
                prop:value=name
                on:input=move |ev| {
                    name.set(event_target_value(&ev));
                }
                on:focus=move |ev| {
                    let input = event_target::<web_sys::HtmlInputElement>(&ev);
                    input.select();
                }
            />
        </div>
    }
}

#[component]
pub fn AvatarPlayground() -> impl IntoView {
    // Default palette (index 493 from the TS version)
    let default_palette = get_color_palettes().get(493).cloned().unwrap_or_else(|| {
        vec![
            "#92A1C6".to_string(),
            "#146A7C".to_string(),
            "#F0AB3D".to_string(),
            "#C271B4".to_string(),
            "#C20D90".to_string(),
        ]
    });

    let palette = RwSignal::new(default_palette.clone());

    // let (color0, set_color0) = signal(default_palette[0].clone());
    // let (color1, set_color1) = signal(default_palette[1].clone());
    // let (color2, set_color2) = signal(default_palette[2].clone());
    // let (color3, set_color3) = signal(default_palette[3].clone());
    // let (color4, set_color4) = signal(default_palette[4].clone());

    // let colors = Signal::derive(move || {
    //     vec![
    //         color0.get(),
    //         color1.get(),
    //         color2.get(),
    //         color3.get(),
    //         color4.get(),
    //     ]
    // });

    let (variant, set_variant) = signal(AvatarVariants::Beam);
    let (size, set_size) = signal(80_u32);
    let (square, set_square) = signal(false);

    let random_palette = move |_| {
        let palette_random = &get_color_palettes()[get_random_palette_index()];
        leptos::logging::log!("Selected random palette: {:?}", palette_random);

        palette.set(palette_random.clone());

        // set_color0.set(palette[0].clone());
        // set_color1.set(palette[1].clone());
        // set_color2.set(palette[2].clone());
        // set_color3.set(palette[3].clone());
        // set_color4.set(palette[4].clone());
    };

    // Create signals for each name
    let names: Vec<_> = EXAMPLE_NAMES
        .iter()
        .map(|&name| RwSignal::new(name.to_string()))
        .collect();

    view! {
        <div class="min-h-screen bg-gray-50 text-black">
            // Banner
            <div class="bg-gray-900 text-white p-8">
                <p>
                    "This is a playground to test the Rust/Leptos implementation of "
                    <a href="https://boringavatars.com" class="text-white underline">
                        "boringavatars.com"
                    </a>
                </p>
            </div>

            <pre>
                {move || {
                    format!(
                        "palette: {:?}\nVariant: {:?}\nSize: {}\nSquare: {}",
                        palette.get(),
                        variant.get(),
                        size.get(),
                        square.get(),
                    )
                }}
            </pre>

            // Header Controls
            <header class="grid grid-cols-[auto_1fr_auto_auto_auto] gap-4 p-6 items-center bg-white border-b">
                // Variant Selector
                <div class="flex gap-1 border rounded-lg overflow-hidden">
                    {["Beam", "Bauhaus", "Ring", "Sunset", "Pixel", "Marble"]
                        .iter()
                        .map(|&v| {
                            let variant_type = match v {
                                "Beam" => AvatarVariants::Beam,
                                "Bauhaus" => AvatarVariants::Bauhaus,
                                "Ring" => AvatarVariants::Ring,
                                "Sunset" => AvatarVariants::Sunset,
                                "Pixel" => AvatarVariants::Pixel,
                                "Marble" => AvatarVariants::Marble,
                                _ => AvatarVariants::Beam,
                            };
                            view! {
                                <button
                                    class=move || {
                                        format!(
                                            "px-4 py-2 text-sm font-medium transition-colors {}",
                                            if matches!(
                                                (variant.get(), variant_type),
                                                (AvatarVariants::Beam, AvatarVariants::Beam)
                                                | (AvatarVariants::Bauhaus, AvatarVariants::Bauhaus)
                                                | (AvatarVariants::Ring, AvatarVariants::Ring)
                                                | (AvatarVariants::Sunset, AvatarVariants::Sunset)
                                                | (AvatarVariants::Pixel, AvatarVariants::Pixel)
                                                | (AvatarVariants::Marble, AvatarVariants::Marble)
                                            ) {
                                                "bg-blue-500 text-white"
                                            } else {
                                                "bg-white hover:bg-gray-100"
                                            },
                                        )
                                    }

                                    on:click=move |_| set_variant.set(variant_type)
                                >
                                    {v}
                                </button>
                            }
                        })
                        .collect_view()}
                </div>

                // Color Pickers
                <div class="flex gap-2">
                    <input
                        type="color"
                        class="w-10 h-10 rounded cursor-pointer"
                        prop:value=move || palette.get()[0].clone()
                        on:input=move |ev| {
                            let mut pal = palette.get();
                            pal[0] = event_target_value(&ev);
                            palette.set(pal);
                        }
                    />
                    <input
                        type="color"
                        class="w-10 h-10 rounded cursor-pointer"
                        prop:value=move || palette.get()[1].clone()
                        on:input=move |ev| {
                            let mut pal = palette.get();
                            pal[1] = event_target_value(&ev);
                            palette.set(pal);
                        }
                    />
                    <input
                        type="color"
                        class="w-10 h-10 rounded cursor-pointer"
                        prop:value=move || palette.get()[2].clone()
                        on:input=move |ev| {
                            let mut pal = palette.get();
                            pal[2] = event_target_value(&ev);
                            palette.set(pal);
                        }
                    />
                    <input
                        type="color"
                        class="w-10 h-10 rounded cursor-pointer"
                        prop:value=move || palette.get()[3].clone()
                        on:input=move |ev| {
                            let mut pal = palette.get();
                            pal[3] = event_target_value(&ev);
                            palette.set(pal);
                        }
                    />
                    <input
                        type="color"
                        class="w-10 h-10 rounded cursor-pointer"
                        prop:value=move || palette.get()[4].clone()
                        on:input=move |ev| {
                            let mut pal = palette.get();
                            pal[4] = event_target_value(&ev);
                            palette.set(pal);
                        }
                    />
                </div>

                // Random Palette Button
                <button
                    class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
                    on:click=random_palette
                >
                    "Random palette"
                </button>

                // Square/Round Toggle
                <button
                    class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
                    on:click=move |_| set_square.update(|s| *s = !*s)
                >
                    {move || if square.get() { "Round" } else { "Square" }}
                </button>

                // Size Selector
                <div class="flex gap-1 items-center border rounded-lg p-1">
                    <button
                        class=move || {
                            format!(
                                "w-8 h-8 rounded flex items-center justify-center {}",
                                if size.get() == 40 { "bg-gray-200" } else { "hover:bg-gray-100" },
                            )
                        }

                        on:click=move |_| set_size.set(40)
                    >
                        <div class="w-2 h-2 bg-current rounded-full"></div>
                    </button>
                    <button
                        class=move || {
                            format!(
                                "w-8 h-8 rounded flex items-center justify-center {}",
                                if size.get() == 80 { "bg-gray-200" } else { "hover:bg-gray-100" },
                            )
                        }

                        on:click=move |_| set_size.set(80)
                    >
                        <div class="w-3.5 h-3.5 bg-current rounded-full"></div>
                    </button>
                    <button
                        class=move || {
                            format!(
                                "w-8 h-8 rounded flex items-center justify-center {}",
                                if size.get() == 128 { "bg-gray-200" } else { "hover:bg-gray-100" },
                            )
                        }

                        on:click=move |_| set_size.set(128)
                    >
                        <div class="w-5 h-5 bg-current rounded-full"></div>
                    </button>
                </div>
            </header>

            // Avatars Grid
            <div class="grid grid-cols-[repeat(auto-fill,minmax(8rem,1fr))] gap-8 p-6">
                {names
                    .into_iter()
                    .map(|name| {
                        view! {
                            <AvatarItem
                                name=name
                                colors=palette
                                size=size
                                square=square
                                variant=variant
                            />
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }.into_any()
}
