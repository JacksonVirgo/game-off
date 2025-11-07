use bevy::prelude::*;

use crate::systems::{
    turns::{
        on_change::TurnChanged,
        turn::{Turn, TurnPhase},
    },
    world::{
        grid::{coord::Coord, grid::HexGrid},
        hex::{
            DEFAULT_HUE_SPEED,
            hex::{Hex, HexFlipAlert, HexInner},
            kind::HexKind,
            relation::HexDirection,
        },
    },
};

pub fn update_tile_state(
    mut commands: Commands,
    q_tiles: Query<(Entity, &Hex, &Coord, Option<&HexFlipAlert>)>,
    turn: Res<Turn>,
    mut ev_turn_change: MessageReader<TurnChanged>,
    hex_grid: Res<HexGrid>,
) {
    if ev_turn_change.read().count() == 0 {
        return;
    }

    if turn.phase != TurnPhase::Environment {
        return;
    }

    let mut remove_alert: Vec<Entity> = vec![];
    let mut add_alert: Vec<Entity> = vec![];

    let alerted_tiles: Vec<(Entity, Coord)> = q_tiles
        .iter()
        .filter_map(|(entity, _, coord, opt)| {
            if opt.is_some() {
                Some((entity, coord.clone()))
            } else {
                None
            }
        })
        .collect();

    if alerted_tiles.len() == 0 {
        let Some((_, _, coord, _)) = q_tiles
            .iter()
            .find(|(_, hex, _, _)| hex.kind == HexKind::Core)
        else {
            return;
        };

        for dir in HexDirection::all() {
            let offset = dir.to_offset();
            match hex_grid
                .map
                .get(&Coord::new(coord.q + offset.0, coord.r + offset.1))
            {
                Some(target) => {
                    add_alert.push(target.clone());
                }
                _ => {}
            }
        }
    } else {
        for (entity, coord) in alerted_tiles.iter() {
            remove_alert.push(*entity);

            let current_distance = coord.q.abs() + coord.r.abs();

            for dir in HexDirection::all() {
                let offset = dir.to_offset();
                let neigh_coord = Coord::new(coord.q + offset.0, coord.r + offset.1);

                if let Some(&neigh_entity) = hex_grid.map.get(&neigh_coord) {
                    let neigh_distance = neigh_coord.q.abs() + neigh_coord.r.abs();
                    if neigh_distance > current_distance {
                        if let Ok((_, neigh_hex, _, neigh_flip)) = q_tiles.get(neigh_entity) {
                            if neigh_hex.kind != HexKind::Core && neigh_flip.is_none() {
                                add_alert.push(neigh_entity);
                            }
                        }
                    }
                }
            }
        }
    }

    for entity in remove_alert {
        commands.entity(entity).remove::<HexFlipAlert>();
    }

    for entity in add_alert {
        commands.entity(entity).insert(HexFlipAlert);
    }
}

pub fn update_tile_visuals(
    mut q_tiles: Query<(&ChildOf, &mut Sprite), With<HexInner>>,
    mut tiles: Query<(&Hex, &mut Sprite, Option<&HexFlipAlert>), Without<HexInner>>,
    time: Res<Time>,
) {
    let hue_speed = DEFAULT_HUE_SPEED;

    let mut core_color: Color = Color::WHITE;

    for (child_of, mut sprite) in &mut q_tiles {
        if let Ok((tile, mut frame_sprite, flip_alert)) = tiles.get_mut(child_of.parent()) {
            match tile.kind {
                HexKind::Core => {
                    let hue = sprite.color.hue();
                    let new_hue = (hue + hue_speed * time.delta_secs()) % 360.0;
                    sprite.color = Color::oklch(0.6, 0.1376, new_hue);
                    core_color = sprite.color.clone();
                }
                _ => {
                    sprite.color = tile.kind.to_color();
                }
            }

            if flip_alert.is_some() {
                frame_sprite.color = core_color;
            } else {
                frame_sprite.color = Color::WHITE;
            }
        }
    }
}

pub fn flip_tile() {}
