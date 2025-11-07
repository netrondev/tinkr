use super::super::utilities::*;
use leptos::prelude::*;

const SIZE: u32 = 80;
const ELEMENTS: usize = 64;

/// Generate pixel colors from name and color palette
fn generate_pixel_colors(name: &str, colors: &[String]) -> Vec<String> {
    let num_from_name = hash_code(name);
    let range = colors.len();

    (0..ELEMENTS)
        .map(|i| get_random_color(num_from_name % ((i as u32) + 1), colors, range))
        .collect()
}

/// Generate SVG string for pixel avatar
fn generate_pixel_svg(
    pixel_colors: &[String],
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

    // Generate 64 pixel rectangles in an 8x8 grid
    // Pattern: rows of 8 pixels, alternating x positions
    let x_positions = [0, 20, 40, 60, 10, 30, 50, 70];
    let mut pixels = String::new();

    for row in 0..8 {
        for col in 0..8 {
            let index = row * 8 + col;
            let x = x_positions[col];
            let y = row * 10;
            pixels.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"10\" height=\"10\" fill=\"{}\" />",
                x, y, pixel_colors[index]
            ));
        }
    }

    format!(
        "<svg viewBox=\"0 0 {0} {0}\" fill=\"none\" role=\"img\" xmlns=\"http://www.w3.org/2000/svg\" width=\"{1}\" height=\"{1}\">\
         {2}\
         <mask id=\"{3}\" maskUnits=\"userSpaceOnUse\" x=\"0\" y=\"0\" width=\"{0}\" height=\"{0}\" mask-type=\"alpha\">\
         <rect width=\"{0}\" height=\"{0}\" rx=\"{4}\" fill=\"#FFFFFF\" />\
         </mask>\
         <g mask=\"url(#{3})\">\
         {5}\
         </g>\
         </svg>",
        SIZE,          // 0
        size,          // 1
        title_element, // 2
        mask_id,       // 3
        mask_rx,       // 4
        pixels,        // 5
    )
}

#[component]
pub fn BoringAvatarPixel(
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
    let pixel_colors = generate_pixel_colors(&name, &colors);
    let svg_string = generate_pixel_svg(&pixel_colors, size, square, &name, title);

    view! {
        <div inner_html=svg_string />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pixel_colors() {
        let colors = vec![
            "#92A1C6".to_string(),
            "#146A7C".to_string(),
            "#F0AB3D".to_string(),
            "#C271B4".to_string(),
            "#C20D90".to_string(),
        ];

        let pixel_colors = generate_pixel_colors("test", &colors);

        assert_eq!(pixel_colors.len(), ELEMENTS);
        assert!(!pixel_colors[0].is_empty());
    }

    #[test]
    fn test_generate_pixel_svg() {
        let colors = vec![
            "#92A1C6".to_string(),
            "#146A7C".to_string(),
            "#F0AB3D".to_string(),
            "#C271B4".to_string(),
            "#C20D90".to_string(),
        ];

        let pixel_colors = generate_pixel_colors("test", &colors);
        let svg = generate_pixel_svg(&pixel_colors, 80, false, "test", false);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("mask-type=\"alpha\""));
        assert!(svg.contains("width=\"10\" height=\"10\""));
        assert!(svg.contains("</svg>"));
    }
}
