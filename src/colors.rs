use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color {
    pub name: String,
    pub hex: String,
}

impl Color {
    pub fn from_tailwind(color_string: &str) -> Self {
        let col = tailwind_to_hex(color_string);

        match col {
            Some(hex) => Self {
                name: color_string.to_string(),
                hex,
            },
            None => {
                eprintln!("Invalid Tailwind color: {}", color_string);
                Self {
                    name: color_string.to_string(),
                    hex: "#000000".to_string(), // Default to black if invalid
                }
            }
        }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
        Self {
            name: "custom".to_string(),
            hex,
        }
    }
}

impl Into<String> for Color {
    fn into(self) -> String {
        self.hex
    }
}

pub fn tailwind_to_hex(color_string: &str) -> Option<String> {
    let parts: Vec<&str> = color_string.split('/').collect();
    let color_part = parts[0];
    let opacity = if parts.len() > 1 {
        parts[1].parse::<u8>().ok()?
    } else {
        100
    };

    let color_map = get_tailwind_colors();
    let hex = color_map.get(color_part)?;

    if opacity == 100 {
        Some(hex.clone())
    } else {
        let alpha = (opacity as f32 / 100.0 * 255.0) as u8;
        Some(format!("{}{:02X}", hex, alpha))
    }
}

fn get_tailwind_colors() -> HashMap<&'static str, String> {
    let mut colors = HashMap::new();

    // Red
    colors.insert("red-50", "#FEF2F2".to_string());
    colors.insert("red-100", "#FEE2E2".to_string());
    colors.insert("red-200", "#FECACA".to_string());
    colors.insert("red-300", "#FCA5A5".to_string());
    colors.insert("red-400", "#F87171".to_string());
    colors.insert("red-500", "#EF4444".to_string());
    colors.insert("red-600", "#DC2626".to_string());
    colors.insert("red-700", "#B91C1C".to_string());
    colors.insert("red-800", "#991B1B".to_string());
    colors.insert("red-900", "#7F1D1D".to_string());
    colors.insert("red-950", "#450A0A".to_string());

    // Orange
    colors.insert("orange-50", "#FFF7ED".to_string());
    colors.insert("orange-100", "#FFEDD5".to_string());
    colors.insert("orange-200", "#FED7AA".to_string());
    colors.insert("orange-300", "#FDBA74".to_string());
    colors.insert("orange-400", "#FB923C".to_string());
    colors.insert("orange-500", "#F97316".to_string());
    colors.insert("orange-600", "#EA580C".to_string());
    colors.insert("orange-700", "#C2410C".to_string());
    colors.insert("orange-800", "#9A3412".to_string());
    colors.insert("orange-900", "#7C2D12".to_string());
    colors.insert("orange-950", "#431407".to_string());

    // Amber
    colors.insert("amber-50", "#FFFBEB".to_string());
    colors.insert("amber-100", "#FEF3C7".to_string());
    colors.insert("amber-200", "#FDE68A".to_string());
    colors.insert("amber-300", "#FCD34D".to_string());
    colors.insert("amber-400", "#FBBF24".to_string());
    colors.insert("amber-500", "#F59E0B".to_string());
    colors.insert("amber-600", "#D97706".to_string());
    colors.insert("amber-700", "#B45309".to_string());
    colors.insert("amber-800", "#92400E".to_string());
    colors.insert("amber-900", "#78350F".to_string());
    colors.insert("amber-950", "#451A03".to_string());

    // Yellow
    colors.insert("yellow-50", "#FEFCE8".to_string());
    colors.insert("yellow-100", "#FEF3C7".to_string());
    colors.insert("yellow-200", "#FDE68A".to_string());
    colors.insert("yellow-300", "#FCD34D".to_string());
    colors.insert("yellow-400", "#FBBF24".to_string());
    colors.insert("yellow-500", "#F59E0B".to_string());
    colors.insert("yellow-600", "#D97706".to_string());
    colors.insert("yellow-700", "#B45309".to_string());
    colors.insert("yellow-800", "#92400E".to_string());
    colors.insert("yellow-900", "#78350F".to_string());
    colors.insert("yellow-950", "#451A03".to_string());

    // Lime
    colors.insert("lime-50", "#F7FEE7".to_string());
    colors.insert("lime-100", "#ECFCCB".to_string());
    colors.insert("lime-200", "#D9F99D".to_string());
    colors.insert("lime-300", "#BEF264".to_string());
    colors.insert("lime-400", "#A3E635".to_string());
    colors.insert("lime-500", "#84CC16".to_string());
    colors.insert("lime-600", "#65A30D".to_string());
    colors.insert("lime-700", "#4D7C0F".to_string());
    colors.insert("lime-800", "#3F6212".to_string());
    colors.insert("lime-900", "#365314".to_string());
    colors.insert("lime-950", "#1A2E05".to_string());

    // Green
    colors.insert("green-50", "#F0FDF4".to_string());
    colors.insert("green-100", "#DCFCE7".to_string());
    colors.insert("green-200", "#BBF7D0".to_string());
    colors.insert("green-300", "#86EFAC".to_string());
    colors.insert("green-400", "#4ADE80".to_string());
    colors.insert("green-500", "#22C55E".to_string());
    colors.insert("green-600", "#16A34A".to_string());
    colors.insert("green-700", "#15803D".to_string());
    colors.insert("green-800", "#166534".to_string());
    colors.insert("green-900", "#14532D".to_string());
    colors.insert("green-950", "#052E16".to_string());

    // Emerald
    colors.insert("emerald-50", "#ECFDF5".to_string());
    colors.insert("emerald-100", "#D1FAE5".to_string());
    colors.insert("emerald-200", "#A7F3D0".to_string());
    colors.insert("emerald-300", "#6EE7B7".to_string());
    colors.insert("emerald-400", "#34D399".to_string());
    colors.insert("emerald-500", "#10B981".to_string());
    colors.insert("emerald-600", "#059669".to_string());
    colors.insert("emerald-700", "#047857".to_string());
    colors.insert("emerald-800", "#065F46".to_string());
    colors.insert("emerald-900", "#064E3B".to_string());
    colors.insert("emerald-950", "#022C22".to_string());

    // Teal
    colors.insert("teal-50", "#F0FDFA".to_string());
    colors.insert("teal-100", "#CCFBF1".to_string());
    colors.insert("teal-200", "#99F6E4".to_string());
    colors.insert("teal-300", "#5EEAD4".to_string());
    colors.insert("teal-400", "#2DD4BF".to_string());
    colors.insert("teal-500", "#14B8A6".to_string());
    colors.insert("teal-600", "#0D9488".to_string());
    colors.insert("teal-700", "#0F766E".to_string());
    colors.insert("teal-800", "#115E59".to_string());
    colors.insert("teal-900", "#134E4A".to_string());
    colors.insert("teal-950", "#042F2E".to_string());

    // Cyan
    colors.insert("cyan-50", "#ECFEFF".to_string());
    colors.insert("cyan-100", "#CFFAFE".to_string());
    colors.insert("cyan-200", "#A5F3FC".to_string());
    colors.insert("cyan-300", "#67E8F9".to_string());
    colors.insert("cyan-400", "#22D3EE".to_string());
    colors.insert("cyan-500", "#06B6D4".to_string());
    colors.insert("cyan-600", "#0891B2".to_string());
    colors.insert("cyan-700", "#0E7490".to_string());
    colors.insert("cyan-800", "#155E75".to_string());
    colors.insert("cyan-900", "#164E63".to_string());
    colors.insert("cyan-950", "#083344".to_string());

    // Sky
    colors.insert("sky-50", "#F0F9FF".to_string());
    colors.insert("sky-100", "#E0F2FE".to_string());
    colors.insert("sky-200", "#BAE6FD".to_string());
    colors.insert("sky-300", "#7DD3FC".to_string());
    colors.insert("sky-400", "#38BDF8".to_string());
    colors.insert("sky-500", "#0EA5E9".to_string());
    colors.insert("sky-600", "#0284C7".to_string());
    colors.insert("sky-700", "#0369A1".to_string());
    colors.insert("sky-800", "#075985".to_string());
    colors.insert("sky-900", "#0C4A6E".to_string());
    colors.insert("sky-950", "#082F49".to_string());

    // Blue
    colors.insert("blue-50", "#EFF6FF".to_string());
    colors.insert("blue-100", "#DBEAFE".to_string());
    colors.insert("blue-200", "#BFDBFE".to_string());
    colors.insert("blue-300", "#93C5FD".to_string());
    colors.insert("blue-400", "#60A5FA".to_string());
    colors.insert("blue-500", "#3B82F6".to_string());
    colors.insert("blue-600", "#2563EB".to_string());
    colors.insert("blue-700", "#1D4ED8".to_string());
    colors.insert("blue-800", "#1E40AF".to_string());
    colors.insert("blue-900", "#1E3A8A".to_string());
    colors.insert("blue-950", "#172554".to_string());

    // Indigo
    colors.insert("indigo-50", "#EEF2FF".to_string());
    colors.insert("indigo-100", "#E0E7FF".to_string());
    colors.insert("indigo-200", "#C7D2FE".to_string());
    colors.insert("indigo-300", "#A5B4FC".to_string());
    colors.insert("indigo-400", "#818CF8".to_string());
    colors.insert("indigo-500", "#6366F1".to_string());
    colors.insert("indigo-600", "#4F46E5".to_string());
    colors.insert("indigo-700", "#4338CA".to_string());
    colors.insert("indigo-800", "#3730A3".to_string());
    colors.insert("indigo-900", "#312E81".to_string());
    colors.insert("indigo-950", "#1E1B4B".to_string());

    // Violet
    colors.insert("violet-50", "#F5F3FF".to_string());
    colors.insert("violet-100", "#EDE9FE".to_string());
    colors.insert("violet-200", "#DDD6FE".to_string());
    colors.insert("violet-300", "#C4B5FD".to_string());
    colors.insert("violet-400", "#A78BFA".to_string());
    colors.insert("violet-500", "#8B5CF6".to_string());
    colors.insert("violet-600", "#7C3AED".to_string());
    colors.insert("violet-700", "#6D28D9".to_string());
    colors.insert("violet-800", "#5B21B6".to_string());
    colors.insert("violet-900", "#4C1D95".to_string());
    colors.insert("violet-950", "#2E1065".to_string());

    // Purple
    colors.insert("purple-50", "#FAF5FF".to_string());
    colors.insert("purple-100", "#F3E8FF".to_string());
    colors.insert("purple-200", "#E9D5FF".to_string());
    colors.insert("purple-300", "#D8B4FE".to_string());
    colors.insert("purple-400", "#C084FC".to_string());
    colors.insert("purple-500", "#A855F7".to_string());
    colors.insert("purple-600", "#9333EA".to_string());
    colors.insert("purple-700", "#7E22CE".to_string());
    colors.insert("purple-800", "#6B21A8".to_string());
    colors.insert("purple-900", "#581C87".to_string());
    colors.insert("purple-950", "#3B0764".to_string());

    // Fuchsia
    colors.insert("fuchsia-50", "#FDF4FF".to_string());
    colors.insert("fuchsia-100", "#FAE8FF".to_string());
    colors.insert("fuchsia-200", "#F5D0FE".to_string());
    colors.insert("fuchsia-300", "#F0ABFC".to_string());
    colors.insert("fuchsia-400", "#E879F9".to_string());
    colors.insert("fuchsia-500", "#D946EF".to_string());
    colors.insert("fuchsia-600", "#C026D3".to_string());
    colors.insert("fuchsia-700", "#A21CAF".to_string());
    colors.insert("fuchsia-800", "#86198F".to_string());
    colors.insert("fuchsia-900", "#701A75".to_string());
    colors.insert("fuchsia-950", "#4A044E".to_string());

    // Pink
    colors.insert("pink-50", "#FDF2F8".to_string());
    colors.insert("pink-100", "#FCE7F3".to_string());
    colors.insert("pink-200", "#FBCFE8".to_string());
    colors.insert("pink-300", "#F9A8D4".to_string());
    colors.insert("pink-400", "#F472B6".to_string());
    colors.insert("pink-500", "#EC4899".to_string());
    colors.insert("pink-600", "#DB2777".to_string());
    colors.insert("pink-700", "#BE185D".to_string());
    colors.insert("pink-800", "#9D174D".to_string());
    colors.insert("pink-900", "#831843".to_string());
    colors.insert("pink-950", "#500724".to_string());

    // Rose
    colors.insert("rose-50", "#FFF1F2".to_string());
    colors.insert("rose-100", "#FFE4E6".to_string());
    colors.insert("rose-200", "#FECDD3".to_string());
    colors.insert("rose-300", "#FDA4AF".to_string());
    colors.insert("rose-400", "#FB7185".to_string());
    colors.insert("rose-500", "#F43F5E".to_string());
    colors.insert("rose-600", "#E11D48".to_string());
    colors.insert("rose-700", "#BE123C".to_string());
    colors.insert("rose-800", "#9F1239".to_string());
    colors.insert("rose-900", "#881337".to_string());
    colors.insert("rose-950", "#4C0519".to_string());

    // Gray
    colors.insert("gray-50", "#F9FAFB".to_string());
    colors.insert("gray-100", "#F3F4F6".to_string());
    colors.insert("gray-200", "#E5E7EB".to_string());
    colors.insert("gray-300", "#D1D5DB".to_string());
    colors.insert("gray-400", "#9CA3AF".to_string());
    colors.insert("gray-500", "#6B7280".to_string());
    colors.insert("gray-600", "#4B5563".to_string());
    colors.insert("gray-700", "#374151".to_string());
    colors.insert("gray-800", "#1F2937".to_string());
    colors.insert("gray-900", "#111827".to_string());
    colors.insert("gray-950", "#030712".to_string());

    // Slate
    colors.insert("slate-50", "#F8FAFC".to_string());
    colors.insert("slate-100", "#F1F5F9".to_string());
    colors.insert("slate-200", "#E2E8F0".to_string());
    colors.insert("slate-300", "#CBD5E1".to_string());
    colors.insert("slate-400", "#94A3B8".to_string());
    colors.insert("slate-500", "#64748B".to_string());
    colors.insert("slate-600", "#475569".to_string());
    colors.insert("slate-700", "#334155".to_string());
    colors.insert("slate-800", "#1E293B".to_string());
    colors.insert("slate-900", "#0F172A".to_string());
    colors.insert("slate-950", "#020617".to_string());

    // Zinc
    colors.insert("zinc-50", "#FAFAFA".to_string());
    colors.insert("zinc-100", "#F4F4F5".to_string());
    colors.insert("zinc-200", "#E4E4E7".to_string());
    colors.insert("zinc-300", "#D4D4D8".to_string());
    colors.insert("zinc-400", "#A1A1AA".to_string());
    colors.insert("zinc-500", "#71717A".to_string());
    colors.insert("zinc-600", "#52525B".to_string());
    colors.insert("zinc-700", "#3F3F46".to_string());
    colors.insert("zinc-800", "#27272A".to_string());
    colors.insert("zinc-900", "#18181B".to_string());
    colors.insert("zinc-950", "#09090B".to_string());

    // Neutral
    colors.insert("neutral-50", "#FAFAFA".to_string());
    colors.insert("neutral-100", "#F5F5F5".to_string());
    colors.insert("neutral-200", "#E5E5E5".to_string());
    colors.insert("neutral-300", "#D4D4D4".to_string());
    colors.insert("neutral-400", "#A3A3A3".to_string());
    colors.insert("neutral-500", "#737373".to_string());
    colors.insert("neutral-600", "#525252".to_string());
    colors.insert("neutral-700", "#404040".to_string());
    colors.insert("neutral-800", "#262626".to_string());
    colors.insert("neutral-900", "#171717".to_string());
    colors.insert("neutral-950", "#0A0A0A".to_string());

    // Stone
    colors.insert("stone-50", "#FAFAF9".to_string());
    colors.insert("stone-100", "#F5F5F4".to_string());
    colors.insert("stone-200", "#E7E5E4".to_string());
    colors.insert("stone-300", "#D6D3D1".to_string());
    colors.insert("stone-400", "#A8A29E".to_string());
    colors.insert("stone-500", "#78716C".to_string());
    colors.insert("stone-600", "#57534E".to_string());
    colors.insert("stone-700", "#44403C".to_string());
    colors.insert("stone-800", "#292524".to_string());
    colors.insert("stone-900", "#1C1917".to_string());
    colors.insert("stone-950", "#0C0A09".to_string());

    colors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_color_conversion() {
        assert_eq!(tailwind_to_hex("sky-500"), Some("#0EA5E9".to_string()));
        assert_eq!(tailwind_to_hex("red-600"), Some("#DC2626".to_string()));
        assert_eq!(tailwind_to_hex("emerald-500"), Some("#10B981".to_string()));
    }

    #[test]
    fn test_invalid_color() {
        assert_eq!(tailwind_to_hex("invalid-color"), None);
        assert_eq!(tailwind_to_hex("sky-999"), None);
    }

    #[test]
    fn test_invalid_opacity() {
        assert_eq!(tailwind_to_hex("sky-500/invalid"), None);
        assert_eq!(tailwind_to_hex("sky-500/"), None);
    }
}
