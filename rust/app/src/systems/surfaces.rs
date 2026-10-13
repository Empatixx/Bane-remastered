use hecs::{Entity, World};

use crate::audio::AudioCommand;
use crate::components::{Dodge, Health, Player, Position};
use crate::dungeon::{Surface, Surfaces};
use crate::systems::damage::hurt_player;

const LAVA_DAMAGE: i32 = 1;

pub(crate) fn burn_on_lava(world: &mut World, surfaces: &Surfaces, sounds: &mut Vec<AudioCommand>) {
    let Some((player, at)) = world
        .query::<(Entity, &Position, &Health)>()
        .with::<&Player>()
        .without::<&Dodge>()
        .iter()
        .filter(|(_, _, health)| health.current > 0)
        .map(|(entity, position, _)| (entity, position.0))
        .next()
    else {
        return;
    };
    if surfaces.at_point(at) == Some(Surface::Lava) {
        hurt_player(world, player, LAVA_DAMAGE, sounds);
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::assets::Textures;
    use crate::components::{Armor, Invulnerable};
    use crate::physics::Physics;
    use crate::prefabs;
    use crate::tilemap::TILE_SIZE;

    use super::*;

    fn standing_at(tile: (usize, usize)) -> (World, Entity) {
        let mut world = World::new();
        let at = Vec2::new(tile.0 as f32 + 0.5, tile.1 as f32 + 0.5) * TILE_SIZE;
        let player = prefabs::player(&mut world, &mut Physics::new(), at, &Textures::default());
        if let Ok(mut armor) = world.get::<&mut Armor>(player) {
            armor.current = 0;
        }
        (world, player)
    }

    fn health(world: &World, player: Entity) -> i32 {
        world
            .get::<&Health>(player)
            .map_or(0, |health| health.current)
    }

    #[test]
    fn standing_on_lava_hurts_and_standing_beside_it_does_not() {
        let mut surfaces = Surfaces::default();
        surfaces.insert((4, 4), Surface::Lava);

        let (mut world, player) = standing_at((5, 4));
        burn_on_lava(&mut world, &surfaces, &mut Vec::new());
        assert_eq!(health(&world, player), 10);

        let (mut world, player) = standing_at((4, 4));
        burn_on_lava(&mut world, &surfaces, &mut Vec::new());
        assert_eq!(health(&world, player), 9);
        assert!(world.get::<&Invulnerable>(player).is_ok());
    }

    #[test]
    fn other_surfaces_do_not_burn() {
        let mut surfaces = Surfaces::default();
        surfaces.insert((4, 4), Surface::Water);
        let (mut world, player) = standing_at((4, 4));
        burn_on_lava(&mut world, &surfaces, &mut Vec::new());
        assert_eq!(health(&world, player), 10);
    }

    #[test]
    fn a_dodging_player_flies_over_the_lava() {
        let mut surfaces = Surfaces::default();
        surfaces.insert((4, 4), Surface::Lava);
        let (mut world, player) = standing_at((4, 4));
        let _ = world.insert_one(
            player,
            Dodge {
                direction: Vec2::X,
                remaining: 0.3,
                trail: 0.0,
            },
        );
        burn_on_lava(&mut world, &surfaces, &mut Vec::new());
        assert_eq!(health(&world, player), 10);
    }
}
