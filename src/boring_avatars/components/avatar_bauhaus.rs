use super::super::utilities::*;
use leptos::prelude::*;

const SIZE: u32 = 80;
const ELEMENTS: usize = 4;

#[derive(Clone, Debug)]
struct BauhausElement {
    color: String,
    translate_x: i32,
    translate_y: i32,
    rotate: i32,
    is_square: bool,
}

/// Generate bauhaus element data from name and colors
fn generate_bauhaus_data(name: &str, colors: &[String]) -> Vec<BauhausElement> {
    let num_from_name = hash_code(name);
    let range = colors.len();

    (0..ELEMENTS)
        .map(|i| {
            let num = num_from_name.wrapping_mul((i as u32) + 1);
            BauhausElement {
                color: get_random_color(num_from_name + (i as u32), colors, range),
                translate_x: get_unit(num, (SIZE / 2 - (i as u32 + 17)) as i32, Some(1)),
                translate_y: get_unit(num, (SIZE / 2 - (i as u32 + 17)) as i32, Some(2)),
                rotate: get_unit(num, 360, None),
                is_square: get_boolean(num_from_name, 2),
            }
        })
        .collect()
}

/// Generate SVG string for bauhaus avatar
fn generate_bauhaus_svg(
    elements: &[BauhausElement],
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

    // Element 1: Rectangle
    let rect_width = SIZE;
    let rect_height = if elements[0].is_square {
        SIZE
    } else {
        SIZE / 8
    };
    let rect_x = (SIZE - 60) / 2;
    let rect_y = (SIZE - 20) / 2;
    let rect_transform = format!(
        "translate({} {}) rotate({} {} {})",
        elements[1].translate_x,
        elements[1].translate_y,
        elements[1].rotate,
        SIZE / 2,
        SIZE / 2
    );

    // Element 2: Circle
    let circle_cx = SIZE / 2;
    let circle_cy = SIZE / 2;
    let circle_r = SIZE / 5;
    let circle_transform = format!(
        "translate({} {})",
        elements[2].translate_x, elements[2].translate_y
    );

    // Element 3: Line
    let line_transform = format!(
        "translate({} {}) rotate({} {} {})",
        elements[3].translate_x,
        elements[3].translate_y,
        elements[3].rotate,
        SIZE / 2,
        SIZE / 2
    );

    format!(
        "<svg viewBox=\"0 0 {0} {0}\" fill=\"none\" role=\"img\" xmlns=\"http://www.w3.org/2000/svg\" width=\"{1}\" height=\"{1}\">\
         {2}\
         <mask id=\"{3}\" maskUnits=\"userSpaceOnUse\" x=\"0\" y=\"0\" width=\"{0}\" height=\"{0}\">\
         <rect width=\"{0}\" height=\"{0}\" rx=\"{4}\" fill=\"#FFFFFF\" />\
         </mask>\
         <g mask=\"url(#{3})\">\
         <rect width=\"{0}\" height=\"{0}\" fill=\"{5}\" />\
         <rect x=\"{6}\" y=\"{7}\" width=\"{8}\" height=\"{9}\" fill=\"{10}\" transform=\"{11}\" />\
         <circle cx=\"{12}\" cy=\"{13}\" r=\"{14}\" fill=\"{15}\" transform=\"{16}\" />\
         <line x1=\"0\" y1=\"{17}\" x2=\"{0}\" y2=\"{17}\" stroke-width=\"2\" stroke=\"{18}\" transform=\"{19}\" />\
         </g>\
         </svg>",
        SIZE,              // 0
        size,              // 1
        title_element,     // 2
        mask_id,           // 3
        mask_rx,           // 4
        elements[0].color, // 5 - background
        rect_x,            // 6
        rect_y,            // 7
        rect_width,        // 8
        rect_height,       // 9
        elements[1].color, // 10
        rect_transform,    // 11
        circle_cx,         // 12
        circle_cy,         // 13
        circle_r,          // 14
        elements[2].color, // 15
        circle_transform,  // 16
        SIZE / 2,          // 17 - line y position
        elements[3].color, // 18
        line_transform,    // 19
    )
}

#[component]
pub fn BoringAvatarBauhaus(
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
    let elements = generate_bauhaus_data(&name, &colors);
    let svg_string = generate_bauhaus_svg(&elements, size, square, &name, title);

    view! {
        <div inner_html=svg_string />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_bauhaus_data() {
        let colors = vec![
            "#92A1C6".to_string(),
            "#146A7C".to_string(),
            "#F0AB3D".to_string(),
            "#C271B4".to_string(),
            "#C20D90".to_string(),
        ];

        let elements = generate_bauhaus_data("test", &colors);

        assert_eq!(elements.len(), ELEMENTS);
        assert!(!elements[0].color.is_empty());
    }

    #[test]
    fn test_generate_bauhaus_svg() {
        let colors = vec![
            "#92A1C6".to_string(),
            "#146A7C".to_string(),
            "#F0AB3D".to_string(),
            "#C271B4".to_string(),
            "#C20D90".to_string(),
        ];

        let elements = generate_bauhaus_data("test", &colors);
        let svg = generate_bauhaus_svg(&elements, 80, false, "test", false);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("<circle"));
        assert!(svg.contains("<line"));
        assert!(svg.contains("</svg>"));
    }
}
