//! Brings the stone floor's prison to life: a pendulum blade (see
//! [`crate::dungeon::DungeonRoom::pendulum`]) swings across a gap from an iron strap clamped round
//! the planks above it and cuts the player when it catches them.

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::audio::AudioCommand;
use crate::components::{
    Color, Dodge, Layer, Lit, Position, Size, Sprite,
};
use crate::dungeon::Dungeon;
use crate::tilemap::TILE_SIZE;

const OVERHEAD_LAYER: i32 = 4;

/// The blade is a 24x16 picture shown four times over, a chain link 8x8 shown two and a half.
const BLADE_SIZE: Vec2 = Vec2::new(96.0, 64.0);
const LINK_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const LINKS: usize = 7;
/// The strap round the planks is a 16x24 picture, the planks' own piece size, shown four times
/// over; the eye at its foot is this far below the middle of the planks.
const STRAP_SIZE: Vec2 = Vec2::new(64.0, 96.0);
const STRAP_EYE: f32 = 34.0;
/// Above the planks the strap clamps, the chain and the blade.
const PENDULUM_LAYER: i32 = OVERHEAD_LAYER + 1;
/// How far below its beam the blade hangs at the bottom of its swing, and how much higher it
/// rides at either end.
const BLADE_DROP: f32 = 1.3 * TILE_SIZE;
const BLADE_RISE: f32 = 0.35;
/// One swing there and back takes about two and a half seconds.
const SWING_RATE: f32 = 2.5;
/// The part of the blade that cuts, round its middle.
const BLADE_HIT: Vec2 = Vec2::new(60.0, 34.0);
const BLADE_DAMAGE: i32 = 1;
/// How wide the blade looks, which stays within the gap at either end of its swing.
const SWING_WIDTH: f32 = BLADE_SIZE.x;
/// After cutting someone the blade can't cut again for a moment, so one pass hurts once.
const BLADE_REST: f32 = 0.8;
const BLADE_SHADOW: Vec2 = Vec2::new(6.0, TILE_SIZE);

#[derive(Debug, Clone)]
pub(crate) struct Blade {
    pivot: Vec2,
    reach: f32,
    phase: f32,
    rest: f32,
    links: Vec<Entity>,
    shadow: Entity,
}

fn tile_center((column, row): (usize, usize)) -> Vec2 {
    (Vec2::new(column as f32, row as f32) + 0.5) * TILE_SIZE
}

pub(crate) fn spawn_prison(world: &mut World, dungeon: &Dungeon, textures: &Textures) {
    for room in &dungeon.rooms {
        if let Some(span) = room.pendulum {
            spawn_pendulum(world, span.first, span.last, textures);
        }
    }
}

fn spawn_pendulum(world: &mut World, first: (usize, usize), last: (usize, usize), textures: &Textures) {
    let texture = textures.get(tex::PENDULUM);
    // The blade sits top left of the sheet, a chain link below it and the strap on the right.
    let blade_uv = [0.0, 0.0, 0.6, 2.0 / 3.0];
    let link_uv = [0.0, 2.0 / 3.0, 0.2, 1.0];
    let strap_uv = [0.6, 0.0, 1.0, 1.0];
    let middle = (tile_center(first) + tile_center(last)) / 2.0;
    // The strap is clamped round the planks right over the middle of the gap, drawn over them
    // piece for piece; the chain hangs from the eye at its foot.
    world.spawn((
        Position(middle),
        Size(STRAP_SIZE),
        Color(Vec4::ONE),
        Sprite { texture, uv: strap_uv },
        Layer(PENDULUM_LAYER),
        Lit,
    ));
    let pivot = middle + Vec2::new(0.0, STRAP_EYE);
    // It swings from side to side of the gap, never into the blocks either side.
    let reach = ((last.0 + 1 - first.0) as f32 * TILE_SIZE - SWING_WIDTH) / 2.0;
    let links = (0..LINKS)
        .map(|_| {
            world.spawn((
                Position(pivot),
                Size(LINK_SIZE),
                Color(Vec4::ONE),
                Sprite { texture, uv: link_uv },
                Layer(PENDULUM_LAYER),
                Lit,
            ))
        })
        .collect();
    let blade_sprite = Sprite { texture, uv: blade_uv };
    let shadow = world.spawn((
        Position(pivot),
        Size(BLADE_SIZE),
        Color(Vec4::new(0.0, 0.0, 0.0, 0.3)),
        blade_sprite,
        Layer(0),
    ));
    world.spawn((
        Blade {
            pivot,
            reach,
            phase: (first.0 * 3 + first.1) as f32 * 0.7,
            rest: 0.0,
            links,
            shadow,
        },
        Position(pivot + Vec2::new(0.0, BLADE_DROP)),
        Size(BLADE_SIZE),
        Color(Vec4::ONE),
        blade_sprite,
        Layer(PENDULUM_LAYER),
        Lit,
    ));
}

pub(crate) fn tick_pendulums(
    world: &mut World,
    sounds: &mut Vec<AudioCommand>,
    fx: &mut super::Fx,
    clock: f32,
    dt: f32,
) {
    let player = crate::systems::player::player_placement(world);
    let dodging = player.is_some_and(|(entity, _, _)| world.get::<&Dodge>(entity).is_ok());
    let mut moves: Vec<(Entity, Vec2)> = Vec::new();
    let mut cut = false;
    for (blade, position) in world.query_mut::<(&mut Blade, &mut Position)>() {
        let swing = (clock * SWING_RATE + blade.phase).sin();
        let at = blade.pivot
            + Vec2::new(blade.reach * swing, BLADE_DROP * (1.0 - BLADE_RISE * swing * swing));
        position.0 = at;
        for (index, &link) in blade.links.iter().enumerate() {
            let along = (index as f32 + 0.5) / LINKS as f32;
            moves.push((link, blade.pivot.lerp(at, along)));
        }
        moves.push((blade.shadow, at + BLADE_SHADOW));
        blade.rest = (blade.rest - dt).max(0.0);
        if let Some((_, player_at, player_size)) = player {
            let reach = (BLADE_HIT + player_size) / 2.0;
            let touching = (player_at - at).abs().cmplt(reach).all();
            if touching && !dodging && blade.rest <= 0.0 {
                blade.rest = BLADE_REST;
                cut = true;
            }
        }
    }
    for (entity, at) in moves {
        if let Ok(mut position) = world.get::<&mut Position>(entity) {
            position.0 = at;
        }
    }
    if let (true, Some((entity, _, _))) = (cut, player) {
        super::damage::hurt_player(world, entity, BLADE_DAMAGE, sounds, fx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    fn floor_with(find: impl Fn(&Dungeon) -> bool) -> Dungeon {
        (0..60)
            .map(|seed| {
                crate::dungeon::Dungeon::generate_themed(&mut Rng::new(seed), crate::dungeon::Theme::Stone)
                    .expect("a dungeon")
            })
            .find(|dungeon| find(dungeon))
            .expect("a stone floor with it")
    }

    #[test]
    fn the_pendulum_swings_across_its_gap_and_cuts_the_player_once_a_pass() {
        let dungeon = floor_with(|dungeon| dungeon.rooms.iter().any(|room| room.pendulum.is_some()));
        let span = dungeon.rooms.iter().find_map(|room| room.pendulum).expect("a pendulum");
        let mut world = World::new();
        spawn_prison(&mut world, &dungeon, &Textures::default());
        let blade = |world: &World| -> Vec2 {
            world.query::<(&Blade, &Position)>().iter().next().map(|(_, at)| at.0).expect("a blade")
        };
        let mut sounds = Vec::new();
        let (mut least, mut most) = (f32::MAX, f32::MIN);
        for step in 0..180 {
            tick_pendulums(
                &mut world,
                &mut sounds,
                &mut crate::systems::Fx::default(),
                step as f32 / 60.0,
                1.0 / 60.0,
            );
            least = least.min(blade(&world).x);
            most = most.max(blade(&world).x);
        }
        let (left, right) = (tile_center(span.first).x, tile_center(span.last).x);
        assert!(least < left + TILE_SIZE && most > right - TILE_SIZE, "{least} {most}");
        let (wall_left, wall_right) = (span.first.0 as f32 * TILE_SIZE, (span.last.0 + 1) as f32 * TILE_SIZE);
        assert!(least - SWING_WIDTH / 2.0 >= wall_left - 1.0 && most + SWING_WIDTH / 2.0 <= wall_right + 1.0);

        let start = blade(&world);
        let player = crate::prefabs::player(
            &mut world,
            &mut crate::physics::Physics::new(),
            start,
            &Textures::default(),
        );
        if let Ok(mut armor) = world.get::<&mut crate::components::Armor>(player) {
            armor.current = 0;
        }
        let health = |world: &World| world.get::<&crate::components::Health>(player).map_or(0, |h| h.current);
        let before = health(&world);
        let mut clock = 3.0;
        for _ in 0..3 {
            let _ = world.insert_one(player, Position(blade(&world)));
            tick_pendulums(
                &mut world,
                &mut sounds,
                &mut crate::systems::Fx::default(),
                clock,
                1.0 / 60.0,
            );
            clock += 1.0 / 60.0;
        }
        assert_eq!(health(&world), before - BLADE_DAMAGE);
    }
}
