use crate::{
    prelude::*,
    systems::{
        turns::turn::{Turn, TurnPhase},
        ui::data::TurnText,
    },
};
use bevy::{camera::visibility::RenderLayers, color::palettes::css::GOLD, prelude::*};

pub mod data;

plugin!(UiPlugin, |app| {
    app.add_systems(Startup, setup_ui);
    app.add_systems(Update, turn_ui_update);
});

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Text::new("Turn: "),
            TextFont {
                font: asset_server.load("fonts/Xolonium/Xolonium-Regular.ttf"),
                font_size: 32.0,
                ..default()
            },
            RenderLayers::layer(CAM_LAYER_UI),
        ))
        .with_child((
            TurnText,
            TextSpan::default(),
            TextFont {
                font: asset_server.load("fonts/Xolonium/Xolonium-Regular.ttf"),
                font_size: 24.0,
                ..default()
            },
            TextColor(GOLD.into()),
            RenderLayers::layer(CAM_LAYER_UI),
        ));
}

pub fn turn_ui_update(mut q_turn_text: Query<&mut TextSpan, With<TurnText>>, turn: Res<Turn>) {
    for mut span in &mut q_turn_text {
        let turn_text = match turn.phase {
            TurnPhase::Player => "Player",
            TurnPhase::Environment => "Environment",
        };

        **span = format!("{} - {}", turn.count, turn_text);
    }
}
