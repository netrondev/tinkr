//! Boring Avatars - Rust port of the TypeScript boring-avatars library
//!
//! This module provides procedurally generated avatar components for Leptos.
//! Avatars are generated deterministically from a name string and color palette.
//!
//! # Example
//!
//! ```rust,ignore
//! use app_web::components::BoringAvatarBeam;
//! use leptos::prelude::*;
//!
//! #[component]
//! fn UserProfile() -> impl IntoView {
//!     view! {
//!         <BoringAvatarBeam
//!             name="Maria Mitchell".to_string()
//!             size=80
//!         />
//!     }
//! }
//! ```
//!
//! # Custom Colors
//!
//! ```rust,ignore
//! use app_web::components::BoringAvatarBeam;
//! use leptos::prelude::*;
//!
//! #[component]
//! fn CustomAvatar() -> impl IntoView {
//!     let custom_colors = vec![
//!         "#FF6B6B".to_string(),
//!         "#4ECDC4".to_string(),
//!         "#45B7D1".to_string(),
//!         "#FFA07A".to_string(),
//!         "#98D8C8".to_string(),
//!     ];
//!
//!     view! {
//!         <BoringAvatarBeam
//!             name="Jane Doe".to_string()
//!             colors=custom_colors
//!             size=120
//!             square=false
//!             title=true
//!         />
//!     }
//! }
//! ```

use super::super::utilities::*;
use leptos::prelude::*;

const SIZE: u32 = 36;

#[derive(Clone, Debug)]
struct BeamData {
    wrapper_color: String,
    face_color: String,
    background_color: String,
    wrapper_translate_x: i32,
    wrapper_translate_y: i32,
    wrapper_rotate: i32,
    wrapper_scale: f32,
    is_mouth_open: bool,
    is_circle: bool,
    eye_spread: i32,
    mouth_spread: i32,
    face_rotate: i32,
    face_translate_x: i32,
    face_translate_y: i32,
}

/// Generate beam avatar data from name and colors
fn generate_beam_data(name: &str, colors: &[String]) -> BeamData {
    let num_from_name = hash_code(name);
    let range = colors.len();

    let wrapper_color = get_random_color(num_from_name, colors, range);
    let pre_translate_x = get_unit(num_from_name, 10, Some(1));
    let wrapper_translate_x = if pre_translate_x < 5 {
        pre_translate_x + SIZE as i32 / 9
    } else {
        pre_translate_x
    };
    let pre_translate_y = get_unit(num_from_name, 10, Some(2));
    let wrapper_translate_y = if pre_translate_y < 5 {
        pre_translate_y + SIZE as i32 / 9
    } else {
        pre_translate_y
    };

    BeamData {
        wrapper_color: wrapper_color.clone(),
        face_color: get_contrast(&wrapper_color),
        background_color: get_random_color(num_from_name + 13, colors, range),
        wrapper_translate_x,
        wrapper_translate_y,
        wrapper_rotate: get_unit(num_from_name, 360, None),
        wrapper_scale: 1.0 + (get_unit(num_from_name, SIZE as i32 / 12, None) as f32) / 10.0,
        is_mouth_open: get_boolean(num_from_name, 2),
        is_circle: get_boolean(num_from_name, 1),
        eye_spread: get_unit(num_from_name, 5, None),
        mouth_spread: get_unit(num_from_name, 3, None),
        face_rotate: get_unit(num_from_name, 10, Some(3)),
        face_translate_x: if wrapper_translate_x > SIZE as i32 / 6 {
            wrapper_translate_x / 2
        } else {
            get_unit(num_from_name, 8, Some(1))
        },
        face_translate_y: if wrapper_translate_y > SIZE as i32 / 6 {
            wrapper_translate_y / 2
        } else {
            get_unit(num_from_name, 7, Some(2))
        },
    }
}

/// Generate SVG string for beam avatar
fn generate_beam_svg(data: &BeamData, size: u32, square: bool, name: &str, title: bool) -> String {
    let mask_id = format!("mask__{}", hash_code(name));

    let mouth_path = if data.is_mouth_open {
        format!("M15,{} c2,1 4,1 6,0", 19 + data.mouth_spread)
    } else {
        format!("M13,{} a1,0.75 0 0,0 10,0", 19 + data.mouth_spread)
    };

    let wrapper_transform = format!(
        "translate({} {}) rotate({} {} {}) scale({})",
        data.wrapper_translate_x,
        data.wrapper_translate_y,
        data.wrapper_rotate,
        SIZE / 2,
        SIZE / 2,
        data.wrapper_scale
    );

    let face_transform = format!(
        "translate({} {}) rotate({} {} {})",
        data.face_translate_x,
        data.face_translate_y,
        data.face_rotate,
        SIZE / 2,
        SIZE / 2
    );

    let title_element = if title {
        format!("<title>{}</title>", name)
    } else {
        String::new()
    };

    let mask_rx = if square { 0 } else { SIZE * 2 };
    let wrapper_rx = if data.is_circle { SIZE } else { SIZE / 6 };

    format!(
        "<svg viewBox=\"0 0 {0} {0}\" fill=\"none\" role=\"img\" xmlns=\"http://www.w3.org/2000/svg\" width=\"{1}\" height=\"{1}\">\
         {2}\
         <mask id=\"{3}\" maskUnits=\"userSpaceOnUse\" x=\"0\" y=\"0\" width=\"{0}\" height=\"{0}\">\
         <rect width=\"{0}\" height=\"{0}\" rx=\"{4}\" fill=\"#FFFFFF\" />\
         </mask>\
         <g mask=\"url(#{3})\">\
         <rect width=\"{0}\" height=\"{0}\" fill=\"{5}\" />\
         <rect x=\"0\" y=\"0\" width=\"{0}\" height=\"{0}\" transform=\"{6}\" fill=\"{7}\" rx=\"{13}\" />\
         <g transform=\"{8}\">\
         <path d=\"{9}\" fill=\"none\" stroke=\"{10}\" stroke-linecap=\"round\" />\
         <rect x=\"{11}\" y=\"14\" width=\"1.5\" height=\"2\" rx=\"1\" stroke=\"none\" fill=\"{10}\" />\
         <rect x=\"{12}\" y=\"14\" width=\"1.5\" height=\"2\" rx=\"1\" stroke=\"none\" fill=\"{10}\" />\
         </g>\
         </g>\
         </svg>",
        SIZE,                          // 0
        size,                          // 1
        title_element,                 // 2
        mask_id,                       // 3
        mask_rx,                       // 4
        data.background_color,         // 5
        wrapper_transform,             // 6
        data.wrapper_color,            // 7
        face_transform,                // 8
        mouth_path,                    // 9
        data.face_color,              // 10
        14 - data.eye_spread,         // 11
        20 + data.eye_spread,         // 12
        wrapper_rx,                    // 13
    )
}

#[component]
pub fn BoringAvatarBeam(
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
    let data = generate_beam_data(&name, &colors);
    let svg_string = generate_beam_svg(&data, size, square, &name, title);

    view! { <div inner_html=svg_string /> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_code() {
        // Test that hash_code produces consistent values
        let hash1 = hash_code("test");
        let hash2 = hash_code("test");
        assert_eq!(hash1, hash2);

        // Different strings should produce different hashes
        let hash3 = hash_code("different");
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_get_digit() {
        assert_eq!(get_digit(12345, 0), 5);
        assert_eq!(get_digit(12345, 1), 4);
        assert_eq!(get_digit(12345, 2), 3);
    }

    #[test]
    fn test_get_boolean() {
        // Test with a number where we know the digit
        let num = 12345; // digit at position 0 is 5 (odd) -> false
        assert_eq!(get_boolean(num, 0), false);
        // digit at position 1 is 4 (even) -> true
        assert_eq!(get_boolean(num, 1), true);
    }

    #[test]
    fn test_get_contrast() {
        // Light color should return black
        assert_eq!(get_contrast("#FFFFFF"), "#000000");
        // Dark color should return white
        assert_eq!(get_contrast("#000000"), "#FFFFFF");
    }

    #[test]
    fn test_generate_beam_data() {
        let colors = vec![
            "#92A1C6".to_string(),
            "#146A7C".to_string(),
            "#F0AB3D".to_string(),
            "#C271B4".to_string(),
            "#C20D90".to_string(),
        ];

        let data = generate_beam_data("test", &colors);

        // Verify data structure is populated
        assert!(!data.wrapper_color.is_empty());
        assert!(!data.face_color.is_empty());
        assert!(!data.background_color.is_empty());
    }

    #[test]
    fn test_generate_beam_svg() {
        let colors = vec![
            "#92A1C6".to_string(),
            "#146A7C".to_string(),
            "#F0AB3D".to_string(),
            "#C271B4".to_string(),
            "#C20D90".to_string(),
        ];

        let data = generate_beam_data("test", &colors);
        let svg = generate_beam_svg(&data, 40, false, "test", false);

        // Verify SVG contains expected elements
        assert!(svg.contains("<svg"));
        assert!(svg.contains("viewBox"));
        assert!(svg.contains("mask"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_consistent_output() {
        let colors = vec![
            "#92A1C6".to_string(),
            "#146A7C".to_string(),
            "#F0AB3D".to_string(),
            "#C271B4".to_string(),
            "#C20D90".to_string(),
        ];

        // Test that same input produces same output
        let data1 = generate_beam_data("Maria Mitchell", &colors);
        let data2 = generate_beam_data("Maria Mitchell", &colors);

        assert_eq!(data1.wrapper_color, data2.wrapper_color);
        assert_eq!(data1.face_color, data2.face_color);
        assert_eq!(data1.background_color, data2.background_color);
        assert_eq!(data1.wrapper_translate_x, data2.wrapper_translate_x);
        assert_eq!(data1.is_mouth_open, data2.is_mouth_open);
        assert_eq!(data1.is_circle, data2.is_circle);
    }

    #[test]
    #[ignore] // Run with --ignored to see output
    fn test_visual_output() {
        let colors = vec![
            "#92A1C6".to_string(),
            "#146A7C".to_string(),
            "#F0AB3D".to_string(),
            "#C271B4".to_string(),
            "#C20D90".to_string(),
        ];

        let test_names = vec!["Maria Mitchell", "Jane Doe", "test", "John Smith"];

        for name in test_names {
            let data = generate_beam_data(name, &colors);
            let svg = generate_beam_svg(&data, 80, false, name, true);
            println!("\n=== Avatar for: {} ===", name);
            println!("{}", svg);
            println!("\nData: {:?}\n", data);
        }
    }
}
