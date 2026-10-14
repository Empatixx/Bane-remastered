use glam::Vec2;
use hecs::{Entity, World};

use crate::artefacts::{
    ArtefactKind, BERSERK_SECONDS, BLOODLUST_SECONDS, CHALICE_HEAL, INFINITE_AMMO_SECONDS,
    MIDAS_SECONDS, PHANTOM_SECONDS, RING_BULLETS, RING_DAMAGE, RING_TURN, TIME_SLOW_SECONDS,
};
use crate::assets::Textures;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Armor, ArtefactDrop, ArtefactIcon, ArtefactSlot, Berserk, Bloodlust, CastLightning,
    ConvertBullets, HealPercent, Hidden, InRange, InfiniteAmmo, MidasTouch, Phantom, PlaceTurret,
    Player, Position, ShieldAura, Shielded, Size, Sprite, SummonGhosts, TimeSlow,
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
                    crate::components::Bullet::plain(RING_DAMAGE),
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
        ArtefactKind::HourglassOfAsh => {
            let remaining = if world.get::<&TimeSlow>(player).is_ok() {
                TIME_SLOW_SECONDS - super::time_slow::EASE_SECONDS
            } else {
                TIME_SLOW_SECONDS
            };
            let _ = world.insert_one(player, TimeSlow { remaining });
        }
        ArtefactKind::BloodChalice => {
            let _ = world.insert(
                player,
                (
                    HealPercent(CHALICE_HEAL),
                    Bloodlust {
                        remaining: BLOODLUST_SECONDS,
                    },
                ),
            );
        }
        ArtefactKind::PhantomCloak => {
            let _ = world.insert_one(
                player,
                Phantom {
                    remaining: PHANTOM_SECONDS,
                },
            );
        }
        ArtefactKind::GraveLantern => {
            let _ = world.insert_one(player, SummonGhosts);
        }
        ArtefactKind::MidasCoin => {
            let _ = world.insert(
                player,
                (
                    ConvertBullets,
                    MidasTouch {
                        remaining: MIDAS_SECONDS,
                    },
                ),
            );
        }
        ArtefactKind::ThunderIdol => {
            let _ = world.insert_one(player, CastLightning);
        }
        ArtefactKind::InfinityQuiver => {
            let _ = world.insert_one(
                player,
                InfiniteAmmo {
                    remaining: INFINITE_AMMO_SECONDS,
                },
            );
        }
        ArtefactKind::BoneTurret => {
            let _ = world.insert_one(player, PlaceTurret);
        }
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

fn tick_timer<T: hecs::Component>(
    world: &mut World,
    dt: f32,
    remaining: impl Fn(&mut T) -> &mut f32,
) {
    let mut over = Vec::new();
    for (entity, effect) in world.query_mut::<(Entity, &mut T)>() {
        let left = remaining(effect);
        *left -= dt;
        if *left <= 0.0 {
            over.push(entity);
        }
    }
    for entity in over {
        let _ = world.remove_one::<T>(entity);
    }
}

pub(crate) fn tick_artefact_effects(world: &mut World, dt: f32) {
    tick_timer::<TimeSlow>(world, dt, |effect| &mut effect.remaining);
    tick_timer::<Bloodlust>(world, dt, |effect| &mut effect.remaining);
    tick_timer::<Phantom>(world, dt, |effect| &mut effect.remaining);
    tick_timer::<InfiniteAmmo>(world, dt, |effect| &mut effect.remaining);
    tick_timer::<MidasTouch>(world, dt, |effect| &mut effect.remaining);
}

/// Ends every timed artefact effect at once, for the death screen where the timers stop.
pub(crate) fn clear_artefact_effects(world: &mut World) {
    let players: Vec<Entity> = world.query::<Entity>().with::<&Player>().iter().collect();
    for player in players {
        let _ = world.remove_one::<TimeSlow>(player);
        let _ = world.remove_one::<Bloodlust>(player);
        let _ = world.remove_one::<Phantom>(player);
        let _ = world.remove_one::<InfiniteAmmo>(player);
        let _ = world.remove_one::<MidasTouch>(player);
    }
    super::blood::pulse_bloodlust(world);
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

    fn player_of(world: &World) -> Entity {
        world
            .query::<Entity>()
            .with::<&Player>()
            .iter()
            .next()
            .expect("a player")
    }

    fn activate(world: &mut World, physics: &mut Physics, kind: ArtefactKind) {
        give(world, Some(kind));
        activate_artefact(world, physics, &press_f(), TextureId::WHITE);
        assert_eq!(held(world), None);
    }

    #[test]
    fn every_new_artefact_inserts_its_components_and_consumes_the_slot() {
        let (mut world, mut physics, _) = setup();
        let player = player_of(&world);
        let has = |world: &World, check: fn(&hecs::EntityRef) -> bool| {
            world.entity(player).map(|e| check(&e)).unwrap_or(false)
        };
        activate(&mut world, &mut physics, ArtefactKind::HourglassOfAsh);
        assert!(has(&world, |e| e.has::<TimeSlow>()));
        activate(&mut world, &mut physics, ArtefactKind::BloodChalice);
        assert!(has(&world, |e| e.has::<HealPercent>() && e.has::<Bloodlust>()));
        activate(&mut world, &mut physics, ArtefactKind::PhantomCloak);
        assert!(has(&world, |e| e.has::<Phantom>()));
        activate(&mut world, &mut physics, ArtefactKind::GraveLantern);
        assert!(has(&world, |e| e.has::<SummonGhosts>()));
        activate(&mut world, &mut physics, ArtefactKind::MidasCoin);
        assert!(has(&world, |e| e.has::<ConvertBullets>()
            && e.has::<MidasTouch>()));
        activate(&mut world, &mut physics, ArtefactKind::ThunderIdol);
        assert!(has(&world, |e| e.has::<CastLightning>()));
        activate(&mut world, &mut physics, ArtefactKind::InfinityQuiver);
        assert!(has(&world, |e| e.has::<InfiniteAmmo>()));
        activate(&mut world, &mut physics, ArtefactKind::BoneTurret);
        assert!(has(&world, |e| e.has::<PlaceTurret>()));
    }

    #[test]
    fn timed_effects_expire_after_their_duration() {
        let (mut world, mut physics, _) = setup();
        let player = player_of(&world);
        activate(&mut world, &mut physics, ArtefactKind::HourglassOfAsh);
        tick_artefact_effects(&mut world, TIME_SLOW_SECONDS - 0.1);
        assert!(world.get::<&TimeSlow>(player).is_ok());
        tick_artefact_effects(&mut world, 0.2);
        assert!(world.get::<&TimeSlow>(player).is_err());
    }

    #[test]
    fn using_a_timed_artefact_again_resets_its_timer() {
        let (mut world, mut physics, _) = setup();
        let player = player_of(&world);
        activate(&mut world, &mut physics, ArtefactKind::PhantomCloak);
        tick_artefact_effects(&mut world, 3.0);
        activate(&mut world, &mut physics, ArtefactKind::PhantomCloak);
        let remaining = world.get::<&Phantom>(player).map(|p| p.remaining);
        assert_eq!(remaining.ok(), Some(PHANTOM_SECONDS));
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

    #[test]
    fn the_hourglass_used_again_stays_fully_slowed() {
        let (mut world, mut physics, _) = setup();
        let player = player_of(&world);
        activate(&mut world, &mut physics, ArtefactKind::HourglassOfAsh);
        tick_artefact_effects(&mut world, 5.0);
        let before = crate::systems::time_scale(&world);
        activate(&mut world, &mut physics, ArtefactKind::HourglassOfAsh);
        assert_eq!(crate::systems::time_scale(&world), before);
        let remaining = world.get::<&TimeSlow>(player).map(|s| s.remaining).ok();
        assert!(remaining.is_some_and(|r| r < TIME_SLOW_SECONDS));
    }

    #[test]
    fn clearing_the_effects_ends_them_and_restores_the_player_colour() {
        let (mut world, mut physics, _) = setup();
        let player = player_of(&world);
        let before = world
            .get::<&crate::components::Color>(player)
            .map(|c| c.0)
            .ok();
        for kind in [
            ArtefactKind::HourglassOfAsh,
            ArtefactKind::BloodChalice,
            ArtefactKind::PhantomCloak,
            ArtefactKind::InfinityQuiver,
            ArtefactKind::MidasCoin,
        ] {
            activate(&mut world, &mut physics, kind);
        }
        for _ in 0..3 {
            tick_artefact_effects(&mut world, 0.1);
            crate::systems::pulse_bloodlust(&mut world);
        }
        let tinted = world
            .get::<&crate::components::Color>(player)
            .map(|c| c.0)
            .ok();
        assert_ne!(tinted, before);

        clear_artefact_effects(&mut world);

        assert!(world.get::<&TimeSlow>(player).is_err());
        assert!(world.get::<&Bloodlust>(player).is_err());
        assert!(world.get::<&Phantom>(player).is_err());
        assert!(world.get::<&InfiniteAmmo>(player).is_err());
        assert!(world.get::<&MidasTouch>(player).is_err());
        let after = world
            .get::<&crate::components::Color>(player)
            .map(|c| c.0)
            .ok();
        assert_eq!(after, before);
    }
}
