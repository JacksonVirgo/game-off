use crate::{
    prelude::*,
    systems::camera::data::{CameraController, CameraType, SpawnCamera, UICameraController},
};
use bevy::{
    camera::{ScalingMode, visibility::RenderLayers},
    prelude::*,
};

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
                    Transform::from_xyz(0.0, 0.0, 10.0),
                    Msaa::Sample4,
                    GlobalTransform::default(),
                ));
            }
            CameraType::Ui => {
                commands.spawn((
                    UICameraController,
                    Camera2d::default(),
                    Camera {
                        order: CAM_LAYER_UI as isize,
                        ..default()
                    },
                    RenderLayers::layer(CAM_LAYER_UI),
                    Transform::default(),
                ));
            }
        }

        commands.entity(entity).despawn();
    }
}
