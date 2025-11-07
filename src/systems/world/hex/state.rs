use bevy::prelude::*;

use crate::systems::world::hex::{
    DEFAULT_HUE_SPEED,
    hex::{Hex, HexFlipAlert, HexInner},
    kind::HexKind,
};

pub fn update_tile_state() {}

pub fn update_tile_visuals(
    mut q_tiles: Query<(&ChildOf, &mut Sprite), With<HexInner>>,
    mut tiles: Query<(&Hex, &mut Sprite, Option<&HexFlipAlert>), Without<HexInner>>,
    time: Res<Time>,
) {
    let hue_speed = DEFAULT_HUE_SPEED;

    for (child_of, mut sprite) in &mut q_tiles {
        if let Ok((tile, mut frame_sprite, flip_alert)) = tiles.get_mut(child_of.parent()) {
            match tile.kind {
                HexKind::Core => {
                    let hue = sprite.color.hue();
                    let new_hue = (hue + hue_speed * time.delta_secs()) % 360.0;
                    sprite.color = Color::oklch(0.6, 0.1376, new_hue);
                }
                _ => {
                    sprite.color = tile.kind.to_color();
                }
            }

            if flip_alert.is_some() {
                let hue = frame_sprite.color.hue();
                let new_hue = (hue + hue_speed * time.delta_secs()) % 360.0;
                frame_sprite.color = Color::oklch(0.6, 0.1376, new_hue);
            }
        }
    }
}

pub fn flip_tile() {}
