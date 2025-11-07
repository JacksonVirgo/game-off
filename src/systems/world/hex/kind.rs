use bevy::color::Color;

#[derive(Debug, Clone, PartialEq)]
pub enum HexKind {
    Unassigned,
    Core,
    Green,
    Blue,
    Red,
}

impl HexKind {
    pub fn to_color(&self) -> Color {
        use HexKind::*;
        match self {
            Unassigned => Color::oklch(0.3, 0.0, 0.0),
            Core => Color::oklch(0.6754, 0.1268, 0.0),
            Green => Color::oklch(0.7, 0.1376, 139.46),
            Blue => Color::oklch(0.6, 0.1376, 239.83),
            Red => Color::oklch(0.6, 0.1376, 21.93),
        }
    }
}
