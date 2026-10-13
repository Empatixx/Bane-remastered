use glam::Vec2;
use hecs::{Entity, World};
use rapier2d::prelude::ColliderHandle;

use crate::assets::Textures;
use crate::components::{Expiring, Player, Position, RoomId};
use crate::dungeon::{Dungeon, RoomKind};
use crate::enemies;
use crate::physics::Physics;
use crate::prefabs;
use crate::rng::Rng;
use crate::tilemap::TILE_SIZE;

const MIN_ENEMIES: usize = 5;
const MAX_ENEMIES: usize = 12;
const AREA_PER_ENEMY: usize = 100;

fn enemy_limit(columns: usize, rows: usize) -> usize {
    (columns * rows / AREA_PER_ENEMY).clamp(MIN_ENEMIES, MAX_ENEMIES)
}
const ENTER_MARGIN: usize = 2;
const WAVE_DELAY: f32 = 1.5;
const AMBUSH_WAVES: usize = 3;
const AMBUSH_SHARE: f32 = 0.7;
const MIN_WAVE: usize = 4;

pub(crate) fn wave_plan(kind: RoomKind, columns: usize, rows: usize) -> (usize, usize) {
    let limit = enemy_limit(columns, rows);
    match kind {
        RoomKind::Ambush => {
            let per_wave = ((limit as f32 * AMBUSH_SHARE).ceil() as usize).max(MIN_WAVE);
            (AMBUSH_WAVES, per_wave.min(limit))
        }
        _ => (1, limit),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum RoomEvent {
    BossEntered,
    WaveStarted { number: usize, total: usize },
    AmbushCleared { center: Vec2 },
    ShopEntered,
    RoomCleared,
    BossDefeated { center: Vec2 },
}

#[derive(Debug)]
struct Fight {
    gates: Vec<(Entity, ColliderHandle)>,
    wave: usize,
    waves: usize,
    delay: Option<f32>,
}

#[derive(Debug)]
enum RoomState {
    Unvisited,
    Fighting(Fight),
    Cleared,
}

#[derive(Debug)]
pub(crate) struct RoomRuntime {
    states: Vec<RoomState>,
    entered: Vec<bool>,
    current: usize,
}

impl RoomRuntime {
    pub(crate) fn new(dungeon: &Dungeon) -> Self {
        Self {
            states: dungeon.rooms.iter().map(|_| RoomState::Unvisited).collect(),
            entered: vec![false; dungeon.rooms.len()],
            current: 0,
        }
    }

    pub(crate) fn reveal_all(&mut self) {
        self.entered.iter_mut().for_each(|entered| *entered = true);
    }

    pub(crate) fn entered(&self) -> &[bool] {
        &self.entered
    }

    pub(crate) fn current(&self) -> usize {
        self.current
    }

    #[cfg(test)]
    pub(crate) fn is_locked(&self, room: usize) -> bool {
        matches!(self.states.get(room), Some(RoomState::Fighting(_)))
    }
}

pub(crate) struct RoomContext<'a> {
    pub(crate) dungeon: &'a Dungeon,
    pub(crate) textures: &'a Textures,
    pub(crate) floor: u32,
}

pub(crate) fn update_rooms(
    runtime: &mut RoomRuntime,
    context: &RoomContext,
    world: &mut World,
    physics: &mut Physics,
    rng: &mut Rng,
    dt: f32,
) -> Vec<RoomEvent> {
    let mut events = Vec::new();
    if let Some(index) = player_tile(context.dungeon, world) {
        runtime.current = index;
        runtime.entered[index] = true;
    }
    if let Some(room) = player_room(context.dungeon, world) {
        if matches!(runtime.states[room], RoomState::Unvisited) {
            events.extend(enter(runtime, context, room, world, physics, rng));
        }
    }
    for room in 0..runtime.states.len() {
        if !matches!(runtime.states[room], RoomState::Fighting(_))
            || living_enemies(world, room) > 0
        {
            continue;
        }
        let data = &context.dungeon.rooms[room];
        let next_wave = match &mut runtime.states[room] {
            RoomState::Fighting(fight) if fight.wave < fight.waves => {
                let delay = fight.delay.get_or_insert(WAVE_DELAY);
                *delay -= dt;
                if *delay <= 0.0 {
                    fight.delay = None;
                    fight.wave += 1;
                    Some((fight.wave, fight.waves))
                } else {
                    None
                }
            }
            _ => {
                if let RoomState::Fighting(fight) =
                    std::mem::replace(&mut runtime.states[room], RoomState::Cleared)
                {
                    for (entity, collider) in fight.gates {
                        physics.remove_collider(collider);
                        super::open_gate(world, entity);
                    }
                }
                events.push(RoomEvent::RoomCleared);
                match data.kind {
                    RoomKind::Boss => events.push(RoomEvent::BossDefeated {
                        center: data.pixel_center(),
                    }),
                    RoomKind::Ambush => events.push(RoomEvent::AmbushCleared {
                        center: data.pixel_center(),
                    }),
                    _ => {}
                }
                continue;
            }
        };
        if let Some((number, total)) = next_wave {
            spawn_wave(context, room, world, rng);
            events.push(RoomEvent::WaveStarted { number, total });
        }
    }
    events
}

fn player_position(world: &World) -> Option<Vec2> {
    world
        .query::<&Position>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|position| position.0)
}

fn player_tile(dungeon: &Dungeon, world: &World) -> Option<usize> {
    let position = player_position(world)?;
    dungeon.room_at(
        (position.x / TILE_SIZE) as usize,
        (position.y / TILE_SIZE) as usize,
    )
}

fn player_room(dungeon: &Dungeon, world: &World) -> Option<usize> {
    let position = player_position(world)?;
    let column = (position.x / TILE_SIZE) as usize;
    let row = (position.y / TILE_SIZE) as usize;
    let room = dungeon.room_at(column, row)?;
    let inside = dungeon.rooms[room].contains_inner_tile(column, row, ENTER_MARGIN);
    inside.then_some(room)
}

fn living_enemies(world: &World, room: usize) -> usize {
    world
        .query::<&RoomId>()
        .without::<&Expiring>()
        .iter()
        .filter(|id| id.0 == room)
        .count()
}

fn spawn_wave(context: &RoomContext, room: usize, world: &mut World, rng: &mut Rng) {
    let data = &context.dungeon.rooms[room];
    let (_, per_wave) = wave_plan(data.kind, data.columns, data.rows);
    let mut spawns = data.spawns.clone();
    for index in (1..spawns.len()).rev() {
        spawns.swap(index, rng.below(index + 1));
    }
    for &(column, row) in spawns.iter().take(per_wave) {
        let position = Vec2::new(column as f32 + 0.5, row as f32 + 0.5) * TILE_SIZE;
        let kind = enemies::pick(context.floor, rng);
        prefabs::enemy_in_room(world, kind, position, context.floor, context.textures, room);
    }
}

fn enter(
    runtime: &mut RoomRuntime,
    context: &RoomContext,
    room: usize,
    world: &mut World,
    physics: &mut Physics,
    rng: &mut Rng,
) -> Option<RoomEvent> {
    let data = &context.dungeon.rooms[room];
    let boss = data.kind == RoomKind::Boss;
    let fights = boss
        || (matches!(data.kind, RoomKind::Normal | RoomKind::Ambush) && !data.spawns.is_empty());
    if !fights {
        runtime.states[room] = RoomState::Cleared;
        return (data.kind == RoomKind::Shop).then_some(RoomEvent::ShopEntered);
    }
    if boss {
        let kind = enemies::pick_boss(context.floor, rng);
        let center = data.pixel_center();
        let spec = kind.spec();
        prefabs::enemy_in_room(world, kind, center, context.floor, context.textures, room);
        if let enemies::Role::Boss(name) = spec.role {
            prefabs::boss_hud(world, name, context.textures);
        }
    } else {
        spawn_wave(context, room, world, rng);
    }
    let mut gates = Vec::new();
    for door in data
        .doors
        .iter()
        .filter(|door| door.leads_to != RoomKind::Secret)
    {
        for &tile in &door.tiles {
            gates.push(prefabs::gate(
                world,
                physics,
                tile,
                door.direction,
                context.textures,
            ));
        }
    }
    let waves = if boss {
        1
    } else {
        wave_plan(data.kind, data.columns, data.rows).0
    };
    runtime.states[room] = RoomState::Fighting(Fight {
        gates,
        wave: 1,
        waves,
        delay: None,
    });
    if boss {
        Some(RoomEvent::BossEntered)
    } else if waves > 1 {
        Some(RoomEvent::WaveStarted {
            number: 1,
            total: waves,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::rng::Rng;

    use super::*;

    fn dungeon() -> Dungeon {
        let mut rng = Rng::new(21);
        Dungeon::generate(&mut rng).expect("a dungeon is generated")
    }

    #[test]
    fn stepping_into_a_fighting_room_locks_it_and_clearing_it_opens_it_again() {
        let dungeon = dungeon();
        let normal = dungeon
            .rooms
            .iter()
            .position(|room| room.kind == RoomKind::Normal)
            .expect("there is a normal room");
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        let center = dungeon.rooms[normal].pixel_center();
        prefabs::player(&mut world, &mut physics, center, &textures);
        let mut runtime = RoomRuntime::new(&dungeon);
        let context = RoomContext {
            dungeon: &dungeon,
            textures: &textures,
            floor: 0,
        };
        let mut rng = Rng::new(1);

        update_rooms(
            &mut runtime,
            &context,
            &mut world,
            &mut physics,
            &mut rng,
            2.0,
        );
        assert!(runtime.is_locked(normal));
        assert!(living_enemies(&world, normal) > 0);

        let enemies: Vec<Entity> = world
            .query::<(Entity, &RoomId)>()
            .iter()
            .map(|(entity, _)| entity)
            .collect();
        for entity in enemies {
            let _ = world.insert_one(entity, Expiring);
        }
        let gates_before = world.query::<&crate::components::Gate>().iter().count();
        assert!(gates_before > 0);
        for _ in 0..4 {
            update_rooms(
                &mut runtime,
                &context,
                &mut world,
                &mut physics,
                &mut rng,
                2.0,
            );
            let enemies: Vec<Entity> = world
                .query::<(Entity, &RoomId)>()
                .iter()
                .map(|(entity, _)| entity)
                .collect();
            for entity in enemies {
                let _ = world.insert_one(entity, Expiring);
            }
        }
        update_rooms(
            &mut runtime,
            &context,
            &mut world,
            &mut physics,
            &mut rng,
            2.0,
        );
        assert!(!runtime.is_locked(normal));
        assert_eq!(
            world.query::<&crate::components::Gate>().iter().count(),
            gates_before
        );
        assert_eq!(
            world.query::<&crate::components::Opening>().iter().count(),
            gates_before
        );

        for _ in 0..60 {
            crate::systems::tick_openings(&mut world, 1.0 / 60.0);
        }
        assert_eq!(world.query::<&crate::components::Gate>().iter().count(), 0);
    }

    #[test]
    fn the_start_room_never_locks() {
        let dungeon = dungeon();
        let start = dungeon.start_room().expect("there is a start room");
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(&mut world, &mut physics, start.pixel_center(), &textures);
        let mut runtime = RoomRuntime::new(&dungeon);
        let context = RoomContext {
            dungeon: &dungeon,
            textures: &textures,
            floor: 0,
        };

        update_rooms(
            &mut runtime,
            &context,
            &mut world,
            &mut physics,
            &mut Rng::new(1),
            2.0,
        );

        assert!(!(0..dungeon.rooms.len()).any(|room| runtime.is_locked(room)));
        assert_eq!(world.query::<&RoomId>().iter().count(), 0);
    }

    #[test]
    fn the_boss_room_spawns_one_boss_and_announces_the_fight_and_the_victory() {
        let dungeon = dungeon();
        let boss_room = dungeon
            .rooms
            .iter()
            .position(|room| room.kind == RoomKind::Boss)
            .expect("there is a boss room");
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        let center = dungeon.rooms[boss_room].pixel_center();
        prefabs::player(
            &mut world,
            &mut physics,
            center + Vec2::new(0.0, 300.0),
            &textures,
        );
        let mut runtime = RoomRuntime::new(&dungeon);
        let context = RoomContext {
            dungeon: &dungeon,
            textures: &textures,
            floor: 1,
        };
        let mut rng = Rng::new(5);

        let events = update_rooms(
            &mut runtime,
            &context,
            &mut world,
            &mut physics,
            &mut rng,
            2.0,
        );

        assert_eq!(events, vec![RoomEvent::BossEntered]);
        assert!(runtime.is_locked(boss_room));
        assert_eq!(living_enemies(&world, boss_room), 1);
        let bosses = world.query::<&crate::components::Boss>().iter().count();
        assert_eq!(bosses, 1);
        assert_eq!(
            world.query::<&crate::components::BossHud>().iter().count(),
            2
        );

        let enemies: Vec<Entity> = world
            .query::<(Entity, &RoomId)>()
            .iter()
            .map(|(entity, _)| entity)
            .collect();
        for entity in enemies {
            let _ = world.insert_one(entity, Expiring);
        }
        let events = update_rooms(
            &mut runtime,
            &context,
            &mut world,
            &mut physics,
            &mut rng,
            2.0,
        );

        assert_eq!(
            events,
            vec![RoomEvent::RoomCleared, RoomEvent::BossDefeated { center }]
        );
        assert!(!runtime.is_locked(boss_room));
    }

    #[test]
    fn ordinary_rooms_do_not_raise_boss_events() {
        let dungeon = dungeon();
        let normal = dungeon
            .rooms
            .iter()
            .position(|room| room.kind == RoomKind::Normal)
            .expect("there is a normal room");
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(
            &mut world,
            &mut physics,
            dungeon.rooms[normal].pixel_center(),
            &textures,
        );
        let mut runtime = RoomRuntime::new(&dungeon);
        let context = RoomContext {
            dungeon: &dungeon,
            textures: &textures,
            floor: 1,
        };

        let events = update_rooms(
            &mut runtime,
            &context,
            &mut world,
            &mut physics,
            &mut Rng::new(2),
            2.0,
        );

        assert!(events.iter().all(|event| !matches!(
            event,
            RoomEvent::BossEntered | RoomEvent::BossDefeated { .. }
        )));
    }

    #[test]
    fn bigger_rooms_hold_more_enemies_but_the_old_sizes_keep_their_limit() {
        assert_eq!(enemy_limit(16, 10), MIN_ENEMIES);
        assert_eq!(enemy_limit(28, 20), MIN_ENEMIES);
        assert!(enemy_limit(34, 22) > MIN_ENEMIES);
        assert!(enemy_limit(38, 24) > enemy_limit(34, 22));
        assert_eq!(enemy_limit(60, 40), MAX_ENEMIES);
    }

    #[test]
    fn only_an_ambush_comes_in_waves_and_a_normal_room_is_one_group() {
        let (waves, per_wave) = wave_plan(RoomKind::Ambush, 34, 22);
        assert_eq!(waves, AMBUSH_WAVES);
        assert!(per_wave >= MIN_WAVE && per_wave <= enemy_limit(34, 22));
        assert!(wave_plan(RoomKind::Ambush, 38, 24).1 > wave_plan(RoomKind::Ambush, 32, 22).1);
        for (columns, rows) in [(22, 14), (28, 20), (38, 24)] {
            assert_eq!(
                wave_plan(RoomKind::Normal, columns, rows),
                (1, enemy_limit(columns, rows))
            );
        }
    }

    fn fight() -> (Dungeon, usize, World, Physics, Textures) {
        let dungeon = dungeon();
        let normal = dungeon
            .rooms
            .iter()
            .position(|room| room.kind == RoomKind::Ambush)
            .expect("there is an ambush room");
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(
            &mut world,
            &mut physics,
            dungeon.rooms[normal].pixel_center(),
            &textures,
        );
        (dungeon, normal, world, physics, textures)
    }

    fn kill_everyone(world: &mut World) {
        let enemies: Vec<Entity> = world
            .query::<(Entity, &RoomId)>()
            .iter()
            .map(|(entity, _)| entity)
            .collect();
        for entity in enemies {
            let _ = world.insert_one(entity, Expiring);
        }
    }

    #[test]
    fn an_ambush_room_sends_its_enemies_in_waves_and_stays_locked_until_the_last_one_is_dead() {
        let (dungeon, normal, mut world, mut physics, textures) = fight();
        let context = RoomContext {
            dungeon: &dungeon,
            textures: &textures,
            floor: 1,
        };
        let mut runtime = RoomRuntime::new(&dungeon);
        let mut rng = Rng::new(3);
        let room = &dungeon.rooms[normal];
        let (waves, per_wave) = wave_plan(room.kind, room.columns, room.rows);
        let wanted = per_wave.min(room.spawns.len());

        let events = update_rooms(
            &mut runtime,
            &context,
            &mut world,
            &mut physics,
            &mut rng,
            0.016,
        );
        assert_eq!(
            events,
            vec![RoomEvent::WaveStarted {
                number: 1,
                total: waves
            }]
        );
        assert_eq!(living_enemies(&world, normal), wanted);

        for number in 2..=waves {
            kill_everyone(&mut world);
            let early = update_rooms(
                &mut runtime,
                &context,
                &mut world,
                &mut physics,
                &mut rng,
                0.5,
            );
            assert!(early.is_empty(), "the next wave waits a moment");
            assert!(runtime.is_locked(normal));
            assert_eq!(living_enemies(&world, normal), 0);

            let later = update_rooms(
                &mut runtime,
                &context,
                &mut world,
                &mut physics,
                &mut rng,
                1.5,
            );
            assert_eq!(
                later,
                vec![RoomEvent::WaveStarted {
                    number,
                    total: waves
                }]
            );
            assert_eq!(living_enemies(&world, normal), wanted);
            assert!(runtime.is_locked(normal));
        }

        kill_everyone(&mut world);
        let last = update_rooms(
            &mut runtime,
            &context,
            &mut world,
            &mut physics,
            &mut rng,
            0.016,
        );
        assert_eq!(
            last,
            vec![
                RoomEvent::RoomCleared,
                RoomEvent::AmbushCleared {
                    center: room.pixel_center()
                }
            ]
        );
        assert!(!runtime.is_locked(normal));
    }

    #[test]
    fn the_boss_room_is_a_single_wave() {
        let dungeon = dungeon();
        let boss = dungeon
            .rooms
            .iter()
            .position(|room| room.kind == RoomKind::Boss)
            .expect("there is a boss room");
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(
            &mut world,
            &mut physics,
            dungeon.rooms[boss].pixel_center(),
            &textures,
        );
        let context = RoomContext {
            dungeon: &dungeon,
            textures: &textures,
            floor: 1,
        };
        let mut runtime = RoomRuntime::new(&dungeon);
        let mut rng = Rng::new(5);

        update_rooms(
            &mut runtime,
            &context,
            &mut world,
            &mut physics,
            &mut rng,
            0.016,
        );
        kill_everyone(&mut world);
        let events = update_rooms(
            &mut runtime,
            &context,
            &mut world,
            &mut physics,
            &mut rng,
            0.016,
        );

        assert!(events.contains(&RoomEvent::RoomCleared));
        assert!(
            !events
                .iter()
                .any(|event| matches!(event, RoomEvent::WaveStarted { .. }))
        );
    }
}
