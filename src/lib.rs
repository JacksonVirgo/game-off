pub mod core;
pub mod macros;
pub mod systems;
pub mod utils;

pub mod prelude {
    pub use crate::{
        core::{input::InputActions, sets::OrderSet},
        plugin,
    };
}
