//! The one place that finds the player. The hero is a singleton, so every system that wants him
//! asks here instead of keeping its own query. The variants differ only in whom they accept:
//!
//! | function | accepts |
//! |---|---|
//! | [`player`], [`player_at`], [`player_entity_at`], [`player_placement`] | the player, however he is |
//! | [`player_body`] | the player while he is alive (cloaked or not) |
//! | [`uncloaked_player`] | the player while he is not a `Phantom` (dead or alive) |
//! | [`living_player_at`] | the player while he is alive and not a `Phantom`: what enemies can target |
//!
//! Positions are pixel centres, sizes are hitbox sizes.

use glam::Vec2;
use hecs::{Entity, World};

use crate::components::{Health, Phantom, Player, Position, Size};

/// Where the player stands, whatever his state.
pub(crate) fn player_at(world: &World) -> Option<Vec2> {
    player_entity_at(world).map(|(_, at)| at)
}

/// The player and where he stands, whatever his state.
pub(crate) fn player_entity_at(world: &World) -> Option<(Entity, Vec2)> {
    world
        .query::<(Entity, &Position)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(entity, position)| (entity, position.0))
}

/// The player, his position and his hitbox size, whatever his state (a dead or cloaked player
/// still counts: this is for hazards that hit whatever stands there).
pub(crate) fn player_placement(world: &World) -> Option<(Entity, Vec2, Vec2)> {
    world
        .query::<(Entity, &Position, &Size)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(entity, position, size)| (entity, position.0, size.0))
}

/// The living player, his position and hitbox size. A cloaked player is still found, so
/// area attacks (lava, magma, frost) keep hitting him.
pub(crate) fn player_body(world: &World) -> Option<(Entity, Vec2, Vec2)> {
    world
        .query::<(Entity, &Position, &Size, &Health)>()
        .with::<&Player>()
        .iter()
        .filter(|(_, _, _, health)| health.current > 0)
        .map(|(entity, position, size, _)| (entity, position.0, size.0))
        .next()
}

/// The player, his position and hitbox size, unless he is cloaked. Dead or alive: enemies that
/// only need a spot to walk towards use this.
pub(crate) fn uncloaked_player(world: &World) -> Option<(Entity, Vec2, Vec2)> {
    world
        .query::<(Entity, &Position, &Size)>()
        .with::<&Player>()
        .without::<&Phantom>()
        .iter()
        .next()
        .map(|(entity, position, size)| (entity, position.0, size.0))
}

/// The living, uncloaked player: the only one an enemy may pick as a target.
pub(crate) fn living_player_at(world: &World) -> Option<(Entity, Vec2, Vec2)> {
    player_body(world).filter(|(entity, _, _)| world.get::<&Phantom>(*entity).is_err())
}

#[cfg(test)]
mod tests {
    use crate::assets::Textures;
    use crate::physics::Physics;
    use crate::prefabs;

    use super::*;

    const AT: Vec2 = Vec2::new(120.0, 80.0);

    fn arena() -> (World, Entity) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let hero = prefabs::player(&mut world, &mut physics, AT, &Textures::default());
        (world, hero)
    }

    fn cloak(world: &mut World, hero: Entity) {
        let _ = world.insert_one(hero, Phantom { remaining: 5.0 });
    }

    fn kill(world: &mut World, hero: Entity) {
        world.get::<&mut Health>(hero).expect("health").current = 0;
    }

    #[test]
    fn nobody_is_found_in_an_empty_world() {
        let world = World::new();
        assert!(player_at(&world).is_none());
        assert!(player_entity_at(&world).is_none());
        assert!(player_placement(&world).is_none());
        assert!(player_body(&world).is_none());
        assert!(uncloaked_player(&world).is_none());
        assert!(living_player_at(&world).is_none());
    }

    #[test]
    fn the_plain_lookups_find_the_player_whatever_his_state() {
        let (mut world, hero) = arena();
        cloak(&mut world, hero);
        kill(&mut world, hero);

        assert_eq!(player_at(&world), Some(AT));
        assert_eq!(player_entity_at(&world), Some((hero, AT)));
        let (entity, at, size) = player_placement(&world).expect("placement");
        assert_eq!((entity, at), (hero, AT));
        assert!(size.x > 0.0 && size.y > 0.0);
    }

    #[test]
    fn the_body_needs_a_living_player_but_not_an_uncloaked_one() {
        let (mut world, hero) = arena();
        let (entity, at, size) = player_body(&world).expect("alive");
        assert_eq!((entity, at), (hero, AT));
        assert_eq!(
            Some(size),
            player_placement(&world).map(|(_, _, size)| size)
        );

        cloak(&mut world, hero);
        assert!(player_body(&world).is_some());

        kill(&mut world, hero);
        assert!(player_body(&world).is_none());
    }

    #[test]
    fn the_uncloaked_player_is_found_unless_he_is_cloaked() {
        let (mut world, hero) = arena();
        assert_eq!(
            uncloaked_player(&world).map(|(e, at, _)| (e, at)),
            Some((hero, AT))
        );

        kill(&mut world, hero);
        assert!(uncloaked_player(&world).is_some());

        cloak(&mut world, hero);
        assert!(uncloaked_player(&world).is_none());
    }

    #[test]
    fn a_target_must_be_both_alive_and_uncloaked() {
        let (mut world, hero) = arena();
        assert_eq!(
            living_player_at(&world).map(|(e, at, _)| (e, at)),
            Some((hero, AT))
        );

        cloak(&mut world, hero);
        assert!(living_player_at(&world).is_none());

        let (mut world, hero) = arena();
        kill(&mut world, hero);
        assert!(living_player_at(&world).is_none());
    }
}
