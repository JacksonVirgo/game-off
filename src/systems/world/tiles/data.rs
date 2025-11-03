use bevy::prelude::*;

#[derive(Clone)]
pub enum TileKind {
    Empty,
    Green,
    Blue,
    Red,
}

impl TileKind {
    pub fn to_color(&self) -> Color {
        match self {
            TileKind::Empty => Color::oklch(0.6, 0.1143, 287.91),
            TileKind::Green => Color::oklch(0.7, 0.1376, 139.46),
            TileKind::Blue => Color::oklch(0.6, 0.1376, 239.83),
            TileKind::Red => Color::oklch(0.6, 0.1376, 21.93),
        }
    }
}

#[derive(Component)]
pub struct Tile {
    pub kind: TileKind,
}

#[derive(Component)]
pub struct TileInner;
