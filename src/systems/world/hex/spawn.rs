use bevy::prelude::*;

use crate::{
    config::UNIT_SIZE,
    systems::world::{
        grid::{coord::Coord, grid::HexGrid},
        hex::{
            HEX_GAP,
            hex::{Hex, HexInner},
            kind::HexKind,
        },
    },
};

#[derive(Component, Debug)]
pub struct SpawnHex {
    pub kind: HexKind,
    pub coord: Coord,
}

impl SpawnHex {
    pub fn new(kind: HexKind, coord: Coord) -> Self {
        Self { kind, coord }
    }
}

pub fn spawn_hex(
    mut commands: Commands,
    mut hex_grid: ResMut<HexGrid>,
    q_spawners: Query<(Entity, &SpawnHex)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, spawner) in q_spawners.iter() {
        let frame = commands
            .spawn((
                Hex {
                    kind: spawner.kind.clone(),
                },
                spawner.coord.clone(),
                Sprite {
                    image: asset_server.load("sprites/hex_frame.png"),
                    custom_size: Some(Vec2::splat(UNIT_SIZE)),
                    image_mode: SpriteImageMode::Scale(ScalingMode::FitCenter),
                    ..default()
                },
                Transform::from_translation(spawner.coord.axial_to_world(UNIT_SIZE, HEX_GAP)),
                GlobalTransform::default(),
            ))
            .with_children(|c| {
                c.spawn((
                    HexInner,
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
            })
            .id();

        commands.entity(entity).despawn();

        hex_grid.map.insert(spawner.coord.clone(), frame);
    }
}
