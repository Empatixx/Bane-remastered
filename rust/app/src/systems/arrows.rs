use glam::Vec2;
use hecs::{Entity, World};

use crate::components::{
    Color, Enemy, EnemyArrow, Expiring, Health, Layer, Position, RoomId, ScreenSpace, Size,
    Spawning, Sprite,
};
use crate::render::TextureId;
use crate::viewport::TARGET;

const ARROW_SIZE: Vec2 = Vec2::new(18.0, 18.0);
const EDGE_INSET: f32 = 20.0;
const SEEN_MARGIN: f32 = 20.0;
const ARROW_ALPHA: f32 = 0.3;
const MERGE_DISTANCE: f32 = 44.0;

pub(crate) fn arrow_at(screen: Vec2) -> Option<(Vec2, f32)> {
    let center = TARGET / 2.0;
    let seen = screen.x > -SEEN_MARGIN
        && screen.x < TARGET.x + SEEN_MARGIN
        && screen.y > -SEEN_MARGIN
        && screen.y < TARGET.y + SEEN_MARGIN;
    if seen {
        return None;
    }
    let direction = screen - center;
    let reach = center - Vec2::splat(EDGE_INSET);
    let scale = (reach.x / direction.x.abs().max(f32::EPSILON))
        .min(reach.y / direction.y.abs().max(f32::EPSILON));
    Some((center + direction * scale, direction.y.atan2(direction.x)))
}

fn merged(arrows: Vec<(Vec2, f32)>) -> Vec<(Vec2, f32)> {
    let mut kept: Vec<(Vec2, f32)> = Vec::new();
    for arrow in arrows {
        if kept
            .iter()
            .all(|(position, _)| position.distance(arrow.0) > MERGE_DISTANCE)
        {
            kept.push(arrow);
        }
    }
    kept
}

pub(crate) fn update_enemy_arrows(world: &mut World, camera: Vec2, texture: TextureId) {
    let old: Vec<Entity> = world
        .query::<Entity>()
        .with::<&EnemyArrow>()
        .iter()
        .collect();
    for entity in old {
        let _ = world.despawn(entity);
    }
    let arrows: Vec<(Vec2, f32)> = world
        .query::<(&Position, &Health)>()
        .with::<(&Enemy, &RoomId)>()
        .without::<(&Expiring, &Spawning)>()
        .iter()
        .filter(|(_, health)| health.current > 0)
        .filter_map(|(position, _)| arrow_at(position.0 - camera))
        .collect();
    for (position, angle) in merged(arrows) {
        world.spawn((
            EnemyArrow { angle },
            Position(position),
            Size(ARROW_SIZE),
            Color(glam::Vec4::new(1.0, 1.0, 1.0, ARROW_ALPHA)),
            Sprite::whole(texture),
            Layer(4),
            ScreenSpace,
        ));
    }
}

#[cfg(test)]
mod tests {
    use crate::assets::Textures;
    use crate::enemies::EnemyKind;
    use crate::prefabs;

    use super::*;

    fn room_enemy(world: &mut World, at: Vec2) -> Entity {
        prefabs::enemy_in_room(world, EnemyKind::Slime, at, 1, &Textures::default(), 0)
    }

    fn arrows(world: &World) -> Vec<(Vec2, f32)> {
        world
            .query::<(&Position, &EnemyArrow)>()
            .iter()
            .map(|(position, arrow)| (position.0, arrow.angle))
            .collect()
    }

    #[test]
    fn an_enemy_on_the_screen_gets_no_arrow_and_one_far_to_the_right_gets_one_on_the_right_edge() {
        assert_eq!(arrow_at(Vec2::new(300.0, 200.0)), None);
        let (at, angle) = arrow_at(Vec2::new(3000.0, 270.0)).expect("an arrow");

        assert!((at.x - (TARGET.x - EDGE_INSET)).abs() < 0.01);
        assert!((at.y - 270.0).abs() < 0.01);
        assert!(angle.abs() < 0.001);
    }

    #[test]
    fn arrows_point_towards_the_enemy_in_every_direction() {
        let center = TARGET / 2.0;
        for (offset, expected) in [
            (Vec2::new(0.0, 2000.0), std::f32::consts::FRAC_PI_2),
            (Vec2::new(0.0, -2000.0), -std::f32::consts::FRAC_PI_2),
            (Vec2::new(-2000.0, 0.0), std::f32::consts::PI),
        ] {
            let (at, angle) = arrow_at(center + offset).expect("an arrow");

            assert!((angle.abs() - expected.abs()).abs() < 0.001, "{offset:?}");
            assert!(at.x >= EDGE_INSET - 0.01 && at.x <= TARGET.x - EDGE_INSET + 0.01);
            assert!(at.y >= EDGE_INSET - 0.01 && at.y <= TARGET.y - EDGE_INSET + 0.01);
        }
    }

    #[test]
    fn a_diagonal_enemy_gets_an_arrow_in_the_matching_corner_area() {
        let (at, angle) = arrow_at(TARGET / 2.0 + Vec2::new(2000.0, 2000.0)).expect("an arrow");

        assert!(at.x > TARGET.x / 2.0 && at.y > TARGET.y / 2.0);
        assert!((angle - std::f32::consts::FRAC_PI_4).abs() < 0.001);
    }

    #[test]
    fn only_living_enemies_of_a_room_outside_the_screen_get_arrows_and_they_do_not_pile_up() {
        let mut world = World::new();
        room_enemy(&mut world, Vec2::new(3000.0, 300.0));
        room_enemy(&mut world, Vec2::new(-2500.0, 300.0));
        room_enemy(&mut world, Vec2::new(400.0, 300.0));
        let texture = Textures::default().get(crate::assets::tex::ENEMY_ARROW);

        update_enemy_arrows(&mut world, Vec2::ZERO, texture);
        assert_eq!(arrows(&world).len(), 2);
        update_enemy_arrows(&mut world, Vec2::ZERO, texture);
        assert_eq!(arrows(&world).len(), 2);

        update_enemy_arrows(&mut world, Vec2::new(2800.0, 0.0), texture);
        let after = arrows(&world);
        assert_eq!(after.len(), 2);
    }

    #[test]
    fn dead_enemies_and_enemies_outside_a_room_get_no_arrow() {
        let mut world = World::new();
        let dead = room_enemy(&mut world, Vec2::new(3000.0, 300.0));
        if let Ok(mut health) = world.get::<&mut Health>(dead) {
            health.current = 0;
        }
        let texture = Textures::default().get(crate::assets::tex::ENEMY_ARROW);

        update_enemy_arrows(&mut world, Vec2::ZERO, texture);

        assert!(arrows(&world).is_empty());
    }

    #[test]
    fn enemies_in_nearly_the_same_direction_share_one_arrow() {
        let mut world = World::new();
        room_enemy(&mut world, Vec2::new(3000.0, 300.0));
        room_enemy(&mut world, Vec2::new(3100.0, 320.0));
        room_enemy(&mut world, Vec2::new(3300.0, 280.0));
        let texture = Textures::default().get(crate::assets::tex::ENEMY_ARROW);

        update_enemy_arrows(&mut world, Vec2::ZERO, texture);

        assert_eq!(arrows(&world).len(), 1);
    }

    #[test]
    fn enemies_in_different_directions_keep_their_own_arrows() {
        let mut world = World::new();
        room_enemy(&mut world, Vec2::new(3000.0, 300.0));
        room_enemy(&mut world, Vec2::new(-2500.0, 300.0));
        room_enemy(&mut world, Vec2::new(500.0, 3000.0));
        let texture = Textures::default().get(crate::assets::tex::ENEMY_ARROW);

        update_enemy_arrows(&mut world, Vec2::ZERO, texture);

        let found = arrows(&world);
        assert_eq!(found.len(), 3);
        for (index, a) in found.iter().enumerate() {
            for b in &found[index + 1..] {
                assert!(a.0.distance(b.0) > MERGE_DISTANCE);
            }
        }
    }
}
