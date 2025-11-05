use bevy::prelude::*;

use crate::{
    systems::{
        camera::data::{CameraType, SpawnCamera},
        world::tiles::spawn::SpawnTile,
    },
    utils::grid::generate_hex_grid,
};

pub fn load_testing(mut commands: Commands) {
    commands.spawn(SpawnCamera(CameraType::World));
    commands.spawn(SpawnCamera(CameraType::Ui));
    for (tile_kind, hex_pos) in generate_hex_grid(3, 577.0) {
        commands.spawn(SpawnTile {
            kind: tile_kind,
            translation: hex_pos,
        });
    }
}
