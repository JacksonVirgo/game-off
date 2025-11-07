use crate::prelude::*;
use bevy::prelude::*;

pub mod hex;
pub mod kind;
pub mod relation;
pub mod spawn;
pub mod state;

pub const DEFAULT_HUE_SPEED: f32 = 60.0;
pub const HEX_GAP: f32 = 1.2;

plugin!(HexPlugin, |app| {
    app.add_systems(
        Update,
        (state::update_tile_visuals).in_set(OrderSet::Visual),
    );

    app.add_systems(Update, spawn::spawn_hex);
});
