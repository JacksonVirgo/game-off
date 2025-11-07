use bevy::prelude::*;
use std::hash::{Hash, Hasher};

use crate::systems::world::hex::relation::HexDirection;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct Coord {
    pub q: i32,
    pub r: i32,
}

impl Hash for Coord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.q.hash(state);
        self.r.hash(state);
    }
}

impl Coord {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    pub fn neighbour(&self, dir: HexDirection) -> Coord {
        let (dq, dr) = dir.to_offset();
        Coord::new(self.q + dq, self.r + dr)
    }

    pub fn axial_to_world(&self, hex_width: f32, gap_mul: f32) -> Vec3 {
        let size = hex_width / 2.0;
        let sqrt3 = f32::sqrt(3.0);
        let qf = self.q as f32;
        let rf = self.r as f32;

        let x = size * 1.5 * qf;
        let y = size * sqrt3 * (rf + qf / 2.0);
        Vec3::new(x * gap_mul, y * gap_mul, 0.0)
    }

    pub fn world_to_axial(raw_pos: Vec2, hex_width: f32, gap_mul: f32) -> Coord {
        let pos = raw_pos / gap_mul;
        let size = hex_width / 2.0;
        let q = (2.0 / 3.0 * pos.x) / size;
        let r = ((-1.0 / 3.0 * pos.x) + (3f32.sqrt() / 3.0 * pos.y)) / size;
        let x = q;
        let z = r;
        let y = -x - z;
        let mut rx = x.round();
        let ry = y.round();
        let mut rz = z.round();
        let x_diff = (rx - x).abs();
        let y_diff = (ry - y).abs();
        let z_diff = (rz - z).abs();
        if x_diff > y_diff && x_diff > z_diff {
            rx = -ry - rz;
        } else if y_diff > z_diff {
            // Do nothing
        } else {
            rz = -rx - ry;
        }
        Coord {
            q: rx as i32,
            r: rz as i32,
        }
    }
}
