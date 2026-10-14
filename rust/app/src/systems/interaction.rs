use hecs::{Entity, World};

use crate::components::{InRange, Interactable, Interaction, Player, Position, Size};
use crate::controls::Action;
use crate::geometry;
use crate::input::Input;

/// Marks the one interactable the player should use: of everything the player overlaps, the
/// one whose center is nearest. Only that one shows its prompt and reacts to the key.
pub(crate) fn interact(world: &mut World, input: &Input) -> Option<Interaction> {
    let player = world
        .query::<(&Position, &Size)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(position, size)| (position.0, size.0));

    let mut all: Vec<Entity> = Vec::new();
    let mut nearest: Option<(Entity, f32, Interaction)> = None;
    for (entity, position, size, interactable) in world
        .query::<(Entity, &Position, &Size, &Interactable)>()
        .iter()
    {
        all.push(entity);
        let Some((player_position, player_size)) = player else {
            continue;
        };
        if !geometry::overlaps(player_position, player_size, position.0, size.0) {
            continue;
        }
        let distance = player_position.distance_squared(position.0);
        if nearest.is_none_or(|(_, best, _)| distance < best) {
            nearest = Some((entity, distance, interactable.action));
        }
    }

    for entity in all {
        if nearest.is_some_and(|(chosen, _, _)| chosen == entity) {
            let _ = world.insert_one(entity, InRange);
        } else {
            let _ = world.remove_one::<InRange>(entity);
        }
    }

    if input.action_pressed(Action::Interact) {
        nearest.map(|(_, _, action)| action)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;
    use winit::keyboard::KeyCode;

    use crate::assets::Textures;
    use crate::physics::Physics;
    use crate::prefabs;
    use crate::props::Prop;

    use super::*;

    fn lobby_with_portal_at(portal: Vec2) -> (World, hecs::Entity) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(&mut world, &mut physics, Vec2::new(300.0, 300.0), &textures);
        let size = Prop::Portal.spec().size();
        let entity = prefabs::portal(&mut world, portal - size / 2.0, &textures);
        (world, entity)
    }

    #[test]
    fn standing_in_the_portal_shows_the_prompt_and_e_enters() {
        let (mut world, portal) = lobby_with_portal_at(Vec2::new(300.0, 300.0));
        let mut input = Input::default();

        assert_eq!(interact(&mut world, &input), None);
        assert!(world.get::<&InRange>(portal).is_ok());

        input.press(KeyCode::KeyE);
        assert_eq!(interact(&mut world, &input), Some(Interaction::EnterPortal));
    }

    #[test]
    fn far_from_the_portal_nothing_happens_even_when_e_is_pressed() {
        let (mut world, portal) = lobby_with_portal_at(Vec2::new(1500.0, 300.0));
        let mut input = Input::default();
        input.press(KeyCode::KeyE);

        assert_eq!(interact(&mut world, &input), None);
        assert!(world.get::<&InRange>(portal).is_err());
    }

    #[test]
    fn of_two_overlapping_interactables_only_the_nearest_is_in_range_and_used() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(&mut world, &mut physics, Vec2::new(300.0, 300.0), &textures);
        let player_at = world
            .query::<&Position>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|position| position.0)
            .expect("player");
        let big = Vec2::splat(200.0);
        let far = world.spawn((
            Position(player_at + Vec2::new(60.0, 0.0)),
            Size(big),
            Interactable {
                action: Interaction::Buy,
            },
        ));
        let near = world.spawn((
            Position(player_at + Vec2::new(-10.0, 0.0)),
            Size(big),
            Interactable {
                action: Interaction::PickUpGun,
            },
        ));
        let mut input = Input::default();
        input.press(KeyCode::KeyE);

        assert_eq!(interact(&mut world, &input), Some(Interaction::PickUpGun));
        assert!(world.get::<&InRange>(near).is_ok());
        assert!(world.get::<&InRange>(far).is_err());

        let _ = world.insert_one(near, Position(player_at + Vec2::new(90.0, 0.0)));
        assert_eq!(interact(&mut world, &input), Some(Interaction::Buy));
        assert!(world.get::<&InRange>(far).is_ok());
        assert!(world.get::<&InRange>(near).is_err());
    }

    #[test]
    fn the_rebound_interact_key_is_the_one_that_works() {
        let (mut world, _portal) = lobby_with_portal_at(Vec2::new(300.0, 300.0));
        let mut input = Input::default();
        let mut bindings = crate::controls::Bindings::default();
        bindings.rebind(Action::Interact, KeyCode::KeyX);
        input.set_bindings(bindings);

        input.press(KeyCode::KeyE);
        assert_eq!(interact(&mut world, &input), None);
        input.press(KeyCode::KeyX);
        assert_eq!(interact(&mut world, &input), Some(Interaction::EnterPortal));
    }
}
