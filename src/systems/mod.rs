use crate::prelude::*;

pub mod camera;
pub mod materials;
pub mod turns;
pub mod world;

plugin!(SystemPlugin, |app| {
    app.add_plugins((camera::CameraPlugin, world::WorldPlugin, turns::TurnPlugin));
});
