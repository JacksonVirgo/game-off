use bevy::prelude::*;

#[derive(Component)]
pub struct CameraController;

#[derive(Component, Default)]
#[require(Transform)]
pub struct CameraFocus(pub u32);
