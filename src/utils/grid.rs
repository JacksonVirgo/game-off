use bevy::math::Vec3;

use crate::systems::world::tiles::data::TileKind;

pub fn generate_hex_grid(rings: i32, hex_width: f32) -> Vec<(TileKind, Vec3)> {
    let size = hex_width / 2.0;
    let sqrt3 = 3f32.sqrt();
    let mut set: Vec<(TileKind, Vec3)> = vec![];

    set.push((TileKind::Core, Vec3::new(0.0, 0.0, 0.0)));
    for radius in 1..=rings {
        let mut q = radius;
        let mut r = -radius;

        let directions = [(0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1), (1, 0)];

        for (dq, dr) in directions {
            for _ in 0..radius {
                let x = size * 1.5 * (q as f32);
                let y = size * sqrt3 * (r as f32 + (q as f32) / 2.0);

                set.push((TileKind::Empty, Vec3::new(x, y, 0.0)));

                q += dq;
                r += dr;
            }
        }
    }

    set
}
