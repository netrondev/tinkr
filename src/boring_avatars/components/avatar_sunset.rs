use super::super::utilities::*;
use leptos::prelude::*;

const SIZE: u32 = 80;
const ELEMENTS: usize = 4;

/// Generate sunset colors from name and color palette
fn generate_sunset_colors(name: &str, colors: &[String]) -> Vec<String> {
    let num_from_name = hash_code(name);
    let range = colors.len();

    (0..ELEMENTS)
        .map(|i| get_random_color(num_from_name + (i as u32), colors, range))
        .collect()
}

/// Generate SVG string for sunset avatar
fn generate_sunset_svg(
    sunset_colors: &[String],
    size: u32,
    square: bool,
    name: &str,
    title: bool,
) -> String {
    let mask_id = format!("mask__{}", hash_code(name));
    let mask_rx = if square { 0 } else { SIZE * 2 };

    let title_element = if title {
        format!("<title>{}</title>", name)
    } else {
        String::new()
    };

    // Create unique gradient IDs based on name (remove spaces)
    let name_no_space = name.replace(" ", "");
    let gradient_id_0 = format!("gradient_paint0_linear_{}", name_no_space);
    let gradient_id_1 = format!("gradient_paint1_linear_{}", name_no_space);

    format!(
        "<svg viewBox=\"0 0 {0} {0}\" fill=\"none\" role=\"img\" xmlns=\"http://www.w3.org/2000/svg\" width=\"{1}\" height=\"{1}\">\
         {2}\
         <mask id=\"{3}\" maskUnits=\"userSpaceOnUse\" x=\"0\" y=\"0\" width=\"{0}\" height=\"{0}\">\
         <rect width=\"{0}\" height=\"{0}\" rx=\"{4}\" fill=\"#FFFFFF\" />\
         </mask>\
         <g mask=\"url(#{3})\">\
         <path fill=\"url(#{5})\" d=\"M0 0h80v40H0z\" />\
         <path fill=\"url(#{6})\" d=\"M0 40h80v40H0z\" />\
         </g>\
         <defs>\
         <linearGradient id=\"{5}\" x1=\"{7}\" y1=\"0\" x2=\"{7}\" y2=\"{8}\">\
         <stop offset=\"0\" stop-color=\"{9}\" />\
         <stop offset=\"1\" stop-color=\"{10}\" />\
         </linearGradient>\
         <linearGradient id=\"{6}\" x1=\"{7}\" y1=\"{8}\" x2=\"{7}\" y2=\"{0}\">\
         <stop offset=\"0\" stop-color=\"{11}\" />\
         <stop offset=\"1\" stop-color=\"{12}\" />\
         </linearGradient>\
         </defs>\
         </svg>",
        SIZE,             // 0
        size,             // 1
        title_element,    // 2
        mask_id,          // 3
        mask_rx,          // 4
        gradient_id_0,    // 5
        gradient_id_1,    // 6
        SIZE / 2,         // 7 - x1, x2 (center)
        SIZE / 2,         // 8 - y2 for first gradient, y1 for second
        sunset_colors[0], // 9
        sunset_colors[1], // 10
        sunset_colors[2], // 11
        sunset_colors[3], // 12
    )
}

#[component]
pub fn BoringAvatarSunset(
    /// Name to generate avatar from
    name: String,
    /// Color palette
    #[prop(default = vec![
        "#92A1C6".to_string(),
        "#146A7C".to_string(),
        "#F0AB3D".to_string(),
        "#C271B4".to_string(),
        "#C20D90".to_string(),
    ])]
    colors: Vec<String>,
    /// Size of the avatar (width and height)
    #[prop(default = 40)]
    size: u32,
    /// Whether to use square corners instead of rounded
    #[prop(default = false)]
    square: bool,
    /// Whether to include a title element
    #[prop(default = false)]
    title: bool,
) -> impl IntoView {
    let sunset_colors = generate_sunset_colors(&name, &colors);
    let svg_string = generate_sunset_svg(&sunset_colors, size, square, &name, title);

    view! {
        <div inner_html=svg_string />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_sunset_colors() {
        let colors = vec![
            "#92A1C6".to_string(),
            "#146A7C".to_string(),
            "#F0AB3D".to_string(),
            "#C271B4".to_string(),
            "#C20D90".to_string(),
        ];

        let sunset_colors = generate_sunset_colors("test", &colors);

        assert_eq!(sunset_colors.len(), ELEMENTS);
        assert!(!sunset_colors[0].is_empty());
    }

    #[test]
    fn test_generate_sunset_svg() {
        let colors = vec![
            "#92A1C6".to_string(),
            "#146A7C".to_string(),
            "#F0AB3D".to_string(),
            "#C271B4".to_string(),
            "#C20D90".to_string(),
        ];

        let sunset_colors = generate_sunset_colors("test", &colors);
        let svg = generate_sunset_svg(&sunset_colors, 80, false, "test", false);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("linearGradient"));
        assert!(svg.contains("stop-color"));
        assert!(svg.contains("</svg>"));
    }
}
