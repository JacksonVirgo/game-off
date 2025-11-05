use bevy::prelude::*;

#[derive(Component)]
pub struct CameraController;

#[derive(Component)]
pub struct UICameraController;

#[derive(Component, Default)]
#[require(Transform)]
pub struct CameraFocus(pub u32);

pub enum CameraType {
    World,
    Ui,
}

#[derive(Component)]
pub struct SpawnCamera(pub CameraType);
