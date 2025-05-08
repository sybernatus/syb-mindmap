use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Color {
    pub hex: String,
}

impl Color {

    pub fn from_hex(hex: String) -> Self {
        if !Self::is_valid_hex_color(&hex) {
            warn!("Invalid hex color: {}", hex);
        }
        Self { hex }
    }

    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { hex: format!("#{:02X}{:02X}{:02X}", red, green, blue) }
    }

    /// check if the hex string color is valid
    fn is_valid_hex_color(string: &str) -> bool {
        // 1. Remove the # prefix if it exists
        let hex = string.strip_prefix('#').unwrap_or(string);
        // 2. Check if the length is 6 and all characters are valid hex digits
        hex.len() == 6 && hex.chars().all(|char| char.is_digit(16))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let color = Color::from_hex("#FF5733".to_string());
        assert_eq!(color.hex, "#FF5733");
    }

    #[test]
    fn test_color_rgb() {
        let color = Color::from_rgb(255, 87, 51);
        assert_eq!(color.hex, "#FF5733");
    }
}
