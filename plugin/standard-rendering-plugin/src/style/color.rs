use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {
    pub fn transparent() -> Self {
        Self(0, 0, 0, 0)
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.0, self.1, self.2, self.3)
    }
}

pub struct ColorParseError;

impl TryFrom<&str> for Color {
    type Error = ColorParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!("Color parsing is not implemented")
    }
}
