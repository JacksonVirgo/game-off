use bevy::prelude::*;

use crate::core::{input::InputPlugin, sets::configure_order_set};

pub fn start_game() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game Off".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GlobalPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

struct GlobalPlugin;
impl Plugin for GlobalPlugin {
    fn build(&self, app: &mut App) {
        configure_order_set(app);
        app.add_plugins(InputPlugin);
    }
}
