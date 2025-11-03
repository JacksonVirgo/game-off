use crate::{prelude::*, systems::camera::controller::CameraController};
use bevy::{camera::ScalingMode, prelude::*};

pub enum CameraType {
    World,
    Ui,
}

#[derive(Component)]
pub struct SpawnCamera(pub CameraType);

pub fn spawn_camera(mut commands: Commands, q_spawners: Query<(Entity, &SpawnCamera)>) {
    for (entity, spawner) in q_spawners.iter() {
        match spawner.0 {
            CameraType::World => {
                commands.spawn((
                    CameraController,
                    Camera2d::default(),
                    Projection::from(OrthographicProjection {
                        scaling_mode: ScalingMode::FixedVertical {
                            viewport_height: BASE_VIEWPORT_HEIGHT,
                        },
                        ..OrthographicProjection::default_2d()
                    }),
                    Transform::from_xyz(0.0, 0.0, 999.0),
                    GlobalTransform::default(),
                ));
            }
            CameraType::Ui => {}
        }

        commands.entity(entity).despawn();
    }
}
