use crate::prelude::*;

pub mod materials;

plugin!(SystemPlugin, |app| {
    app.add_plugins(materials::MaterialPlugin);
});
