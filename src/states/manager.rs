use bevy::{ecs::system::ScheduleSystem, prelude::*};

use crate::systems::turns::turn::Turn;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash, Copy)]
pub enum SceneState {
    #[default]
    Testing,
}

#[derive(Component)]
pub struct StatePersist;

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, Without<StatePersist>>,
    mut turn: ResMut<Turn>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }

    turn.reset();
}

pub fn add_scene<M>(
    app: &mut App,
    scene: SceneState,
    systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
) {
    app.add_systems(OnEnter(scene), systems);
    app.add_systems(OnExit(scene), cleanup);
}
