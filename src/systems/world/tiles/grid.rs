use bevy::{platform::collections::HashMap, prelude::*};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Resource, Default)]
pub struct HexGrid {
    pub map: HashMap<Coord, Entity>,
    pub reverse: HashMap<Entity, Coord>,
}

impl HexGrid {
    pub fn neighbour_coords(&self, coord: Coord) -> [Coord; 6] {
        let mut neighbors = [Coord { q: 0, r: 0 }; 6];
        for (i, (dq, dr)) in AXIAL_DIRS.iter().enumerate() {
            neighbors[i] = Coord {
                q: coord.q + dq,
                r: coord.r + dr,
            };
        }
        neighbors
    }
}

pub const AXIAL_DIRS: [(i32, i32); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];

pub fn axial_to_world(coord: Coord, hex_width: f32) -> Vec3 {
    let size = hex_width / 2.0;
    let sqrt3 = 3f32.sqrt();
    let qf = coord.q as f32;
    let rf = coord.r as f32;
    let x = size * 1.5 * qf;
    let y = size * sqrt3 * (rf + qf / 2.0);
    Vec3::new(x, y, 0.0)
}

pub fn world_to_axial(pos: Vec2, hex_width: f32) -> Coord {
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
        _ = -rx - rz;
    } else {
        rz = -rx - ry;
    }
    Coord {
        q: rx as i32,
        r: rz as i32,
    }
}
