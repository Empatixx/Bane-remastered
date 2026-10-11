use glam::Vec2;
use hecs::{Entity, World};
use rapier2d::prelude::ColliderHandle;

use crate::assets::Textures;
use crate::components::{Expiring, Player, Position, RoomId};
use crate::dungeon::{Dungeon, Rng, RoomKind, ROOM_COLUMNS, ROOM_ROWS};
use crate::physics::Physics;
use crate::prefabs;
use crate::render::TextureId;
use crate::tilemap::TILE_SIZE;

const MAX_ENEMIES: usize = 5;
const ENTER_MARGIN: usize = 2;

#[derive(Debug)]
enum RoomState {
    Unvisited,
    Fighting(Vec<(Entity, ColliderHandle)>),
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
    pub(crate) rat_texture: TextureId,
}

pub(crate) fn update_rooms(
    runtime: &mut RoomRuntime,
    context: &RoomContext,
    world: &mut World,
    physics: &mut Physics,
    rng: &mut Rng,
) {
    if let Some(index) = player_tile(context.dungeon, world) {
        runtime.current = index;
        runtime.entered[index] = true;
    }
    if let Some(room) = player_room(context.dungeon, world) {
        if matches!(runtime.states[room], RoomState::Unvisited) {
            enter(runtime, context, room, world, physics, rng);
        }
    }
    for room in 0..runtime.states.len() {
        let finished = matches!(runtime.states[room], RoomState::Fighting(_))
            && living_enemies(world, room) == 0;
        if finished {
            if let RoomState::Fighting(gates) =
                std::mem::replace(&mut runtime.states[room], RoomState::Cleared)
            {
                for (entity, collider) in gates {
                    let _ = world.despawn(entity);
                    physics.remove_collider(collider);
                }
            }
        }
    }
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
    let origin = dungeon.rooms[room].origin;
    let inside = column >= origin.0 + ENTER_MARGIN
        && column < origin.0 + ROOM_COLUMNS - ENTER_MARGIN
        && row >= origin.1 + ENTER_MARGIN
        && row < origin.1 + ROOM_ROWS - ENTER_MARGIN;
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

fn enter(
    runtime: &mut RoomRuntime,
    context: &RoomContext,
    room: usize,
    world: &mut World,
    physics: &mut Physics,
    rng: &mut Rng,
) {
    let data = &context.dungeon.rooms[room];
    let fights = matches!(data.kind, RoomKind::Normal | RoomKind::Boss) && !data.spawns.is_empty();
    if !fights {
        runtime.states[room] = RoomState::Cleared;
        return;
    }
    let mut spawns = data.spawns.clone();
    for index in (1..spawns.len()).rev() {
        spawns.swap(index, rng.below(index + 1));
    }
    for &(column, row) in spawns.iter().take(MAX_ENEMIES) {
        let position = Vec2::new(column as f32 + 0.5, row as f32 + 0.5) * TILE_SIZE;
        prefabs::rat_in_room(world, physics, position, context.rat_texture, room);
    }
    let mut gates = Vec::new();
    for door in &data.doors {
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
    runtime.states[room] = RoomState::Fighting(gates);
}

#[cfg(test)]
mod tests {
    use crate::dungeon::Rng;

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
        let center = dungeon.rooms[normal].center(TILE_SIZE);
        prefabs::player(&mut world, &mut physics, center, &textures);
        let mut runtime = RoomRuntime::new(&dungeon);
        let context = RoomContext {
            dungeon: &dungeon,
            textures: &textures,
            rat_texture: TextureId::WHITE,
        };
        let mut rng = Rng::new(1);

        update_rooms(&mut runtime, &context, &mut world, &mut physics, &mut rng);
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
        update_rooms(&mut runtime, &context, &mut world, &mut physics, &mut rng);
        assert!(!runtime.is_locked(normal));
    }

    #[test]
    fn the_start_room_never_locks() {
        let dungeon = dungeon();
        let start = dungeon.start_room().expect("there is a start room");
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(&mut world, &mut physics, start.center(TILE_SIZE), &textures);
        let mut runtime = RoomRuntime::new(&dungeon);
        let context = RoomContext {
            dungeon: &dungeon,
            textures: &textures,
            rat_texture: TextureId::WHITE,
        };

        update_rooms(
            &mut runtime,
            &context,
            &mut world,
            &mut physics,
            &mut Rng::new(1),
        );

        assert!(!(0..dungeon.rooms.len()).any(|room| runtime.is_locked(room)));
        assert_eq!(world.query::<&RoomId>().iter().count(), 0);
    }
}
