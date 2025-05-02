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
