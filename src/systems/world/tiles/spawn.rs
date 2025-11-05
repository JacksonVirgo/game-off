use bevy::prelude::*;

use crate::{
    config::UNIT_SIZE,
    systems::world::tiles::data::{Tile, TileInner, TileKind},
};

#[derive(Component)]
pub struct SpawnTile {
    pub kind: TileKind,
    pub translation: Vec3,
}

impl Default for SpawnTile {
    fn default() -> Self {
        Self {
            kind: TileKind::Empty,
            translation: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

pub fn spawn_tile(
    mut commands: Commands,
    q_spawners: Query<(Entity, &SpawnTile)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, spawner) in q_spawners.iter() {
        let mut frame = commands.spawn((
            Tile {
                kind: spawner.kind.clone(),
            },
            Sprite {
                image: asset_server.load("sprites/hex_frame.png"),
                custom_size: Some(Vec2::splat(UNIT_SIZE)),
                image_mode: SpriteImageMode::Scale(ScalingMode::FitCenter),
                ..default()
            },
            Transform::from_translation(spawner.translation),
            GlobalTransform::default(),
        ));

        frame.with_children(|c| {
            c.spawn((
                TileInner,
                Sprite {
                    image: asset_server.load("sprites/hex_blank_inner.png"),
                    custom_size: Some(Vec2::splat(UNIT_SIZE)),
                    image_mode: SpriteImageMode::Scale(ScalingMode::FitCenter),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, -1.0),
                GlobalTransform::default(),
                InheritedVisibility::default(),
            ));
        });

        commands.entity(entity).despawn();
    }
}
