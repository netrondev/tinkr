use super::super::utilities::*;
use leptos::prelude::*;

const SIZE: u32 = 80;
const ELEMENTS: usize = 3;

#[derive(Clone, Debug)]
struct MarbleElement {
    color: String,
    translate_x: i32,
    translate_y: i32,
    scale: f32,
    rotate: i32,
}

/// Generate marble element data from name and colors
fn generate_marble_data(name: &str, colors: &[String]) -> Vec<MarbleElement> {
    let num_from_name = hash_code(name);
    let range = colors.len();

    (0..ELEMENTS)
        .map(|i| {
            let num = num_from_name.wrapping_mul((i as u32) + 1);
            MarbleElement {
                color: get_random_color(num_from_name + (i as u32), colors, range),
                translate_x: get_unit(num, (SIZE / 10) as i32, Some(1)),
                translate_y: get_unit(num, (SIZE / 10) as i32, Some(2)),
                scale: 1.2 + (get_unit(num, (SIZE / 20) as i32, None) as f32) / 10.0,
                rotate: get_unit(num, 360, Some(1)),
            }
        })
        .collect()
}

/// Generate SVG string for marble avatar
fn generate_marble_svg(
    elements: &[MarbleElement],
    size: u32,
    square: bool,
    name: &str,
    title: bool,
) -> String {
    let mask_id = format!("mask__{}", hash_code(name));
    let filter_id = format!("filter__{}", hash_code(name));
    let mask_rx = if square { 0 } else { SIZE * 2 };

    let title_element = if title {
        format!("<title>{}</title>", name)
    } else {
        String::new()
    };

    // Path 1 transform
    let path1_transform = format!(
        "translate({} {}) rotate({} {} {}) scale({})",
        elements[1].translate_x,
        elements[1].translate_y,
        elements[1].rotate,
        SIZE / 2,
        SIZE / 2,
        elements[1].scale
    );

    // Path 2 transform
    let path2_transform = format!(
        "translate({} {}) rotate({} {} {}) scale({})",
        elements[2].translate_x,
        elements[2].translate_y,
        elements[2].rotate,
        SIZE / 2,
        SIZE / 2,
        elements[2].scale
    );

    format!(
        "<svg viewBox=\"0 0 {0} {0}\" fill=\"none\" role=\"img\" xmlns=\"http://www.w3.org/2000/svg\" width=\"{1}\" height=\"{1}\">\
         {2}\
         <mask id=\"{3}\" maskUnits=\"userSpaceOnUse\" x=\"0\" y=\"0\" width=\"{0}\" height=\"{0}\">\
         <rect width=\"{0}\" height=\"{0}\" rx=\"{4}\" fill=\"#FFFFFF\" />\
         </mask>\
         <g mask=\"url(#{3})\">\
         <rect width=\"{0}\" height=\"{0}\" fill=\"{5}\" />\
         <path filter=\"url(#{6})\" d=\"M32.414 59.35L50.376 70.5H72.5v-71H33.728L26.5 13.381l19.057 27.08L32.414 59.35z\" fill=\"{7}\" transform=\"{8}\" />\
         <path filter=\"url(#{6})\" d=\"M22.216 24L0 46.75l14.108 38.129L78 86l-3.081-59.276-22.378 4.005 12.972 20.186-23.35 27.395L22.215 24z\" fill=\"{9}\" transform=\"{10}\" style=\"mix-blend-mode: overlay\" />\
         </g>\
         <defs>\
         <filter id=\"{6}\" filterUnits=\"userSpaceOnUse\" color-interpolation-filters=\"sRGB\">\
         <feFlood flood-opacity=\"0\" result=\"BackgroundImageFix\" />\
         <feBlend in=\"SourceGraphic\" in2=\"BackgroundImageFix\" result=\"shape\" />\
         <feGaussianBlur stdDeviation=\"7\" result=\"effect1_foregroundBlur\" />\
         </filter>\
         </defs>\
         </svg>",
        SIZE,              // 0
        size,              // 1
        title_element,     // 2
        mask_id,           // 3
        mask_rx,           // 4
        elements[0].color, // 5 - background
        filter_id,         // 6
        elements[1].color, // 7
        path1_transform,   // 8
        elements[2].color, // 9
        path2_transform,   // 10
    )
}

#[component]
pub fn BoringAvatarMarble(
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
    let elements = generate_marble_data(&name, &colors);
    let svg_string = generate_marble_svg(&elements, size, square, &name, title);

    view! { <div inner_html=svg_string /> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_marble_data() {
        let colors = vec![
            "#92A1C6".to_string(),
            "#146A7C".to_string(),
            "#F0AB3D".to_string(),
            "#C271B4".to_string(),
            "#C20D90".to_string(),
        ];

        let elements = generate_marble_data("test", &colors);

        assert_eq!(elements.len(), ELEMENTS);
        assert!(!elements[0].color.is_empty());
        assert!(elements[1].scale > 1.0);
    }

    #[test]
    fn test_generate_marble_svg() {
        let colors = vec![
            "#92A1C6".to_string(),
            "#146A7C".to_string(),
            "#F0AB3D".to_string(),
            "#C271B4".to_string(),
            "#C20D90".to_string(),
        ];

        let elements = generate_marble_data("test", &colors);
        let svg = generate_marble_svg(&elements, 80, false, "test", false);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("filter"));
        assert!(svg.contains("feGaussianBlur"));
        assert!(svg.contains("mix-blend-mode"));
        assert!(svg.contains("</svg>"));
    }
}
