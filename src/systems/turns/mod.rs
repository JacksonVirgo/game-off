use crate::prelude::*;

pub mod turn;

plugin!(TurnPlugin, |app| {
    app.init_resource::<turn::Turn>();
});
