use glam::Vec2;
use hecs::{Entity, World};

use crate::artefacts::{ArtefactKind, BERSERK_SECONDS, RING_BULLETS, RING_DAMAGE, RING_TURN};
use crate::assets::Textures;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Armor, ArtefactDrop, ArtefactIcon, ArtefactSlot, Berserk, Hidden, InRange, Player, Position,
    ShieldAura, Shielded, Size, Sprite,
};
use crate::controls::Action;
use crate::input::Input;
use crate::physics::Physics;
use crate::prefabs;
use crate::render::TextureId;

const BULLET_SPEED: f32 = 700.0;

pub(crate) fn activate_artefact(
    world: &mut World,
    physics: &mut Physics,
    input: &Input,
    bullet_texture: TextureId,
) -> Option<ArtefactKind> {
    if !input.action_pressed(Action::UseArtefact) {
        return None;
    }
    let (player, origin, kind) = world
        .query_mut::<(Entity, &Position, &mut ArtefactSlot)>()
        .with::<&Player>()
        .into_iter()
        .next()
        .and_then(|(entity, position, slot)| {
            slot.current.take().map(|kind| (entity, position.0, kind))
        })?;
    match kind {
        ArtefactKind::RingOfDeath => {
            let base = (-origin.y).atan2(-origin.x);
            for index in 0..RING_BULLETS {
                let angle = base + RING_TURN * index as f32;
                prefabs::bullet(
                    world,
                    physics,
                    origin,
                    Vec2::from_angle(angle) * BULLET_SPEED,
                    RING_DAMAGE,
                    bullet_texture,
                );
            }
        }
        ArtefactKind::ArmorRefill => {
            if let Ok(mut armor) = world.get::<&mut Armor>(player) {
                armor.current = armor.max;
            }
        }
        ArtefactKind::ShieldHorn => {
            let _ = world.insert_one(player, Shielded);
        }
        ArtefactKind::BerserkPot => {
            let _ = world.insert_one(
                player,
                Berserk {
                    remaining: BERSERK_SECONDS,
                },
            );
        }
        ArtefactKind::RevealMap => {}
    }
    Some(kind)
}

pub(crate) fn tick_berserk(world: &mut World, dt: f32) {
    let mut over = Vec::new();
    for (entity, berserk) in world.query_mut::<(Entity, &mut Berserk)>() {
        berserk.remaining -= dt;
        if berserk.remaining <= 0.0 {
            over.push(entity);
        }
    }
    for entity in over {
        let _ = world.remove_one::<Berserk>(entity);
    }
}

pub(crate) fn update_shield_aura(world: &mut World, textures: &Textures) {
    let center = world
        .query::<&Position>()
        .with::<(&Player, &Shielded)>()
        .iter()
        .next()
        .map(|position| position.0);
    let auras: Vec<Entity> = world
        .query::<Entity>()
        .with::<&ShieldAura>()
        .iter()
        .collect();
    match center {
        Some(center) => {
            if auras.is_empty() {
                prefabs::shield_aura(world, center, textures);
            }
            for position in world.query_mut::<&mut Position>().with::<&ShieldAura>() {
                position.0 = center;
            }
        }
        None => {
            for aura in auras {
                let _ = world.despawn(aura);
            }
        }
    }
}

pub(crate) fn unused_artefact(slot: &ArtefactSlot, roll: usize) -> ArtefactKind {
    let free: Vec<ArtefactKind> = ArtefactKind::ALL
        .into_iter()
        .filter(|kind| Some(*kind) != slot.current)
        .collect();
    free[roll % free.len()]
}

pub(crate) fn pick_up_artefact(
    world: &mut World,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
) {
    let drop = world
        .query::<(Entity, &ArtefactDrop)>()
        .with::<&InRange>()
        .iter()
        .next()
        .map(|(entity, drop)| (entity, drop.kind));
    let Some((entity, kind)) = drop else {
        return;
    };
    let _ = world.despawn(entity);
    super::stats::record(world, super::stats::Stat::Item, 1);
    let replaced = world
        .query_mut::<(&Position, &mut ArtefactSlot)>()
        .with::<&Player>()
        .into_iter()
        .next()
        .map(|(position, slot)| (position.0, slot.current.replace(kind)));
    sounds.push(AudioCommand::Effect(SoundEffect::SwitchGun));
    if let Some((origin, Some(previous))) = replaced {
        prefabs::artefact_drop(world, previous, origin, Vec2::ZERO, textures);
    }
}

pub(crate) fn update_artefact_hud(world: &mut World, textures: &Textures) {
    let current = world
        .query::<&ArtefactSlot>()
        .with::<&Player>()
        .iter()
        .next()
        .and_then(|slot| slot.current);
    let icons: Vec<Entity> = world
        .query::<Entity>()
        .with::<&ArtefactIcon>()
        .iter()
        .collect();
    for icon in icons {
        match current {
            Some(kind) => {
                let _ = world.remove_one::<Hidden>(icon);
                if let Ok((sprite, size)) = world.query_one_mut::<(&mut Sprite, &mut Size)>(icon) {
                    sprite.texture = textures.get(kind.texture());
                    size.0 = kind.icon();
                }
            }
            None => {
                let _ = world.insert_one(icon, Hidden);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::Health;
    use crate::physics::Physics;
    use winit::keyboard::KeyCode;

    use super::*;

    fn setup() -> (World, Physics, Textures) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(&mut world, &mut physics, Vec2::new(300.0, 200.0), &textures);
        (world, physics, textures)
    }

    fn press_f() -> Input {
        let mut input = Input::default();
        input.press(KeyCode::KeyF);
        input
    }

    fn give(world: &mut World, kind: Option<ArtefactKind>) {
        for slot in world.query_mut::<&mut ArtefactSlot>().with::<&Player>() {
            slot.current = kind;
        }
    }

    fn held(world: &World) -> Option<ArtefactKind> {
        world
            .query::<&ArtefactSlot>()
            .with::<&Player>()
            .iter()
            .next()
            .and_then(|slot| slot.current)
    }

    #[test]
    fn the_player_starts_with_the_reveal_map_artefact() {
        let (world, _, _) = setup();
        assert_eq!(held(&world), Some(ArtefactKind::RevealMap));
    }

    #[test]
    fn f_uses_the_artefact_up_and_only_when_pressed() {
        let (mut world, mut physics, _) = setup();
        let idle = Input::default();
        assert_eq!(
            activate_artefact(&mut world, &mut physics, &idle, TextureId::WHITE),
            None
        );
        assert!(held(&world).is_some());

        let used = activate_artefact(&mut world, &mut physics, &press_f(), TextureId::WHITE);
        assert_eq!(used, Some(ArtefactKind::RevealMap));
        assert_eq!(held(&world), None);
        assert_eq!(
            activate_artefact(&mut world, &mut physics, &press_f(), TextureId::WHITE),
            None
        );
    }

    #[test]
    fn the_ring_of_death_fires_fifty_bullets_in_a_widening_spiral() {
        let (mut world, mut physics, _) = setup();
        give(&mut world, Some(ArtefactKind::RingOfDeath));
        activate_artefact(&mut world, &mut physics, &press_f(), TextureId::WHITE);

        let bullets: Vec<(Vec2, i32)> = world
            .query::<(&crate::components::Velocity, &crate::components::Bullet)>()
            .iter()
            .map(|(velocity, bullet)| (velocity.0, bullet.damage))
            .collect();
        assert_eq!(bullets.len(), 50);
        assert!(
            bullets
                .iter()
                .all(|(velocity, damage)| *damage == 2 && (velocity.length() - 700.0).abs() < 0.1)
        );
        let first = bullets[0].0.to_angle();
        let second = bullets[1].0.to_angle();
        assert!(((second - first).rem_euclid(std::f32::consts::TAU) - 0.15).abs() < 1e-3);
    }

    #[test]
    fn the_armor_refill_tops_up_the_armor() {
        let (mut world, mut physics, _) = setup();
        for armor in world.query_mut::<&mut Armor>().with::<&Player>() {
            armor.current = 0;
        }
        give(&mut world, Some(ArtefactKind::ArmorRefill));
        activate_artefact(&mut world, &mut physics, &press_f(), TextureId::WHITE);
        let armor = world
            .query::<&Armor>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|a| (a.current, a.max));
        assert_eq!(armor.map(|(c, m)| c == m), Some(true));
    }

    #[test]
    fn the_shield_horn_blocks_exactly_one_hit_and_shows_an_aura_meanwhile() {
        let (mut world, mut physics, textures) = setup();
        give(&mut world, Some(ArtefactKind::ShieldHorn));
        activate_artefact(&mut world, &mut physics, &press_f(), TextureId::WHITE);
        update_shield_aura(&mut world, &textures);
        assert_eq!(world.query::<&ShieldAura>().iter().count(), 1);
        update_shield_aura(&mut world, &textures);
        assert_eq!(world.query::<&ShieldAura>().iter().count(), 1);

        let player = world
            .query::<Entity>()
            .with::<&Player>()
            .iter()
            .next()
            .expect("a player");
        let before = world
            .query::<(&Health, &Armor)>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|(h, a)| (h.current, a.current));
        let mut sounds = Vec::new();
        crate::systems::damage::hurt_player(&mut world, player, 3, &mut sounds);
        let after = world
            .query::<(&Health, &Armor)>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|(h, a)| (h.current, a.current));
        assert_eq!(before, after);
        assert!(world.get::<&Shielded>(player).is_err());

        update_shield_aura(&mut world, &textures);
        assert_eq!(world.query::<&ShieldAura>().iter().count(), 0);
    }

    #[test]
    fn berserk_lasts_fifteen_seconds() {
        let (mut world, mut physics, _) = setup();
        give(&mut world, Some(ArtefactKind::BerserkPot));
        activate_artefact(&mut world, &mut physics, &press_f(), TextureId::WHITE);
        tick_berserk(&mut world, 14.9);
        assert_eq!(world.query::<&Berserk>().iter().count(), 1);
        tick_berserk(&mut world, 0.2);
        assert_eq!(world.query::<&Berserk>().iter().count(), 0);
    }

    #[test]
    fn picking_up_a_new_artefact_drops_the_old_one() {
        let (mut world, _, textures) = setup();
        let drop = prefabs::artefact_drop(
            &mut world,
            ArtefactKind::RingOfDeath,
            Vec2::new(300.0, 200.0),
            Vec2::ZERO,
            &textures,
        );
        let _ = world.insert_one(drop, InRange);

        pick_up_artefact(&mut world, &textures, &mut Vec::new());

        assert_eq!(held(&world), Some(ArtefactKind::RingOfDeath));
        let lying: Vec<ArtefactKind> = world
            .query::<&ArtefactDrop>()
            .iter()
            .map(|d| d.kind)
            .collect();
        assert_eq!(lying, vec![ArtefactKind::RevealMap]);
    }

    #[test]
    fn a_random_artefact_is_never_the_one_you_hold() {
        let slot = ArtefactSlot {
            current: Some(ArtefactKind::ShieldHorn),
        };
        for roll in 0..20 {
            assert_ne!(unused_artefact(&slot, roll), ArtefactKind::ShieldHorn);
        }
    }

    #[test]
    fn the_hud_icon_shows_the_current_artefact_and_hides_when_empty() {
        let (mut world, _, textures) = setup();
        prefabs::artefact_hud(&mut world, &textures);
        update_artefact_hud(&mut world, &textures);
        assert_eq!(
            world
                .query::<&Hidden>()
                .with::<&ArtefactIcon>()
                .iter()
                .count(),
            0
        );
        give(&mut world, None);
        update_artefact_hud(&mut world, &textures);
        assert_eq!(
            world
                .query::<&Hidden>()
                .with::<&ArtefactIcon>()
                .iter()
                .count(),
            1
        );
    }
}
