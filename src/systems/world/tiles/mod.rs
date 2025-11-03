use crate::{
    prelude::*,
    systems::world::tiles::data::{Tile, TileInner},
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
) {
    for (child_of, mut sprite) in &mut q_tile {
        if let Ok(tile) = tiles.get(child_of.parent()) {
            sprite.color = tile.kind.to_color();
        }
    }
}
