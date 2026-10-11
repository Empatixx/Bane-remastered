use std::collections::VecDeque;

use glam::{Vec2, Vec4};
use hecs::World;

use crate::components::{MapSize, MiniRoom, Minimap, Sprite};
use crate::dungeon::{Direction, RoomKind};
use crate::render::{Draw, Quad, TextureId};

const ICON: f32 = 16.0;
const SMALL_CENTER: Vec2 = Vec2::new(865.0, 94.0);
const BIG_CENTER: Vec2 = Vec2::new(480.0, 270.0);
const BIG_SCALE: f32 = 4.0;
const ALPHA_DECREASE: f32 = 0.25;
const UNVISITED_PENALTY: f32 = 0.35;
const OUTLINE_WIDTH: f32 = 0.0625;
const GATE_WIDTH: f32 = 0.4325;
const SLIDE: f32 = 0.2;
const GLOW_SPEED: f32 = 2.0;
const GATE_COLOR: Vec4 = Vec4::new(0.8196, 0.5333, 0.4117, 1.0);
const GLOW_COLOR: Vec3Rgb = (1.0, 0.25, 0.25);
const FOG_TINT: (f32, f32, f32) = (0.35, 0.35, 0.42);

type Vec3Rgb = (f32, f32, f32);

fn frame(kind: RoomKind) -> usize {
    match kind {
        RoomKind::Start => 0,
        RoomKind::Normal => 1,
        RoomKind::Treasure => 2,
        RoomKind::Shop => 3,
        RoomKind::Boss => 4,
        RoomKind::Sacrifice => 5,
    }
}

pub(crate) fn rooms_for(
    dungeon_rooms: impl Iterator<Item = ((i32, i32), RoomKind, Vec<Direction>)>,
) -> Vec<MiniRoom> {
    dungeon_rooms
        .map(|(cell, kind, links)| MiniRoom {
            cell,
            kind,
            links,
            visited: false,
            seen: false,
            distance: usize::MAX,
        })
        .collect()
}

pub(crate) fn update_minimaps(world: &mut World, visited: &[bool], current: usize, dt: f32) {
    for map in world.query_mut::<&mut Minimap>() {
        refresh(map, visited, current);
        let target = Vec2::new(
            map.rooms[map.current].cell.0 as f32,
            map.rooms[map.current].cell.1 as f32,
        );
        map.shown += (target - map.shown) * SLIDE.max((dt * 12.0).min(1.0));
        map.glow += dt;
    }
}

fn refresh(map: &mut Minimap, visited: &[bool], current: usize) {
    map.current = current.min(map.rooms.len().saturating_sub(1));
    for (room, seen_by_player) in map.rooms.iter_mut().zip(visited) {
        room.visited = *seen_by_player;
    }
    let cells: Vec<(i32, i32)> = map.rooms.iter().map(|room| room.cell).collect();
    let find = |cell: (i32, i32)| cells.iter().position(|candidate| *candidate == cell);
    let neighbours: Vec<Vec<usize>> = map
        .rooms
        .iter()
        .map(|room| {
            room.links
                .iter()
                .filter_map(|direction| {
                    let (dx, dy) = direction.delta();
                    find((room.cell.0 + dx, room.cell.1 + dy))
                })
                .collect()
        })
        .collect();
    for room in &mut map.rooms {
        room.distance = usize::MAX;
        room.seen = room.visited;
    }
    let mut queue = VecDeque::from([map.current]);
    map.rooms[map.current].distance = 0;
    while let Some(index) = queue.pop_front() {
        for &next in &neighbours[index] {
            if map.rooms[next].distance == usize::MAX {
                map.rooms[next].distance = map.rooms[index].distance + 1;
                queue.push_back(next);
            }
        }
    }
    let visited_rooms: Vec<usize> = (0..map.rooms.len())
        .filter(|&index| map.rooms[index].visited)
        .collect();
    for index in visited_rooms {
        for &next in &neighbours[index] {
            map.rooms[next].seen = true;
        }
    }
}

fn alpha(room: &MiniRoom, size: MapSize) -> f32 {
    match size {
        MapSize::Big => {
            if room.seen {
                1.0
            } else {
                0.0
            }
        }
        MapSize::Small => {
            let distance = room.distance as f32;
            1.0 - ALPHA_DECREASE * distance.powf(1.25)
                - if room.distance > 1 && !room.visited {
                    UNVISITED_PENALTY
                } else {
                    0.0
                }
        }
    }
}

pub(crate) fn minimap_draws(map: &Minimap) -> Vec<Draw> {
    let (center, scale) = match map.size {
        MapSize::Small => (SMALL_CENTER, 1.0),
        MapSize::Big => (BIG_CENTER, BIG_SCALE),
    };
    let icon = ICON * scale;
    let mut draws = Vec::new();
    for (index, room) in map.rooms.iter().enumerate() {
        let alpha = alpha(room, map.size);
        if alpha <= 0.0 || room.distance == usize::MAX && map.size == MapSize::Small {
            continue;
        }
        let shown = Vec2::new(room.cell.0 as f32, room.cell.1 as f32) - map.shown;
        let position = center + shown * icon;
        let tint = if room.visited {
            Vec4::new(1.0, 1.0, 1.0, alpha)
        } else {
            Vec4::new(FOG_TINT.0, FOG_TINT.1, FOG_TINT.2, alpha)
        };
        draws.push(Draw {
            texture: map.icons,
            quad: Quad::with_uv(
                position,
                Vec2::splat(icon),
                tint,
                Sprite::cell_uv(frame(room.kind), 0, 6, 1),
            ),
        });
        for direction in &room.links {
            draws.push(solid(
                gate_rect(position, icon, *direction),
                Vec4::new(GATE_COLOR.x, GATE_COLOR.y, GATE_COLOR.z, alpha),
            ));
        }
        if index == map.current {
            let pulse = 0.5 + 0.5 * (map.glow * GLOW_SPEED).sin();
            let color = Vec4::new(GLOW_COLOR.0, GLOW_COLOR.1, GLOW_COLOR.2, pulse);
            for edge in outline(position, icon) {
                draws.push(solid(edge, color));
            }
        }
    }
    draws
}

fn solid(rect: (Vec2, Vec2), color: Vec4) -> Draw {
    Draw {
        texture: TextureId::WHITE,
        quad: Quad::new(rect.0, rect.1, color),
    }
}

fn gate_rect(center: Vec2, icon: f32, direction: Direction) -> (Vec2, Vec2) {
    let thickness = icon * OUTLINE_WIDTH;
    let length = icon * GATE_WIDTH;
    let offset = icon / 2.0 - thickness / 2.0;
    match direction {
        Direction::Left => (
            center + Vec2::new(-offset, 0.0),
            Vec2::new(thickness, length),
        ),
        Direction::Right => (
            center + Vec2::new(offset, 0.0),
            Vec2::new(thickness, length),
        ),
        Direction::Up => (
            center + Vec2::new(0.0, -offset),
            Vec2::new(length, thickness),
        ),
        Direction::Down => (
            center + Vec2::new(0.0, offset),
            Vec2::new(length, thickness),
        ),
    }
}

fn outline(center: Vec2, icon: f32) -> [(Vec2, Vec2); 4] {
    let thickness = icon * OUTLINE_WIDTH;
    let offset = icon / 2.0 - thickness / 2.0;
    [
        (center + Vec2::new(0.0, -offset), Vec2::new(icon, thickness)),
        (center + Vec2::new(0.0, offset), Vec2::new(icon, thickness)),
        (center + Vec2::new(-offset, 0.0), Vec2::new(thickness, icon)),
        (center + Vec2::new(offset, 0.0), Vec2::new(thickness, icon)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map(size: MapSize) -> Minimap {
        let rooms = rooms_for(
            [
                ((0, 0), RoomKind::Start, vec![Direction::Right]),
                (
                    (1, 0),
                    RoomKind::Normal,
                    vec![Direction::Left, Direction::Right],
                ),
                (
                    (2, 0),
                    RoomKind::Normal,
                    vec![Direction::Left, Direction::Right],
                ),
                ((3, 0), RoomKind::Boss, vec![Direction::Left]),
            ]
            .into_iter(),
        );
        let mut map = Minimap {
            rooms,
            current: 0,
            shown: Vec2::ZERO,
            glow: 0.0,
            icons: TextureId::WHITE,
            size,
        };
        refresh(&mut map, &[true, false, false, false], 0);
        map
    }

    #[test]
    fn distances_come_from_the_current_room_and_neighbours_of_visited_rooms_are_seen() {
        let map = map(MapSize::Small);
        let distances: Vec<usize> = map.rooms.iter().map(|room| room.distance).collect();
        assert_eq!(distances, vec![0, 1, 2, 3]);
        let seen: Vec<bool> = map.rooms.iter().map(|room| room.seen).collect();
        assert_eq!(seen, vec![true, true, false, false]);
    }

    #[test]
    fn small_map_rooms_fade_with_distance_and_unvisited_far_rooms_fade_faster() {
        let map = map(MapSize::Small);
        let alphas: Vec<f32> = map
            .rooms
            .iter()
            .map(|room| alpha(room, MapSize::Small))
            .collect();
        assert!((alphas[0] - 1.0).abs() < 0.001);
        assert!(alphas[1] < alphas[0]);
        assert!(alphas[2] < alphas[1] - UNVISITED_PENALTY + 0.01);
        assert!(alphas[3] <= 0.0);
    }

    #[test]
    fn the_big_map_shows_only_rooms_the_player_has_seen() {
        let map = map(MapSize::Big);
        let shown = minimap_draws(&map).len();
        assert!(shown > 0);
        let seen = map.rooms.iter().filter(|room| room.seen).count();
        assert_eq!(seen, 2);
    }

    #[test]
    fn the_current_room_is_outlined_and_linked_rooms_get_gate_marks() {
        let map = map(MapSize::Small);
        let draws = minimap_draws(&map);
        let solids = draws
            .iter()
            .filter(|draw| draw.texture == TextureId::WHITE)
            .count();
        assert!(solids >= 5, "outline (4) and a gate mark: {solids}");
    }

    #[test]
    fn the_map_slides_towards_the_new_current_room() {
        let mut world = World::new();
        let mut map = map(MapSize::Small);
        map.rooms[1].visited = true;
        world.spawn((map,));
        update_minimaps(&mut world, &[true, true, false, false], 1, 0.5);
        let shown = world
            .query::<&Minimap>()
            .iter()
            .next()
            .map(|map| map.shown.x)
            .unwrap_or(0.0);
        assert!(shown > 0.0 && shown <= 1.0, "{shown}");
    }
}
