mod overlays;

pub use overlays::spawn_debug_ui;

/// Format accessory name for display (e.g. "drow_armor" -> "Armor")
#[allow(dead_code)]
pub fn format_accessory_name(name: &str) -> String {
    let without_prefix = name.strip_prefix("drow_").unwrap_or(name);
    let mut result = String::new();
    for (i, c) in without_prefix.chars().enumerate() {
        if i == 0 {
            result.push(c.to_ascii_uppercase());
        } else if c == '_' {
            result.push(' ');
        } else {
            result.push(c);
        }
    }
    result
}
