use std::collections::VecDeque;

use glam::{Vec2, Vec4};
use hecs::World;

use crate::components::{BigKind, MapSize, MiniRoom, Minimap, Sprite};
use crate::dungeon::{Direction, RoomKind};
use crate::render::{Draw, MapRoomLook, Quad};

const ICON: f32 = 16.0;
const SMALL_CENTER: Vec2 = Vec2::new(865.0, 94.0);
const BIG_CENTER: Vec2 = Vec2::new(480.0, 270.0);
const BIG_SCALE: f32 = 4.0;
const BIG_WINDOW_MIN: Vec2 = Vec2::new(300.0, 90.0);
const BIG_WINDOW_MAX: Vec2 = Vec2::new(660.0, 450.0);
const ALPHA_DECREASE: f32 = 0.25;
const UNVISITED_PENALTY: f32 = 0.35;
const SLIDE: f32 = 0.2;
const SEED_RANGE: f32 = 10.0;

fn frame(kind: RoomKind) -> usize {
    match kind {
        RoomKind::Start => 0,
        RoomKind::Normal => 1,
        RoomKind::Treasure => 2,
        RoomKind::Shop => 3,
        RoomKind::Boss => 4,
        RoomKind::Sacrifice => 5,
        RoomKind::Ambush => 6,
    }
}

fn link_bit(direction: Direction) -> u8 {
    match direction {
        Direction::Left => 1,
        Direction::Right => 2,
        Direction::Up => 4,
        Direction::Down => 8,
    }
}

fn seed_for(cell: (i32, i32)) -> f32 {
    let hash = (cell.0 as i64 * 73_856_093) ^ (cell.1 as i64 * 19_349_663);
    (hash.rem_euclid(1000) as f32 / 1000.0) * SEED_RANGE
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
            big: BigKind::Unseen,
            distance: usize::MAX,
        })
        .collect()
}

pub(crate) fn update_minimaps(world: &mut World, visited: &[bool], current: usize, dt: f32) {
    for map in world.query_mut::<&mut Minimap>() {
        let before = map.current;
        refresh(map, visited, current);
        if map.current != before {
            map.pan = Vec2::ZERO;
        }
        let target = Vec2::new(
            map.rooms[map.current].cell.0 as f32,
            map.rooms[map.current].cell.1 as f32,
        );
        map.shown += (target - map.shown) * SLIDE.max((dt * 12.0).min(1.0));
        map.glow += dt;
    }
}

pub(crate) fn pan_big_maps(world: &mut World, delta: Vec2) {
    for map in world.query_mut::<&mut Minimap>() {
        if map.size == MapSize::Big {
            map.pan += delta;
        }
    }
}

pub(crate) fn reset_big_map_pan(world: &mut World) {
    for map in world.query_mut::<&mut Minimap>() {
        map.pan = Vec2::ZERO;
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
        room.big = BigKind::Unseen;
    }
    let mut queue = VecDeque::from([map.current]);
    map.rooms[map.current].distance = 0;
    map.rooms[map.current].big = BigKind::Current;
    while let Some(index) = queue.pop_front() {
        for &next in &neighbours[index] {
            if map.rooms[next].distance == usize::MAX {
                map.rooms[next].distance = map.rooms[index].distance + 1;
                map.rooms[next].big = match map.rooms[index].big {
                    BigKind::Current | BigKind::Visited if map.rooms[next].visited => {
                        BigKind::Visited
                    }
                    BigKind::Current | BigKind::Visited => BigKind::Unvisited,
                    BigKind::Unvisited | BigKind::Unseen => BigKind::Unseen,
                };
                queue.push_back(next);
            }
        }
    }
}

fn alpha(room: &MiniRoom, size: MapSize) -> f32 {
    match size {
        MapSize::Big => room.big.alpha(),
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

struct Clipped {
    center: Vec2,
    size: Vec2,
    uv: [f32; 4],
    local: [f32; 4],
}

fn clip(center: Vec2, size: Vec2, uv: [f32; 4], min: Vec2, max: Vec2) -> Option<Clipped> {
    let low = (center - size / 2.0).max(min);
    let high = (center + size / 2.0).min(max);
    if high.x <= low.x || high.y <= low.y {
        return None;
    }
    let from = ((low - (center - size / 2.0)) / size).to_array();
    let to = ((high - (center - size / 2.0)) / size).to_array();
    let mix = |a: f32, b: f32, t: f32| a + (b - a) * t;
    Some(Clipped {
        center: (low + high) / 2.0,
        size: high - low,
        uv: [
            mix(uv[0], uv[2], from[0]),
            mix(uv[1], uv[3], from[1]),
            mix(uv[0], uv[2], to[0]),
            mix(uv[1], uv[3], to[1]),
        ],
        local: [from[0], from[1], to[0], to[1]],
    })
}

pub(crate) fn minimap_draws(map: &Minimap) -> Vec<Draw> {
    let (center, scale) = match map.size {
        MapSize::Small => (SMALL_CENTER, 1.0),
        MapSize::Big => (BIG_CENTER + map.pan, BIG_SCALE),
    };
    let icon = ICON * scale;
    let mut draws = Vec::new();
    for room in &map.rooms {
        let alpha = alpha(room, map.size);
        if alpha <= 0.0 {
            continue;
        }
        let shown = Vec2::new(room.cell.0 as f32, room.cell.1 as f32) - map.shown;
        let position = center + shown * icon;
        let uv = Sprite::cell_uv(frame(room.kind), 0, 7, 1);
        let placed = match map.size {
            MapSize::Small => Some(Clipped {
                center: position,
                size: Vec2::splat(icon),
                uv,
                local: [0.0, 0.0, 1.0, 1.0],
            }),
            MapSize::Big => clip(
                position,
                Vec2::splat(icon),
                uv,
                BIG_WINDOW_MIN,
                BIG_WINDOW_MAX,
            ),
        };
        let Some(placed) = placed else {
            continue;
        };
        let links = room
            .links
            .iter()
            .fold(0, |bits, direction| bits | link_bit(*direction));
        draws.push(Draw {
            texture: map.icons,
            quad: Quad::with_uv(
                placed.center,
                placed.size,
                Vec4::new(1.0, 1.0, 1.0, alpha),
                placed.uv,
            )
            .map_room(MapRoomLook {
                time: map.glow,
                seed: seed_for(room.cell),
                links,
                fogged: !room.visited,
                local: placed.local,
            }),
        });
    }
    draws
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
            pan: Vec2::ZERO,
            glow: 0.0,
            icons: crate::render::TextureId::WHITE,
            size,
        };
        refresh(&mut map, &[true, false, false, false], 0);
        map
    }

    #[test]
    fn distances_come_from_the_current_room_and_follow_the_links() {
        let map = map(MapSize::Small);
        let distances: Vec<usize> = map.rooms.iter().map(|room| room.distance).collect();
        assert_eq!(distances, vec![0, 1, 2, 3]);
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
    fn the_big_map_uses_bane_alphas_for_current_visited_unvisited_and_unseen_rooms() {
        let mut map = map(MapSize::Big);
        map.rooms[1].visited = true;
        refresh(&mut map, &[true, true, false, false], 0);
        let kinds: Vec<BigKind> = map.rooms.iter().map(|room| room.big).collect();
        assert_eq!(
            kinds,
            vec![
                BigKind::Current,
                BigKind::Visited,
                BigKind::Unvisited,
                BigKind::Unseen
            ]
        );
        let alphas: Vec<f32> = map
            .rooms
            .iter()
            .map(|room| alpha(room, MapSize::Big))
            .collect();
        assert_eq!(alphas, vec![1.0, 0.75, 0.5, 0.0]);
        assert_eq!(minimap_draws(&map).len(), 3);
    }

    #[test]
    fn rooms_behind_an_unvisited_room_stay_hidden_on_the_big_map() {
        let map = map(MapSize::Big);
        let kinds: Vec<BigKind> = map.rooms.iter().map(|room| room.big).collect();
        assert_eq!(
            kinds,
            vec![
                BigKind::Current,
                BigKind::Unvisited,
                BigKind::Unseen,
                BigKind::Unseen
            ]
        );
    }

    #[test]
    fn unvisited_rooms_are_fogged_and_visited_ones_are_not_and_nothing_is_outlined() {
        let mut map = map(MapSize::Small);
        map.rooms[1].visited = true;
        let draws = minimap_draws(&map);
        assert!(draws.iter().all(|draw| draw.texture == map.icons));
        assert_eq!(draws.len(), 3);
    }

    #[test]
    fn links_are_packed_into_direction_bits() {
        assert_eq!(link_bit(Direction::Left) | link_bit(Direction::Down), 9);
    }

    #[test]
    fn the_seed_is_stable_and_within_range() {
        for cell in [(0, 0), (3, -2), (-7, 5)] {
            let seed = seed_for(cell);
            assert_eq!(seed, seed_for(cell));
            assert!((0.0..SEED_RANGE).contains(&seed));
        }
    }

    #[test]
    fn clipping_trims_a_room_that_sticks_out_of_the_window() {
        let uv = [0.0, 0.0, 1.0, 1.0];
        let inside = clip(
            Vec2::new(400.0, 200.0),
            Vec2::splat(64.0),
            uv,
            BIG_WINDOW_MIN,
            BIG_WINDOW_MAX,
        );
        assert!(inside.is_some_and(|c| c.size == Vec2::splat(64.0)));
        let edge = clip(
            Vec2::new(300.0, 200.0),
            Vec2::splat(64.0),
            uv,
            BIG_WINDOW_MIN,
            BIG_WINDOW_MAX,
        );
        let edge = edge.expect("half of it is visible");
        assert_eq!(edge.size.x, 32.0);
        assert!((edge.uv[0] - 0.5).abs() < 1e-6 && (edge.local[0] - 0.5).abs() < 1e-6);
        assert!(
            clip(
                Vec2::new(100.0, 200.0),
                Vec2::splat(64.0),
                uv,
                BIG_WINDOW_MIN,
                BIG_WINDOW_MAX
            )
            .is_none()
        );
    }

    #[test]
    fn panning_moves_only_the_big_map_and_resets_when_the_room_changes() {
        let mut world = World::new();
        world.spawn((map(MapSize::Small),));
        world.spawn((map(MapSize::Big),));
        pan_big_maps(&mut world, Vec2::new(10.0, -5.0));
        let pans: Vec<(MapSize, Vec2)> = world
            .query::<&Minimap>()
            .iter()
            .map(|m| (m.size, m.pan))
            .collect();
        assert!(pans.contains(&(MapSize::Big, Vec2::new(10.0, -5.0))));
        assert!(pans.contains(&(MapSize::Small, Vec2::ZERO)));

        update_minimaps(&mut world, &[true, true, false, false], 1, 0.1);
        assert!(
            world
                .query::<&Minimap>()
                .iter()
                .all(|m| m.pan == Vec2::ZERO)
        );
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
