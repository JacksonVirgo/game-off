use crate::{prelude::*, systems::camera::controller::CameraController};
use bevy::{camera::ScalingMode, prelude::*};

#[derive(Component)]
pub struct SpawnCamera;

pub fn spawn_camera(mut commands: Commands, q_spawners: Query<(Entity, &SpawnCamera)>) {
    for (entity, _) in q_spawners.iter() {
        commands.spawn((
            CameraController,
            Camera2d::default(),
            Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical {
                    viewport_height: BASE_VIEWPORT_HEIGHT,
                },
                ..OrthographicProjection::default_2d()
            }),
            Msaa::Sample4,
            Transform::from_xyz(0.0, 0.0, 999.0),
            GlobalTransform::default(),
        ));
        commands.entity(entity).despawn();
    }
}
