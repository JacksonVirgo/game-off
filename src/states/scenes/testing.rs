use bevy::prelude::*;

use crate::{
    systems::{
        camera::spawn::SpawnCamera,
        world::tiles::{data::TileKind, spawn::SpawnTile},
    },
    utils::grid::generate_hex_grid,
};

pub fn load_testing(mut commands: Commands) {
    commands.spawn(SpawnCamera);

    for hex_pos in generate_hex_grid(2, 577.0) {
        commands.spawn(SpawnTile {
            kind: TileKind::Empty,
            translation: hex_pos,
        });
    }
}
