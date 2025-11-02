use bevy::prelude::*;

use crate::app::input::InputPlugin;

pub fn start_game() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game Off".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(InputPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
