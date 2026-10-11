use hecs::{Entity, World};
use winit::keyboard::KeyCode;

use crate::components::{InRange, Interactable, Interaction, Player, Position, Size};
use crate::geometry;
use crate::input::Input;

pub(crate) fn interact(world: &mut World, input: &Input) -> Option<Interaction> {
    let player = world
        .query::<(&Position, &Size)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(position, size)| (position.0, size.0));

    let all: Vec<(Entity, bool, Interaction)> = world
        .query::<(Entity, &Position, &Size, &Interactable)>()
        .iter()
        .map(|(entity, position, size, interactable)| {
            let near = player.is_some_and(|(player_position, player_size)| {
                geometry::overlaps(player_position, player_size, position.0, size.0)
            });
            (entity, near, interactable.action)
        })
        .collect();

    for &(entity, near, _) in &all {
        if near {
            let _ = world.insert_one(entity, InRange);
        } else {
            let _ = world.remove_one::<InRange>(entity);
        }
    }

    if input.just_pressed(KeyCode::KeyE) {
        all.iter()
            .find(|(_, near, _)| *near)
            .map(|(_, _, action)| *action)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

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
}
