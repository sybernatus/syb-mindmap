use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Default, Serialize)]
pub struct Color {
    pub hex: String,
}

impl Color {

    pub fn from_hex(hex: String) -> Self {
        Self { hex }
    }

    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { hex: format!("#{:02X}{:02X}{:02X}", red, green, blue) }
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
