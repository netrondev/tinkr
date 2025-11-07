use super::super::utilities::*;
use leptos::prelude::*;

const SIZE: u32 = 90;
const COLORS: usize = 5;

/// Generate ring colors from name and color palette
fn generate_ring_colors(name: &str, colors: &[String]) -> Vec<String> {
    let num_from_name = hash_code(name);
    let range = colors.len();

    // Generate 5 unique colors
    let colors_shuffle: Vec<String> = (0..COLORS)
        .map(|i| get_random_color(num_from_name + (i as u32), colors, range))
        .collect();

    // Map to 9-element array with specific pattern
    vec![
        colors_shuffle[0].clone(),
        colors_shuffle[1].clone(),
        colors_shuffle[1].clone(),
        colors_shuffle[2].clone(),
        colors_shuffle[2].clone(),
        colors_shuffle[3].clone(),
        colors_shuffle[3].clone(),
        colors_shuffle[0].clone(),
        colors_shuffle[4].clone(),
    ]
}

/// Generate SVG string for ring avatar
fn generate_ring_svg(
    ring_colors: &[String],
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

    format!(
        "<svg viewBox=\"0 0 {0} {0}\" fill=\"none\" role=\"img\" xmlns=\"http://www.w3.org/2000/svg\" width=\"{1}\" height=\"{1}\">\
         {2}\
         <mask id=\"{3}\" maskUnits=\"userSpaceOnUse\" x=\"0\" y=\"0\" width=\"{0}\" height=\"{0}\">\
         <rect width=\"{0}\" height=\"{0}\" rx=\"{4}\" fill=\"#FFFFFF\" />\
         </mask>\
         <g mask=\"url(#{3})\">\
         <path d=\"M0 0h90v45H0z\" fill=\"{5}\" />\
         <path d=\"M0 45h90v45H0z\" fill=\"{6}\" />\
         <path d=\"M83 45a38 38 0 00-76 0h76z\" fill=\"{7}\" />\
         <path d=\"M83 45a38 38 0 01-76 0h76z\" fill=\"{8}\" />\
         <path d=\"M77 45a32 32 0 10-64 0h64z\" fill=\"{9}\" />\
         <path d=\"M77 45a32 32 0 11-64 0h64z\" fill=\"{10}\" />\
         <path d=\"M71 45a26 26 0 00-52 0h52z\" fill=\"{11}\" />\
         <path d=\"M71 45a26 26 0 01-52 0h52z\" fill=\"{12}\" />\
         <circle cx=\"45\" cy=\"45\" r=\"23\" fill=\"{13}\" />\
         </g>\
         </svg>",
        SIZE,           // 0
        size,           // 1
        title_element,  // 2
        mask_id,        // 3
        mask_rx,        // 4
        ring_colors[0], // 5
        ring_colors[1], // 6
        ring_colors[2], // 7
        ring_colors[3], // 8
        ring_colors[4], // 9
        ring_colors[5], // 10
        ring_colors[6], // 11
        ring_colors[7], // 12
        ring_colors[8], // 13
    )
}

#[component]
pub fn BoringAvatarRing(
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
    let ring_colors = generate_ring_colors(&name, &colors);
    let svg_string = generate_ring_svg(&ring_colors, size, square, &name, title);

    view! {
        <div inner_html=svg_string />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_ring_colors() {
        let colors = vec![
            "#92A1C6".to_string(),
            "#146A7C".to_string(),
            "#F0AB3D".to_string(),
            "#C271B4".to_string(),
            "#C20D90".to_string(),
        ];

        let ring_colors = generate_ring_colors("test", &colors);

        assert_eq!(ring_colors.len(), 9);
        assert!(!ring_colors[0].is_empty());
    }

    #[test]
    fn test_generate_ring_svg() {
        let colors = vec![
            "#92A1C6".to_string(),
            "#146A7C".to_string(),
            "#F0AB3D".to_string(),
            "#C271B4".to_string(),
            "#C20D90".to_string(),
        ];

        let ring_colors = generate_ring_colors("test", &colors);
        let svg = generate_ring_svg(&ring_colors, 90, false, "test", false);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("<circle"));
        assert!(svg.contains("<path"));
        assert!(svg.contains("</svg>"));
    }
}
