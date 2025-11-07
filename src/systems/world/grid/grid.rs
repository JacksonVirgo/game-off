use bevy::prelude::*;
use std::collections::HashMap;

use crate::systems::world::{
    grid::coord::Coord,
    hex::{kind::HexKind, relation::HexDirection, spawn::SpawnHex},
};

#[derive(Resource, Default)]
pub struct HexGrid {
    pub map: HashMap<Coord, Entity>,
}

impl HexGrid {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn generate(rings: i32) -> Vec<SpawnHex> {
        let mut set: Vec<SpawnHex> = vec![];

        let core_coord = Coord::new(0, 0);
        set.push(SpawnHex::new(HexKind::Core, core_coord));

        for radius in 1..=rings {
            let mut q = radius;
            let mut r = -radius;

            for dir in HexDirection::all() {
                let (dq, dr) = dir.to_offset();
                for _ in 0..radius {
                    let coord = Coord { q, r };
                    set.push(SpawnHex::new(HexKind::Unassigned, coord));
                    q += dq;
                    r += dr;
                }
            }
        }

        set
    }
}
