use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum InputActions {
    QuickExit,
}

impl InputActions {
    pub fn player() -> InputMap<InputActions> {
        InputMap::new([(InputActions::QuickExit, KeyCode::Escape)])
    }
}

pub fn quick_exit(q_input: Query<&ActionState<InputActions>>, mut exit: MessageWriter<AppExit>) {
    let Ok(input) = q_input.single() else {
        return;
    };

    if input.pressed(&InputActions::QuickExit) {
        exit.write(AppExit::Success);
    }
}

pub fn setup_input(mut commands: Commands) {
    commands.spawn(InputActions::player());
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<InputActions>::default())
            .add_systems(Startup, setup_input)
            .add_systems(Update, quick_exit);
    }
}
