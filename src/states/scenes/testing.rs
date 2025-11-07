use bevy::prelude::*;

use crate::systems::{
    camera::data::{CameraType, SpawnCamera},
    world::grid::grid::HexGrid,
};

pub fn load_testing(mut commands: Commands) {
    commands.spawn(SpawnCamera(CameraType::World));
    commands.spawn(SpawnCamera(CameraType::Ui));

    for spawn_hex in HexGrid::generate(3) {
        commands.spawn(spawn_hex);
    }
}
