use glam::Vec2;
use hecs::{Entity, World};
use winit::keyboard::KeyCode;

use crate::assets::Textures;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Animation, ArtefactSlot, Chest, ChestKind, Clip, GunDrop, InRange, Loadout, Player, Position,
    STARTING_SLOTS, Size, Weapon,
};
use crate::controls::Action;
use crate::geometry;
use crate::guns::{AmmoClass, GunKind, Mutation};
use crate::input::Input;
use crate::items::AmmoBox;
use crate::prefabs;

const SLOT_KEYS: [KeyCode; 9] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
    KeyCode::Digit6,
    KeyCode::Digit7,
    KeyCode::Digit8,
    KeyCode::Digit9,
];
const DROP_SPEED: f32 = 320.0;

pub(crate) const CHEST_OPEN: Clip = Clip {
    row: 1,
    frames: 4,
    frame_time: 0.175,
    looping: false,
    flipped: false,
};

/// Ticks a weapon's cooldown and its reload; a finished reload moves ammo from the reserve into the clip.
pub(crate) fn advance_weapon(weapon: &mut Weapon, dt: f32) {
    weapon.cooldown = (weapon.cooldown - dt).max(0.0);
    if let Some(remaining) = weapon.reloading {
        let remaining = remaining - dt;
        if remaining > 0.0 {
            weapon.reloading = Some(remaining);
        } else {
            weapon.reloading = None;
            let missing = weapon.clip_size() - weapon.clip;
            let loaded = missing.min(weapon.ammo);
            weapon.clip += loaded;
            weapon.ammo -= loaded;
        }
    }
}

/// Begins a reload if the gun can take one; whether it started.
pub(crate) fn start_reload(weapon: &mut Weapon) -> bool {
    let can = !weapon.runs_on_battery()
        && weapon.burst.is_none()
        && weapon.reloading.is_none()
        && weapon.ammo > 0
        && weapon.clip < weapon.clip_size();
    if can {
        weapon.reloading = Some(weapon.spec().reload_time);
    }
    can
}

/// Counts a pulled trigger and says whether Deadeye makes it a critical hit.
pub(crate) fn count_shot(weapon: &mut Weapon) -> bool {
    weapon.shots = weapon.shots.wrapping_add(1);
    weapon.spec().mutation == Some(Mutation::Deadeye)
        && weapon
            .shots
            .is_multiple_of(crate::weapon_upgrades::DEADEYE_EVERY)
}

/// Limits the loadout to one kind of gun (or lifts the limit), emptying the slots of any other kind.
pub(crate) fn restrict_to(loadout: &mut Loadout, only: Option<GunKind>) {
    loadout.only = only;
    let Some(only) = only else {
        return;
    };
    for slot in &mut loadout.slots {
        if slot.as_ref().is_some_and(|weapon| weapon.kind != only) {
            *slot = None;
        }
    }
    if loadout
        .slots
        .get(loadout.current)
        .is_none_or(Option::is_none)
    {
        loadout.current = loadout.slots.iter().position(Option::is_some).unwrap_or(0);
    }
}

/// Grows the slots to the starting count plus the extra ones and sets every gun's ammo capacity scale.
pub(crate) fn apply_perks(loadout: &mut Loadout, extra_slots: usize, ammo_scale: f32) {
    while loadout.slots.len() < STARTING_SLOTS + extra_slots {
        loadout.slots.push(None);
    }
    loadout.ammo_scale = ammo_scale;
    for weapon in loadout.slots.iter_mut().flatten() {
        weapon.ammo_scale = ammo_scale;
    }
}

/// The kind of ammo the held gun that is emptiest needs, if any gun needs some.
pub(crate) fn neediest_ammo(loadout: &Loadout) -> Option<AmmoClass> {
    loadout
        .slots
        .iter()
        .flatten()
        .filter(|weapon| !weapon.has_full_ammo())
        .min_by(|a, b| a.fullness().total_cmp(&b.fullness()))
        .map(|weapon| weapon.kind.ammo_class())
}

/// Whether any held gun takes this box and has room for more.
pub(crate) fn wants_ammo(loadout: &Loadout, ammo: AmmoBox) -> bool {
    loadout
        .slots
        .iter()
        .flatten()
        .any(|weapon| Loadout::takes(weapon, ammo) && !weapon.has_full_ammo())
}

/// Pours a box into every held gun that takes it.
pub(crate) fn add_ammo(loadout: &mut Loadout, ammo: AmmoBox) {
    for weapon in loadout.slots.iter_mut().flatten() {
        if Loadout::takes(weapon, ammo) {
            weapon.add_percent(ammo.percent());
        }
    }
}

/// Takes a gun into the loadout, putting on the upgrades; returns the gun it pushed out, if the slots were full.
pub(crate) fn add_gun(loadout: &mut Loadout, mut weapon: Weapon) -> Option<Weapon> {
    weapon.ammo_scale = loadout.ammo_scale;
    weapon.upgrade(loadout.mods.of(weapon.kind));
    let displaced = if loadout.slots.iter().all(Option::is_some) {
        loadout.drop_current()
    } else {
        None
    };
    if loadout.slots[loadout.current].is_none() {
        loadout.slots[loadout.current] = Some(weapon);
    } else if let Some(free) = loadout.slots.iter().position(Option::is_none) {
        loadout.slots[free] = Some(weapon);
    }
    displaced
}

pub(crate) fn switch_guns(world: &mut World, input: &Input, sounds: &mut Vec<AudioCommand>) {
    for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
        for (slot, key) in SLOT_KEYS.iter().enumerate() {
            if input.just_pressed(*key) && loadout.switch_to(slot) {
                sounds.push(AudioCommand::Effect(SoundEffect::SwitchGun));
            }
        }
        for _ in 0..input.scrolled().max(0) {
            loadout.next();
        }
        for _ in 0..(-input.scrolled()).max(0) {
            loadout.previous();
        }
    }
}

pub(crate) fn drop_gun(world: &mut World, input: &Input, camera: Vec2, textures: &Textures) {
    if !input.action_pressed(Action::DropGun) {
        return;
    }
    let target = input.mouse() + camera;
    let dropped = world
        .query_mut::<(&Position, &mut Loadout)>()
        .with::<&Player>()
        .into_iter()
        .next()
        .and_then(|(position, loadout)| loadout.drop_current().map(|weapon| (position.0, weapon)));
    if let Some((origin, weapon)) = dropped {
        let direction = (target - origin).normalize_or(Vec2::X);
        prefabs::gun_drop(world, weapon, origin, direction * DROP_SPEED, textures);
    }
}

pub(crate) fn pick_up_gun(world: &mut World, textures: &Textures, sounds: &mut Vec<AudioCommand>) {
    let drop = world
        .query::<(Entity, &GunDrop)>()
        .with::<&InRange>()
        .iter()
        .next()
        .map(|(entity, drop)| (entity, drop.weapon));
    let Some((entity, weapon)) = drop else {
        return;
    };
    let _ = world.despawn(entity);
    super::stats::record(world, super::stats::Stat::Item, 1);
    let placed = world
        .query_mut::<(&Position, &mut Loadout)>()
        .with::<&Player>()
        .into_iter()
        .next()
        .map(|(position, loadout)| (position.0, add_gun(loadout, weapon)));
    sounds.push(AudioCommand::Effect(SoundEffect::SwitchGun));
    if let Some((origin, Some(displaced))) = placed {
        prefabs::gun_drop(world, displaced, origin, Vec2::ZERO, textures);
    }
}

pub(crate) fn fresh_weapon(world: &World, kind: GunKind) -> Weapon {
    world
        .query::<&Loadout>()
        .with::<&Player>()
        .iter()
        .next()
        .map_or_else(|| Weapon::new(kind), |loadout| loadout.fresh(kind))
}

pub(crate) fn unused_gun(loadout: &Loadout, roll: f32) -> Option<GunKind> {
    let free: Vec<GunKind> = GunKind::ALL
        .into_iter()
        .filter(|kind| !loadout.has(*kind) && loadout.allows(*kind))
        .collect();
    if free.is_empty() {
        return None;
    }
    Some(free[((roll * free.len() as f32) as usize).min(free.len() - 1)])
}

pub(crate) fn open_chests(
    world: &mut World,
    textures: &Textures,
    roll: f32,
    mut pick: impl FnMut(usize) -> usize,
    sounds: &mut Vec<AudioCommand>,
) {
    let player = world
        .query::<(&Position, &Size)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(position, size)| (position.0, size.0));
    let mut opening = Vec::new();
    let mut finished = Vec::new();
    for (entity, chest, position, size, animation) in
        world.query_mut::<(Entity, &mut Chest, &Position, &Size, &mut Animation)>()
    {
        if chest.opened {
            if animation.finished() {
                finished.push((entity, chest.kind, position.0));
            }
        } else if player
            .is_some_and(|(at, extent)| geometry::overlaps(at, extent, position.0, size.0))
        {
            chest.opened = true;
            animation.play(CHEST_OPEN);
            opening.push(entity);
        }
    }
    if !opening.is_empty() {
        sounds.push(AudioCommand::Effect(SoundEffect::OpenChest));
    }
    for (entity, kind, center) in finished {
        let _ = world.despawn(entity);
        crate::systems::burst_items(world, center, textures, &mut pick);
        if kind == ChestKind::Artefact {
            let slot = world
                .query::<&ArtefactSlot>()
                .with::<&Player>()
                .iter()
                .next()
                .copied();
            if let Some(slot) = slot {
                let found = crate::systems::unused_artefact(&slot, (roll * 1000.0) as usize);
                prefabs::artefact_drop(world, found, center, Vec2::ZERO, textures);
            }
        }
        if kind == ChestKind::Weapon {
            let kind = world
                .query::<&Loadout>()
                .with::<&Player>()
                .iter()
                .next()
                .and_then(|loadout| unused_gun(loadout, roll));
            if let Some(kind) = kind {
                let weapon = fresh_weapon(world, kind);
                prefabs::gun_drop(world, weapon, center, Vec2::ZERO, textures);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{Burst, Velocity};
    use crate::physics::Physics;

    use super::*;

    #[test]
    fn a_restricted_loadout_is_never_offered_another_gun() {
        let mut loadout = Loadout::starting();
        restrict_to(&mut loadout, Some(GunKind::Pistol));
        for roll in [0.0, 0.3, 0.6, 0.99] {
            assert_eq!(unused_gun(&loadout, roll), None);
        }
    }

    fn setup() -> (World, Textures) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(&mut world, &mut physics, Vec2::new(100.0, 100.0), &textures);
        (world, textures)
    }

    fn current(world: &World) -> Option<GunKind> {
        world
            .query::<&Loadout>()
            .with::<&Player>()
            .iter()
            .next()
            .and_then(|loadout| loadout.current().map(|weapon| weapon.kind))
    }

    #[test]
    fn number_keys_and_the_wheel_switch_slots() {
        let (mut world, _) = setup();
        let mut sounds = Vec::new();
        let mut input = Input::default();
        input.press(KeyCode::Digit2);

        switch_guns(&mut world, &input, &mut sounds);
        assert_eq!(current(&world), Some(GunKind::Shotgun));
        assert_eq!(sounds.len(), 1);

        input.end_frame();
        input.press(KeyCode::Digit2);
        switch_guns(&mut world, &input, &mut sounds);
        assert_eq!(sounds.len(), 1);

        input.end_frame();
        input.handle_scroll(1.0);
        switch_guns(&mut world, &input, &mut sounds);
        assert_eq!(current(&world), Some(GunKind::Pistol));
    }

    #[test]
    fn q_drops_the_current_gun_towards_the_mouse_and_e_picks_it_up_again() {
        let (mut world, textures) = setup();
        let mut input = Input::default();
        input.set_mouse(Vec2::new(300.0, 100.0));
        input.press(KeyCode::KeyQ);

        drop_gun(&mut world, &input, Vec2::ZERO, &textures);
        assert_eq!(current(&world), None);
        let velocity = world
            .query::<&Velocity>()
            .with::<&GunDrop>()
            .iter()
            .next()
            .map(|v| v.0)
            .unwrap_or_default();
        assert!(velocity.x > 300.0 && velocity.y.abs() < 1.0, "{velocity:?}");

        for _ in 0..120 {
            crate::systems::glide(&mut world, 1.0 / 60.0);
        }
        let entity = world.query::<Entity>().with::<&GunDrop>().iter().next();
        let entity = entity.expect("the gun lies on the floor");
        let _ = world.insert_one(entity, InRange);
        pick_up_gun(&mut world, &textures, &mut Vec::new());

        assert_eq!(current(&world), Some(GunKind::Pistol));
        assert_eq!(world.query::<&GunDrop>().iter().count(), 0);
    }

    #[test]
    fn picking_up_with_full_slots_drops_the_current_gun() {
        let (mut world, textures) = setup();
        let drop = prefabs::gun_drop(
            &mut world,
            Weapon::new(GunKind::Uzi),
            Vec2::new(100.0, 100.0),
            Vec2::ZERO,
            &textures,
        );
        let _ = world.insert_one(drop, InRange);

        pick_up_gun(&mut world, &textures, &mut Vec::new());

        assert_eq!(current(&world), Some(GunKind::Uzi));
        let lying: Vec<GunKind> = world
            .query::<&GunDrop>()
            .iter()
            .map(|drop| drop.weapon.kind)
            .collect();
        assert_eq!(lying, vec![GunKind::Pistol]);
    }

    #[test]
    fn a_weapon_chest_opens_on_touch_and_then_drops_a_gun_you_do_not_have() {
        let (mut world, textures) = setup();
        let chest = prefabs::chest(
            &mut world,
            ChestKind::Weapon,
            Vec2::new(80.0, 80.0),
            &textures,
        );
        let mut sounds = Vec::new();

        open_chests(&mut world, &textures, 0.0, |_| 0, &mut sounds);
        assert_eq!(sounds.len(), 1);
        assert!(world.get::<&Chest>(chest).is_ok_and(|c| c.opened));
        assert_eq!(world.query::<&GunDrop>().iter().count(), 0);

        if let Ok(mut animation) = world.get::<&mut Animation>(chest) {
            animation.advance(1.0);
        }
        open_chests(&mut world, &textures, 0.0, |_| 0, &mut sounds);

        assert!(world.get::<&Chest>(chest).is_err());
        let kinds: Vec<GunKind> = world
            .query::<&GunDrop>()
            .iter()
            .map(|drop| drop.weapon.kind)
            .collect();
        assert_eq!(kinds, vec![GunKind::Luger]);
    }

    #[test]
    fn a_far_chest_stays_closed_and_the_random_gun_is_never_one_you_hold() {
        let (mut world, textures) = setup();
        let chest = prefabs::chest(
            &mut world,
            ChestKind::Weapon,
            Vec2::new(900.0, 900.0),
            &textures,
        );
        open_chests(&mut world, &textures, 0.0, |_| 0, &mut Vec::new());
        assert!(world.get::<&Chest>(chest).is_ok_and(|c| !c.opened));

        let loadout = Loadout::starting();
        for step in 0..20 {
            let kind = unused_gun(&loadout, step as f32 / 20.0);
            assert!(kind.is_some_and(|kind| !loadout.has(kind)));
        }
    }

    #[test]
    fn the_boss_chest_drops_an_artefact_you_do_not_hold_plus_five_items() {
        let (mut world, textures) = setup();
        let chest = prefabs::chest(
            &mut world,
            ChestKind::Artefact,
            Vec2::new(80.0, 80.0),
            &textures,
        );
        let mut sounds = Vec::new();
        open_chests(&mut world, &textures, 0.0, |_| 0, &mut sounds);
        if let Ok(mut animation) = world.get::<&mut Animation>(chest) {
            animation.advance(1.0);
        }
        open_chests(&mut world, &textures, 0.0, |_| 0, &mut sounds);

        let artefacts: Vec<crate::artefacts::ArtefactKind> = world
            .query::<&crate::components::ArtefactDrop>()
            .iter()
            .map(|drop| drop.kind)
            .collect();
        assert_eq!(artefacts.len(), 1);
        assert_ne!(artefacts[0], crate::artefacts::ArtefactKind::RevealMap);
        assert_eq!(
            world.query::<&crate::components::Pickup>().iter().count(),
            5
        );
        assert_eq!(world.query::<&GunDrop>().iter().count(), 0);
    }

    #[test]
    fn reload_refills_the_clip_from_the_reserve_after_the_reload_time() {
        let mut weapon = Weapon::pistol();
        weapon.clip = 2;
        weapon.ammo = 10;
        assert!(start_reload(&mut weapon));
        assert!(!weapon.can_fire());

        advance_weapon(&mut weapon, 0.5);
        assert_eq!(weapon.clip, 2);
        advance_weapon(&mut weapon, 0.3);
        assert_eq!((weapon.clip, weapon.ammo), (7, 5));
        assert!(weapon.reloading.is_none());
    }

    #[test]
    fn reload_uses_whatever_reserve_is_left() {
        let mut weapon = Weapon::pistol();
        weapon.clip = 0;
        weapon.ammo = 3;
        start_reload(&mut weapon);
        advance_weapon(&mut weapon, 1.0);
        assert_eq!((weapon.clip, weapon.ammo), (3, 0));
    }

    #[test]
    fn nothing_happens_with_a_full_clip_or_no_reserve() {
        let mut full = Weapon::pistol();
        assert!(!start_reload(&mut full));
        assert!(full.reloading.is_none());

        let mut dry = Weapon::pistol();
        dry.clip = 0;
        dry.ammo = 0;
        assert!(!start_reload(&mut dry));
        assert!(dry.reloading.is_none());
        assert!(!dry.can_fire());
    }

    #[test]
    fn a_gun_cannot_reload_in_the_middle_of_a_burst() {
        let mut m4 = Weapon::new(GunKind::M4);
        m4.clip = 10;
        m4.burst = Some(Burst {
            left: 2,
            timer: 0.05,
            aim: Vec2::X,
        });
        assert!(!start_reload(&mut m4));
        m4.burst = None;
        assert!(start_reload(&mut m4));
    }

    #[test]
    fn pistol_ammo_does_not_feed_the_shotgun_and_universal_feeds_everything() {
        let mut loadout = Loadout::starting();
        for weapon in loadout.slots.iter_mut().flatten() {
            weapon.ammo = 0;
        }
        add_ammo(&mut loadout, AmmoBox::Pistol);
        let ammo: Vec<i32> = loadout.slots.iter().flatten().map(|w| w.ammo).collect();
        assert_eq!(ammo, vec![12, 0]);

        add_ammo(&mut loadout, AmmoBox::Universal);
        let ammo: Vec<i32> = loadout.slots.iter().flatten().map(|w| w.ammo).collect();
        assert_eq!(ammo, vec![72, 24]);
    }

    #[test]
    fn full_guns_do_not_want_ammo_and_ammo_never_exceeds_the_maximum() {
        let mut loadout = Loadout::starting();
        assert!(!wants_ammo(&loadout, AmmoBox::Universal));
        if let Some(weapon) = loadout.current_mut() {
            weapon.ammo = 118;
        }
        assert!(wants_ammo(&loadout, AmmoBox::SubMachine));
        assert!(!wants_ammo(&loadout, AmmoBox::Shotgun));
        add_ammo(&mut loadout, AmmoBox::Universal);
        assert_eq!(loadout.current().map(|w| w.ammo), Some(120));
    }

    #[test]
    fn upgrades_add_a_slot_and_raise_the_ammo_capacity_of_every_gun() {
        let mut loadout = Loadout::starting();
        apply_perks(&mut loadout, 1, 1.1);
        assert!(loadout.switch_to(2));
        let fresh = loadout.fresh(GunKind::Uzi);
        assert_eq!((fresh.max_ammo(), fresh.ammo), (220, 220));
        assert_eq!(add_gun(&mut loadout, fresh).map(|w| w.kind), None);
        assert!(loadout.has(GunKind::Uzi));
        let pistol = loadout.slots[0].map(|w| (w.max_ammo(), w.ammo));
        assert_eq!(pistol, Some((132, 120)));
        assert!(!loadout.slots[0].is_some_and(|w| w.has_full_ammo()));
    }

    #[test]
    fn dropping_empties_the_slot_and_the_next_gun_fills_it() {
        let mut loadout = Loadout::starting();
        let dropped = loadout.drop_current();
        assert_eq!(dropped.map(|w| w.kind), Some(GunKind::Pistol));
        assert!(loadout.current().is_none());

        let displaced = add_gun(&mut loadout, Weapon::new(GunKind::Uzi));
        assert!(displaced.is_none());
        assert_eq!(loadout.current().map(|w| w.kind), Some(GunKind::Uzi));
    }

    #[test]
    fn picking_up_with_full_slots_swaps_out_the_current_gun() {
        let mut loadout = Loadout::starting();
        loadout.switch_to(1);
        let displaced = add_gun(&mut loadout, Weapon::new(GunKind::M4));
        assert_eq!(displaced.map(|w| w.kind), Some(GunKind::Shotgun));
        assert_eq!(loadout.current().map(|w| w.kind), Some(GunKind::M4));
        assert!(loadout.has(GunKind::Pistol));
    }

    #[test]
    fn a_free_other_slot_is_used_when_the_current_one_is_taken() {
        let mut loadout = Loadout::starting();
        loadout.switch_to(1);
        loadout.drop_current();
        loadout.switch_to(0);
        let displaced = add_gun(&mut loadout, Weapon::new(GunKind::Luger));
        assert!(displaced.is_none());
        assert_eq!(loadout.current().map(|w| w.kind), Some(GunKind::Pistol));
        loadout.switch_to(1);
        assert_eq!(loadout.current().map(|w| w.kind), Some(GunKind::Luger));
    }

    #[test]
    fn each_box_feeds_only_its_own_guns_and_the_universal_box_feeds_all() {
        let mut tesla = Weapon::new(GunKind::TeslaGun);
        tesla.clip = 0;
        let mut loadout = Loadout::starting();
        loadout.drop_current();
        add_gun(&mut loadout, tesla);
        assert!(wants_ammo(&loadout, AmmoBox::Energy));
        assert!(!wants_ammo(&loadout, AmmoBox::Pistol) || loadout.has(GunKind::Pistol));
        assert!(!wants_ammo(&loadout, AmmoBox::Rocket));
        assert_eq!(neediest_ammo(&loadout), Some(AmmoClass::Energy));
        add_ammo(&mut loadout, AmmoBox::Energy);
        let charged = loadout
            .current()
            .map(|weapon| weapon.clip)
            .expect("the tesla is in hand");
        assert!(charged > 0);
    }

    #[test]
    fn a_laser_gun_never_reloads_and_a_battery_cell_tops_up_its_charge() {
        let mut blaster = Weapon::new(GunKind::Blaster);
        assert_eq!(blaster.ammo, 0, "all the charge is in the battery");
        blaster.clip = 10;
        assert!(!start_reload(&mut blaster));
        blaster.add_percent(AmmoBox::Energy.percent());
        assert_eq!(blaster.clip, 10 + blaster.clip_size() / 4);
        blaster.add_percent(1.0);
        assert_eq!(blaster.clip, blaster.clip_size());
        assert!(blaster.has_full_ammo());
    }
}
