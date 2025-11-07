use crate::prelude::*;

pub mod grid;
pub mod hex;

plugin!(WorldPlugin, |app| {
    app.init_resource::<grid::grid::HexGrid>();
    app.add_plugins(hex::HexPlugin);
});
