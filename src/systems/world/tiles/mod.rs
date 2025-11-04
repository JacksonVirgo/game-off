use crate::prelude::*;
use bevy::prelude::*;

pub mod data;
pub mod spawn;
pub mod tile_state;

plugin!(TilePlugin, |app| {
    app.add_systems(Update, spawn::spawn_tile).add_systems(
        Update,
        (tile_state::update_tile_visuals).in_set(OrderSet::Visual),
    );
});
