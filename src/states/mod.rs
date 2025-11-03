use bevy::state::app::AppExtStates;

use crate::{
    prelude::*,
    states::manager::{SceneState, add_scene},
};

pub mod manager;
pub mod scenes;

plugin!(StateManagerPlugin, |app| {
    app.init_state::<SceneState>();
    add_scene(
        app,
        manager::SceneState::Testing,
        scenes::testing::load_testing,
    );
});
