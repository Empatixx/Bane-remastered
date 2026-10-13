mod groups;
mod walls;

use std::fmt;

use glam::Vec2;
use hecs::{Entity, World};
use rapier2d::prelude::{
    ColliderBuilder, ColliderHandle, PhysicsWorld, RigidBodyBuilder, RigidBodyHandle, Vector,
};

use crate::components::{Body, Solid};

const WALL_OWNER: u128 = 0;
const PLAYER_CORNER_RADIUS: f32 = 2.0;
const ENEMY_CORNER_FRACTION: f32 = 0.25;
const ENEMY_CORNER_MAX: f32 = 14.0;
const LENGTH_UNIT: f32 = 32.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Owner {
    Wall,
    Entity(Entity),
}

pub(crate) struct Physics {
    world: PhysicsWorld,
    solids: Vec<(ColliderHandle, Vec2, Vec2)>,
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
        Self {
            world,
            solids: Vec::new(),
        }
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
        let handle = self.world.colliders.insert(collider);
        self.solids.push((handle, center, size));
        handle
    }

    pub(crate) fn remove_collider(&mut self, collider: ColliderHandle) {
        self.solids.retain(|(handle, _, _)| *handle != collider);
        self.world.remove_collider(collider);
    }

    pub(crate) fn solids_within(&self, min: Vec2, max: Vec2) -> Vec<(Vec2, Vec2)> {
        self.solids
            .iter()
            .filter(|(_, center, size)| {
                let half = *size / 2.0;
                center.x + half.x >= min.x
                    && center.x - half.x <= max.x
                    && center.y + half.y >= min.y
                    && center.y - half.y <= max.y
            })
            .map(|(_, center, size)| (*center, *size))
            .collect()
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

    pub(crate) fn add_pet(&mut self, entity: Entity, position: Vec2, size: Vec2) -> Body {
        let body = RigidBodyBuilder::dynamic()
            .translation(to_rapier(position))
            .lock_rotations();
        let collider = ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0)
            .collision_groups(groups::PETS)
            .user_data(owner_bits(entity));
        self.insert(body, collider)
    }

    pub(crate) fn add_enemy(&mut self, entity: Entity, position: Vec2, size: Vec2) -> Body {
        let body = RigidBodyBuilder::dynamic()
            .translation(to_rapier(position))
            .lock_rotations();
        let radius = (size.min_element() * ENEMY_CORNER_FRACTION).min(ENEMY_CORNER_MAX);
        let half = size / 2.0 - Vec2::splat(radius);
        let collider = ColliderBuilder::round_cuboid(half.x, half.y, radius)
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
        let handle = self.world.colliders.insert(collider);
        self.solids.push((handle, center, size));
        handle
    }

    pub(crate) fn remove(&mut self, body: Body) {
        self.world.remove_body(body.body);
    }

    /// Takes the entity's body and object collider out of the simulation, the entity stays.
    pub(crate) fn detach(&mut self, world: &mut World, entity: Entity) {
        if let Ok(body) = world.remove_one::<Body>(entity) {
            self.remove(body);
        }
        if let Ok(solid) = world.remove_one::<Solid>(entity) {
            self.remove_collider(solid.0);
        }
    }

    /// Drops what an entity still owns in the simulation after it was despawned, or lost its
    /// `Body` or `Solid`, without `detach`. Returns how many were left behind.
    pub(crate) fn remove_orphans(&mut self, world: &World) -> usize {
        let bodies: Vec<RigidBodyHandle> = self
            .world
            .bodies
            .iter()
            .filter(|(handle, body)| {
                body.colliders().iter().any(|collider| {
                    matches!(self.owner(*collider), Some(Owner::Entity(entity))
                        if !world.get::<&Body>(entity).is_ok_and(|held| held.body == *handle))
                })
            })
            .map(|(handle, _)| handle)
            .collect();
        let colliders: Vec<ColliderHandle> = self
            .world
            .colliders
            .iter()
            .filter(|(_, collider)| collider.parent().is_none())
            .filter(|(handle, _)| {
                matches!(self.owner(*handle), Some(Owner::Entity(entity))
                    if !world.get::<&Solid>(entity).is_ok_and(|held| held.0 == *handle))
            })
            .map(|(handle, _)| handle)
            .collect();
        for body in &bodies {
            self.world.remove_body(*body);
        }
        for collider in &colliders {
            self.remove_collider(*collider);
        }
        bodies.len() + colliders.len()
    }

    #[cfg(test)]
    pub(crate) fn body_count(&self) -> usize {
        self.world.bodies.len()
    }

    pub(crate) fn set_velocity(&mut self, body: Body, velocity: Vec2) {
        if let Some(rigid_body) = self.world.bodies.get_mut(body.body) {
            rigid_body.set_linvel(to_rapier(velocity), true);
        }
    }

    pub(crate) fn teleport(&mut self, body: Body, position: Vec2) {
        if let Some(rigid_body) = self.world.bodies.get_mut(body.body) {
            rigid_body.set_translation(to_rapier(position), true);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solid_boxes_are_found_near_a_corridor_and_forgotten_when_removed() {
        let mut physics = Physics::new();
        let handle = physics.add_static_box(Vec2::new(300.0, 300.0), Vec2::new(64.0, 64.0));

        let near = physics.solids_within(Vec2::new(200.0, 200.0), Vec2::new(400.0, 400.0));
        assert_eq!(near, vec![(Vec2::new(300.0, 300.0), Vec2::new(64.0, 64.0))]);
        assert!(physics
            .solids_within(Vec2::new(1000.0, 1000.0), Vec2::new(1200.0, 1200.0))
            .is_empty());

        physics.remove_collider(handle);
        assert!(physics
            .solids_within(Vec2::new(200.0, 200.0), Vec2::new(400.0, 400.0))
            .is_empty());
    }
}
