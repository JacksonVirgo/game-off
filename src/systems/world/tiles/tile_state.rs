use bevy::prelude::*;

use crate::systems::world::tiles::data::{DEFAULT_HUE_SPEED, Tile, TileInner, TileKind};

pub fn update_tile_state() {}

pub fn update_tile_visuals(
    mut q_tiles: Query<(&ChildOf, &mut Sprite), With<TileInner>>,
    tiles: Query<&Tile>,
    time: Res<Time>,
) {
    let hue_speed = DEFAULT_HUE_SPEED;

    for (child_of, mut sprite) in &mut q_tiles {
        if let Ok(tile) = tiles.get(child_of.parent()) {
            match tile.kind {
                TileKind::Core => {
                    let hue = sprite.color.hue();
                    let new_hue = (hue + hue_speed * time.delta_secs()) % 360.0;
                    sprite.color = Color::oklch(0.6, 0.1376, new_hue);
                }
                _ => {
                    sprite.color = tile.kind.to_color();
                }
            }
        }
    }
}

pub fn flip_tile() {}
