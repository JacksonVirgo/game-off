use crate::{
    prelude::*,
    systems::world::tiles::data::{Tile, TileInner, TileKind},
};
use bevy::prelude::*;

pub mod data;
pub mod spawn;

plugin!(TilePlugin, |app| {
    app.add_systems(Update, (spawn::spawn_tile, update_tile_inner_colors));
});

fn update_tile_inner_colors(
    mut q_tile: Query<(&ChildOf, &mut Sprite), With<TileInner>>,
    tiles: Query<&Tile>,
    time: Res<Time>,
) {
    let hue_speed = 60.0;

    for (child_of, mut sprite) in &mut q_tile {
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
