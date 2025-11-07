use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TurnPhase {
    Player,
    Environment,
}

impl TurnPhase {
    pub fn next(&mut self) {
        *self = match self {
            TurnPhase::Player => TurnPhase::Environment,
            TurnPhase::Environment => TurnPhase::Player,
        };
    }
}

#[derive(Resource, Debug, Clone)]
pub struct Turn {
    pub phase: TurnPhase,
    pub count: u32,
}

impl Default for Turn {
    fn default() -> Self {
        Self {
            phase: TurnPhase::Player,
            count: 1,
        }
    }
}

impl Turn {
    pub fn next(&mut self) {
        self.phase.next();
        if let TurnPhase::Player = self.phase {
            self.count += 1;
        }
    }

    pub fn reset(&mut self) {
        self.phase = TurnPhase::Player;
        self.count = 1;
    }
}
