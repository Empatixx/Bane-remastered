use glam::Vec2;
use hecs::{Entity, World};
use winit::keyboard::KeyCode;

use crate::assets::Textures;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Animation, Chest, ChestKind, Clip, GunDrop, InRange, Loadout, Player, Position, Size, Weapon,
};
use crate::geometry;
use crate::guns::GunKind;
use crate::input::Input;
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
    if !input.just_pressed(KeyCode::KeyQ) {
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
    let placed = world
        .query_mut::<(&Position, &mut Loadout)>()
        .with::<&Player>()
        .into_iter()
        .next()
        .map(|(position, loadout)| (position.0, loadout.add(weapon)));
    sounds.push(AudioCommand::Effect(SoundEffect::SwitchGun));
    if let Some((origin, Some(displaced))) = placed {
        prefabs::gun_drop(world, displaced, origin, Vec2::ZERO, textures);
    }
}

pub(crate) fn unused_gun(loadout: &Loadout, roll: f32) -> Option<GunKind> {
    let free: Vec<GunKind> = GunKind::ALL
        .into_iter()
        .filter(|kind| !loadout.has(*kind))
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
        crate::systems::burst_items(world, center, textures, |bound| {
            rand::random_range(0..bound)
        });
        if kind == ChestKind::Weapon {
            let kind = world
                .query::<&Loadout>()
                .with::<&Player>()
                .iter()
                .next()
                .and_then(|loadout| unused_gun(loadout, roll));
            if let Some(kind) = kind {
                prefabs::gun_drop(world, Weapon::new(kind), center, Vec2::ZERO, textures);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::Velocity;
    use crate::physics::Physics;

    use super::*;

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

        open_chests(&mut world, &textures, 0.0, &mut sounds);
        assert_eq!(sounds.len(), 1);
        assert!(world.get::<&Chest>(chest).is_ok_and(|c| c.opened));
        assert_eq!(world.query::<&GunDrop>().iter().count(), 0);

        if let Ok(mut animation) = world.get::<&mut Animation>(chest) {
            animation.advance(1.0);
        }
        open_chests(&mut world, &textures, 0.0, &mut sounds);

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
        open_chests(&mut world, &textures, 0.0, &mut Vec::new());
        assert!(world.get::<&Chest>(chest).is_ok_and(|c| !c.opened));

        let loadout = Loadout::starting();
        for step in 0..20 {
            let kind = unused_gun(&loadout, step as f32 / 20.0);
            assert!(kind.is_some_and(|kind| !loadout.has(kind)));
        }
    }
}
