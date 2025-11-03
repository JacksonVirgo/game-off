use crate::core::{input::InputPlugin, sets::configure_order_set};

pub mod config;
pub mod core;
pub mod macros;
pub mod states;
pub mod systems;
pub mod utils;

pub mod prelude {
    pub use crate::{
        config::*,
        core::{input::InputActions, sets::OrderSet},
        plugin,
    };
}

plugin!(GlobalPlugin, |app| {
    configure_order_set(app);
    app.add_plugins((
        InputPlugin,
        systems::SystemPlugin,
        states::StateManagerPlugin,
    ));
});
