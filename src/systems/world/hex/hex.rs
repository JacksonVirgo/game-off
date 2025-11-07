use crate::systems::world::hex::kind::HexKind;
use bevy::prelude::*;

#[derive(Component)]
pub struct Hex {
    pub kind: HexKind,
}

#[derive(Component)]
pub struct HexInner;

#[derive(Component)]
pub struct HexFlipAlert;
