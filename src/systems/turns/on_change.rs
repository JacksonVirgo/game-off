use bevy::prelude::*;

use crate::systems::turns::turn::Turn;

#[derive(Message, Debug, Clone)]
pub struct TurnChanged {
    pub old: Turn,
}
