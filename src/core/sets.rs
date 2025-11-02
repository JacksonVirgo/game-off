use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum OrderSet {
    Input,
    Movement,
    Camera,
    Visual,
}

pub fn configure_order_set(app: &mut App) {
    app.configure_sets(
        Update,
        (
            OrderSet::Input,
            OrderSet::Movement.after(OrderSet::Input),
            OrderSet::Camera.after(OrderSet::Movement),
            OrderSet::Visual.after(OrderSet::Camera),
        ),
    );
}
