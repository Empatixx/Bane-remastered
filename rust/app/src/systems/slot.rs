use std::f32::consts::TAU;

use glam::Vec2;
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Animation, Hidden, InRange, Player, Position, Size, SlotMachine, SlotPrice, Wallet,
};
use crate::guns::GunKind;
use crate::items::{AmmoBox, ItemKind, PotionSize, Vital};
use crate::objects::SPIN_COST;
use crate::prefabs;
use crate::sheets::slot::{LOSE, SPIN, WIN};
use crate::systems::loadout::unused_gun;

pub(crate) const SPIN_SECONDS: f32 = 2.0;
const TOTAL_CHANCE: usize = 150;
const SCATTER_SPEED: f32 = 417.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Outcome {
    RareItem,
    Cashout,
    ItemShower,
    Triple,
    Double,
    Lose,
}

pub(crate) fn outcome_for(chance: usize) -> Outcome {
    match chance {
        0..=9 => Outcome::RareItem,
        10..=19 => Outcome::Cashout,
        20..=29 => Outcome::ItemShower,
        30..=39 => Outcome::Triple,
        40..=59 => Outcome::Double,
        _ => Outcome::Lose,
    }
}

pub(crate) fn spin(world: &mut World, sounds: &mut Vec<AudioCommand>) {
    let machine = world
        .query::<(Entity, &SlotMachine)>()
        .with::<&InRange>()
        .iter()
        .find(|(_, machine)| machine.spinning.is_none())
        .map(|(entity, _)| entity);
    let Some(machine) = machine else {
        return;
    };
    let paid = world
        .query_mut::<&mut Wallet>()
        .with::<&Player>()
        .into_iter()
        .next()
        .is_some_and(|wallet| {
            let enough = wallet.coins >= SPIN_COST;
            if enough {
                wallet.coins -= SPIN_COST;
            }
            enough
        });
    if !paid {
        return;
    }
    if let Ok(mut state) = world.get::<&mut SlotMachine>(machine) {
        state.spinning = Some(SPIN_SECONDS);
    }
    if let Ok(mut animation) = world.get::<&mut Animation>(machine) {
        animation.play(SPIN);
    }
    sounds.push(AudioCommand::Effect(SoundEffect::SlotSpin));
}

fn scatter(
    world: &mut World,
    center: Vec2,
    kinds: impl Iterator<Item = ItemKind>,
    count: usize,
    textures: &Textures,
) {
    for (index, kind) in kinds.enumerate() {
        let angle = TAU * index as f32 / count as f32;
        let velocity = Vec2::new(angle.cos(), angle.sin()) * SCATTER_SPEED;
        prefabs::item(world, kind, center, true, velocity, textures);
    }
}

fn rare_prize(
    world: &mut World,
    center: Vec2,
    textures: &Textures,
    roll: &mut impl FnMut(usize) -> usize,
) {
    let velocity = Vec2::new(0.0, SCATTER_SPEED);
    let uncommon = roll(2) == 0;
    let mut artefact = false;
    let item = if uncommon {
        let vital = if roll(2) == 0 {
            Vital::Health
        } else {
            Vital::Armor
        };
        Some(ItemKind::Potion(vital, PotionSize::Small))
    } else {
        match roll(5) {
            0 => None,
            1 => {
                artefact = true;
                None
            }
            2 => Some(ItemKind::Ammo(AmmoBox::Universal)),
            3 => Some(ItemKind::Potion(Vital::Health, PotionSize::Large)),
            _ => Some(ItemKind::Potion(Vital::Armor, PotionSize::Large)),
        }
    };
    match item {
        Some(kind) => {
            prefabs::item(world, kind, center, true, velocity, textures);
        }
        None if artefact => {
            let slot = world
                .query::<&crate::components::ArtefactSlot>()
                .with::<&Player>()
                .iter()
                .next()
                .copied();
            if let Some(slot) = slot {
                let kind = crate::systems::unused_artefact(&slot, roll(1000));
                prefabs::artefact_drop(world, kind, center, velocity, textures);
            }
        }
        None => {
            let gun: Option<GunKind> = world
                .query::<&crate::components::Loadout>()
                .with::<&Player>()
                .iter()
                .next()
                .and_then(|loadout| unused_gun(loadout, roll(1000) as f32 / 1000.0));
            if let Some(kind) = gun {
                let weapon = crate::systems::loadout::fresh_weapon(world, kind);
                let drop = prefabs::gun_drop(world, weapon, center, velocity, textures);
                let _ = world.remove_one::<crate::components::Interactable>(drop);
                let _ = world.insert_one(
                    drop,
                    crate::components::Interactable {
                        action: crate::components::Interaction::PickUpGun,
                    },
                );
            }
        }
    }
}

pub(crate) fn tick_slot_machines(
    world: &mut World,
    textures: &Textures,
    dt: f32,
    sounds: &mut Vec<AudioCommand>,
    mut roll: impl FnMut(usize) -> usize,
) {
    let mut finished = Vec::new();
    for (entity, machine, position, size) in
        world.query_mut::<(Entity, &mut SlotMachine, &Position, &Size)>()
    {
        if let Some(remaining) = machine.spinning {
            let remaining = remaining - dt;
            if remaining <= 0.0 {
                machine.spinning = None;
                finished.push((entity, position.0 + Vec2::new(0.0, size.0.y / 8.0)));
            } else {
                machine.spinning = Some(remaining);
            }
        }
    }
    for (entity, center) in finished {
        let outcome = outcome_for(roll(TOTAL_CHANCE));
        let (sound, clip) = match outcome {
            Outcome::Lose => (SoundEffect::SlotLose, LOSE),
            Outcome::Cashout => (SoundEffect::SlotCashout, WIN),
            _ => (SoundEffect::SlotWin, WIN),
        };
        match outcome {
            Outcome::RareItem => rare_prize(world, center, textures, &mut roll),
            Outcome::Cashout => scatter(
                world,
                center,
                std::iter::repeat_n(ItemKind::Coin, 10),
                10,
                textures,
            ),
            Outcome::Triple => scatter(
                world,
                center,
                std::iter::repeat_n(ItemKind::Coin, (SPIN_COST * 3) as usize),
                (SPIN_COST * 3) as usize,
                textures,
            ),
            Outcome::Double => scatter(
                world,
                center,
                std::iter::repeat_n(ItemKind::Coin, (SPIN_COST * 2) as usize),
                (SPIN_COST * 2) as usize,
                textures,
            ),
            Outcome::ItemShower => {
                let kinds: Vec<ItemKind> = (0..5)
                    .map(|_| ItemKind::COMMON[roll(ItemKind::COMMON.len())])
                    .collect();
                scatter(world, center, kinds.into_iter(), 5, textures);
            }
            Outcome::Lose => {}
        }
        sounds.push(AudioCommand::Effect(sound));
        if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
            animation.play(clip);
        }
    }
}

pub(crate) fn update_slot_prices(world: &mut World) {
    let tags: Vec<(Entity, Entity)> = world
        .query::<(Entity, &SlotPrice)>()
        .iter()
        .map(|(entity, tag)| (entity, tag.owner))
        .collect();
    for (tag, owner) in tags {
        let spinning = world
            .get::<&SlotMachine>(owner)
            .is_ok_and(|machine| machine.spinning.is_some());
        if spinning {
            let _ = world.insert_one(tag, Hidden);
        } else {
            let _ = world.remove_one::<Hidden>(tag);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{Pickup, Velocity};
    use crate::physics::Physics;

    use super::*;
    use crate::clip::Clip;
    use crate::sheets::slot;

    fn setup(coins: i32) -> (World, Entity, Textures) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(&mut world, &mut physics, Vec2::new(100.0, 100.0), &textures);
        for wallet in world.query_mut::<&mut Wallet>().with::<&Player>() {
            wallet.coins = coins;
        }
        let machine = prefabs::slot_machine(&mut world, Vec2::new(0.0, 0.0), &textures);
        let _ = world.insert_one(machine, InRange);
        (world, machine, textures)
    }

    fn coins(world: &World) -> i32 {
        world
            .query::<&Wallet>()
            .with::<&Player>()
            .iter()
            .next()
            .map_or(-1, |w| w.coins)
    }

    #[test]
    fn the_odds_match_bane() {
        let count = |wanted: Outcome| {
            (0..TOTAL_CHANCE)
                .filter(|c| outcome_for(*c) == wanted)
                .count()
        };
        assert_eq!(count(Outcome::RareItem), 10);
        assert_eq!(count(Outcome::Cashout), 10);
        assert_eq!(count(Outcome::ItemShower), 10);
        assert_eq!(count(Outcome::Triple), 10);
        assert_eq!(count(Outcome::Double), 20);
        assert_eq!(count(Outcome::Lose), 90);
    }

    #[test]
    fn a_spin_costs_five_coins_and_cannot_be_started_twice() {
        let (mut world, machine, _) = setup(12);
        let mut sounds = Vec::new();

        spin(&mut world, &mut sounds);
        spin(&mut world, &mut sounds);

        assert_eq!(coins(&world), 7);
        assert_eq!(sounds, vec![AudioCommand::Effect(SoundEffect::SlotSpin)]);
        assert!(
            world
                .get::<&SlotMachine>(machine)
                .is_ok_and(|m| m.spinning.is_some())
        );
    }

    #[test]
    fn without_five_coins_the_machine_stays_idle() {
        let (mut world, machine, _) = setup(4);
        spin(&mut world, &mut Vec::new());
        assert_eq!(coins(&world), 4);
        assert!(
            world
                .get::<&SlotMachine>(machine)
                .is_ok_and(|m| m.spinning.is_none())
        );
    }

    fn spin_with(chance: usize) -> (World, Vec<AudioCommand>) {
        let (mut world, _, textures) = setup(10);
        let mut sounds = Vec::new();
        spin(&mut world, &mut sounds);
        sounds.clear();
        let mut first = true;
        tick_slot_machines(
            &mut world,
            &textures,
            SPIN_SECONDS + 0.1,
            &mut sounds,
            |bound| {
                if std::mem::take(&mut first) {
                    chance
                } else {
                    bound - 1
                }
            },
        );
        (world, sounds)
    }

    #[test]
    fn losing_plays_the_lose_sound_and_drops_nothing() {
        let (world, sounds) = spin_with(100);
        assert_eq!(sounds, vec![AudioCommand::Effect(SoundEffect::SlotLose)]);
        assert_eq!(world.query::<&Pickup>().iter().count(), 0);
    }

    #[test]
    fn a_double_scatters_ten_coins_and_a_triple_fifteen() {
        let (world, _) = spin_with(50);
        assert_eq!(world.query::<&Pickup>().iter().count(), 10);
        let (world, _) = spin_with(35);
        assert_eq!(world.query::<&Pickup>().iter().count(), 15);
        let flying = world
            .query::<&Velocity>()
            .with::<&Pickup>()
            .iter()
            .filter(|v| v.0.length() > 400.0)
            .count();
        assert_eq!(flying, 15);
    }

    #[test]
    fn the_item_shower_drops_five_things_and_cashout_ten_coins() {
        let (world, sounds) = spin_with(25);
        assert_eq!(world.query::<&Pickup>().iter().count(), 5);
        assert_eq!(sounds, vec![AudioCommand::Effect(SoundEffect::SlotWin)]);
        let (world, sounds) = spin_with(15);
        assert_eq!(world.query::<&Pickup>().iter().count(), 10);
        assert_eq!(sounds, vec![AudioCommand::Effect(SoundEffect::SlotCashout)]);
    }

    #[test]
    fn the_rare_prize_is_one_useful_thing() {
        let (world, sounds) = spin_with(3);
        let items = world.query::<&Pickup>().iter().count()
            + world.query::<&crate::components::GunDrop>().iter().count();
        assert_eq!(items, 1);
        assert_eq!(sounds, vec![AudioCommand::Effect(SoundEffect::SlotWin)]);
    }

    #[test]
    fn the_price_tag_hides_while_the_reels_spin() {
        let (mut world, machine, _) = setup(10);
        update_slot_prices(&mut world);
        assert_eq!(
            world.query::<&Hidden>().with::<&SlotPrice>().iter().count(),
            0
        );
        spin(&mut world, &mut Vec::new());
        update_slot_prices(&mut world);
        assert_eq!(
            world.query::<&Hidden>().with::<&SlotPrice>().iter().count(),
            2
        );
        let _ = machine;
    }

    fn clip_of(world: &World, machine: Entity) -> Option<Clip> {
        world.get::<&Animation>(machine).ok().map(|a| a.clip())
    }

    #[test]
    fn the_machine_has_an_animation_that_matches_its_sheet() {
        let (world, machine, _textures) = setup(0);

        assert_eq!(clip_of(&world, machine), Some(slot::IDLE));
        let spec = crate::props::Prop::SlotMachine.spec();
        assert_eq!((spec.columns, spec.rows), (slot::COLUMNS, slot::ROWS));
        const { assert!(SPIN.frames <= slot::COLUMNS && WIN.frames <= slot::COLUMNS) };
        const { assert!(SPIN.row < slot::ROWS && WIN.row < slot::ROWS && LOSE.row < slot::ROWS) };
    }

    #[test]
    fn spinning_plays_the_reels_and_the_result_plays_when_they_stop() {
        let (mut world, machine, textures) = setup(SPIN_COST);
        let mut sounds = Vec::new();

        spin(&mut world, &mut sounds);
        assert_eq!(clip_of(&world, machine), Some(SPIN));

        tick_slot_machines(
            &mut world,
            &textures,
            SPIN_SECONDS + 0.1,
            &mut sounds,
            |_| 5,
        );
        assert_eq!(clip_of(&world, machine), Some(WIN));

        let (mut world, machine, textures) = setup(SPIN_COST);
        spin(&mut world, &mut sounds);
        tick_slot_machines(
            &mut world,
            &textures,
            SPIN_SECONDS + 0.1,
            &mut sounds,
            |_| 100,
        );
        assert_eq!(clip_of(&world, machine), Some(LOSE));
    }
}
