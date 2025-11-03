use crate::prelude::*;

pub mod tiles;

plugin!(WorldPlugin, |app| {
    app.add_plugins(tiles::TilePlugin);
});
