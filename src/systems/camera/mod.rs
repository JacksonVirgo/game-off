use bevy::app::Update;

use crate::prelude::*;

pub mod controller;
pub mod spawn;

plugin!(CameraPlugin, |app| {
    app.add_systems(Update, spawn::spawn_camera);
});
