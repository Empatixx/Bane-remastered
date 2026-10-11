mod groups;
mod walls;

use std::fmt;

use glam::Vec2;
use hecs::Entity;
use rapier2d::prelude::{
    ColliderBuilder, ColliderHandle, PhysicsWorld, RigidBodyBuilder, RigidBodyHandle, Vector,
};

use crate::components::Body;

const WALL_OWNER: u128 = 0;
const PLAYER_CORNER_RADIUS: f32 = 2.0;
const LENGTH_UNIT: f32 = 32.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Owner {
    Wall,
    Entity(Entity),
}

pub(crate) struct Physics {
    world: PhysicsWorld,
}

impl fmt::Debug for Physics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Physics")
            .field("bodies", &self.world.bodies.len())
            .field("colliders", &self.world.colliders.len())
            .finish()
    }
}

impl Default for Physics {
    fn default() -> Self {
        Self::new()
    }
}

impl Physics {
    pub(crate) fn new() -> Self {
        let mut world = PhysicsWorld {
            gravity: Vector::new(0.0, 0.0),
            ..PhysicsWorld::default()
        };
        world.integration_parameters.length_unit = LENGTH_UNIT;
        Self { world }
    }

    pub(crate) fn step(&mut self, dt: f32) {
        self.world.integration_parameters.dt = dt;
        self.world.step();
    }

    pub(crate) fn add_walls(&mut self, solid_cells: &[(usize, usize)], tile_size: f32) {
        for rect in walls::merge_cells(solid_cells) {
            let size = Vec2::new(rect.width as f32, rect.height as f32) * tile_size;
            let center = Vec2::new(rect.column as f32, rect.row as f32) * tile_size + size / 2.0;
            let collider = ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0)
                .translation(to_rapier(center))
                .collision_groups(groups::WALLS)
                .user_data(WALL_OWNER);
            self.world.colliders.insert(collider);
        }
    }

    pub(crate) fn add_static_box(&mut self, center: Vec2, size: Vec2) -> ColliderHandle {
        let collider = ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0)
            .translation(to_rapier(center))
            .collision_groups(groups::WALLS)
            .user_data(WALL_OWNER);
        self.world.colliders.insert(collider)
    }

    pub(crate) fn remove_collider(&mut self, collider: ColliderHandle) {
        self.world.remove_collider(collider);
    }

    pub(crate) fn add_player(&mut self, entity: Entity, position: Vec2, size: Vec2) -> Body {
        let half = size / 2.0 - Vec2::splat(PLAYER_CORNER_RADIUS);
        let body = RigidBodyBuilder::dynamic()
            .translation(to_rapier(position))
            .lock_rotations();
        let collider = ColliderBuilder::round_cuboid(half.x, half.y, PLAYER_CORNER_RADIUS)
            .collision_groups(groups::PLAYERS)
            .user_data(owner_bits(entity));
        self.insert(body, collider)
    }

    pub(crate) fn add_enemy(&mut self, entity: Entity, position: Vec2, size: Vec2) -> Body {
        let body = RigidBodyBuilder::dynamic()
            .translation(to_rapier(position))
            .lock_rotations();
        let collider = ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0)
            .collision_groups(groups::ENEMIES)
            .user_data(owner_bits(entity));
        self.insert(body, collider)
    }

    pub(crate) fn add_bullet(
        &mut self,
        entity: Entity,
        position: Vec2,
        velocity: Vec2,
        size: Vec2,
    ) -> Body {
        let body = RigidBodyBuilder::dynamic()
            .translation(to_rapier(position))
            .linvel(to_rapier(velocity))
            .lock_rotations();
        let collider = ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0)
            .sensor(true)
            .collision_groups(groups::BULLETS)
            .user_data(owner_bits(entity));
        self.insert(body, collider)
    }

    pub(crate) fn add_enemy_bullet(
        &mut self,
        entity: Entity,
        position: Vec2,
        velocity: Vec2,
        size: Vec2,
    ) -> Body {
        let body = RigidBodyBuilder::dynamic()
            .translation(to_rapier(position))
            .linvel(to_rapier(velocity))
            .lock_rotations();
        let collider = ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0)
            .sensor(true)
            .collision_groups(groups::ENEMY_BULLETS)
            .user_data(owner_bits(entity));
        self.insert(body, collider)
    }

    pub(crate) fn add_object_box(
        &mut self,
        entity: Entity,
        center: Vec2,
        size: Vec2,
    ) -> ColliderHandle {
        let collider = ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0)
            .translation(to_rapier(center))
            .collision_groups(groups::OBJECTS)
            .user_data(owner_bits(entity));
        self.world.colliders.insert(collider)
    }

    pub(crate) fn remove(&mut self, body: Body) {
        self.world.remove_body(body.body);
    }

    pub(crate) fn set_velocity(&mut self, body: Body, velocity: Vec2) {
        if let Some(rigid_body) = self.world.bodies.get_mut(body.body) {
            rigid_body.set_linvel(to_rapier(velocity), true);
        }
    }

    pub(crate) fn velocity(&self, body: Body) -> Option<Vec2> {
        self.world
            .bodies
            .get(body.body)
            .map(|rigid_body| from_rapier(rigid_body.linvel()))
    }

    pub(crate) fn position(&self, body: Body) -> Option<Vec2> {
        self.world
            .bodies
            .get(body.body)
            .map(|rigid_body| from_rapier(rigid_body.translation()))
    }

    pub(crate) fn touching(&self, body: Body) -> Vec<Owner> {
        self.world
            .narrow_phase
            .intersection_pairs_with(body.collider)
            .filter(|&(_, _, intersecting)| intersecting)
            .filter_map(|(first, second, _)| {
                let other = if first == body.collider {
                    second
                } else {
                    first
                };
                self.owner(other)
            })
            .collect()
    }

    fn owner(&self, collider: ColliderHandle) -> Option<Owner> {
        let bits = self.world.colliders.get(collider)?.user_data;
        if bits == WALL_OWNER {
            return Some(Owner::Wall);
        }
        Entity::from_bits(u64::try_from(bits).ok()?).map(Owner::Entity)
    }

    fn insert(&mut self, body: RigidBodyBuilder, collider: ColliderBuilder) -> Body {
        let (body, collider): (RigidBodyHandle, ColliderHandle) = self.world.insert(body, collider);
        Body { body, collider }
    }
}

fn owner_bits(entity: Entity) -> u128 {
    u128::from(entity.to_bits().get())
}

fn to_rapier(vector: Vec2) -> Vector {
    Vector::new(vector.x, vector.y)
}

fn from_rapier(vector: Vector) -> Vec2 {
    Vec2::new(vector.x, vector.y)
}
