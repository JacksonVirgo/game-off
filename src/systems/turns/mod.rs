use crate::{prelude::*, systems::turns::on_change::TurnChanged};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

pub mod on_change;
pub mod turn;

plugin!(TurnPlugin, |app| {
    app.init_resource::<turn::Turn>();
    app.add_systems(Update, (turn_detected, advance_turn_system).chain());
    app.add_message::<TurnChanged>();
});

fn turn_detected(mut ev_turn_change: MessageReader<TurnChanged>, turn: Res<turn::Turn>) {
    for event in ev_turn_change.read() {
        info!(
            "TURN CHANGED from {} ({:?}) -> {} ({:?})",
            event.old.count, event.old.phase, turn.count, turn.phase
        );
    }
}

fn advance_turn_system(
    mut turn: ResMut<turn::Turn>,
    q_input: Query<&ActionState<InputActions>>,
    mut ev_turn_change: MessageWriter<TurnChanged>,
) {
    let Ok(input) = q_input.single() else {
        return;
    };

    if input.just_pressed(&InputActions::EndTurn) {
        ev_turn_change.write(TurnChanged { old: turn.clone() });
        turn.next();
    }
}
