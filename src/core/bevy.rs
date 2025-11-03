use crate::GlobalPlugin;
use bevy::prelude::*;

pub fn start_game() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Game Off".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_linear()),
        )
        .add_plugins(GlobalPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
