use glam::Vec2;

use super::animation::Clip;
use crate::guns::{AmmoClass, GunKind, GunSpec, Mutation};
use crate::items::AmmoBox;
use crate::weapon_upgrades::{GunMods, WeaponMods};

pub(crate) const BULLET_COLUMNS: usize = 4;
pub(crate) const BULLET_ROWS: usize = 2;

pub(crate) const BULLET_FLIGHT: Clip = Clip {
    row: 0,
    frames: 4,
    frame_time: 0.07,
    looping: true,
    flipped: false,
};

pub(crate) const BULLET_IMPACT: Clip = Clip {
    row: 1,
    frames: 4,
    frame_time: 0.07,
    looping: false,
    flipped: false,
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Bullet {
    pub(crate) damage: i32,
    pub(crate) pierce: u32,
    pub(crate) bounces: u32,
    pub(crate) mutation: Option<Mutation>,
    pub(crate) last_hit: Option<hecs::Entity>,
}

impl Bullet {
    pub(crate) fn plain(damage: i32) -> Self {
        Self {
            damage,
            pierce: 0,
            bounces: 0,
            mutation: None,
            last_hit: None,
        }
    }

    pub(crate) fn mutated(damage: i32, mutation: Option<Mutation>) -> Self {
        Self {
            pierce: u32::from(mutation == Some(Mutation::Piercing)),
            bounces: u32::from(mutation == Some(Mutation::Ricochet)),
            mutation,
            ..Self::plain(damage)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Expiring;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Health {
    pub(crate) current: i32,
    pub(crate) max: i32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Armor {
    pub(crate) current: i32,
    pub(crate) max: i32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Invulnerable {
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ContactDamage(pub(crate) i32);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Trap {
    pub(crate) from_frame: usize,
    pub(crate) until_frame: usize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Wallet {
    pub(crate) coins: i32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Enemy {
    pub(crate) death: Clip,
    pub(crate) kind: crate::enemies::EnemyKind,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Spawning {
    pub(crate) remaining: f32,
    pub(crate) total: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct HitFlash {
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Dash {
    pub(crate) base_speed: f32,
    pub(crate) bonus_speed: f32,
    pub(crate) cycle: f32,
    pub(crate) trail: f32,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Path {
    pub(crate) cells: std::collections::VecDeque<Vec2>,
    pub(crate) planned_for: Option<Vec2>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Chase {
    pub(crate) speed: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Progress {
    pub(crate) anchor: Vec2,
    pub(crate) elapsed: f32,
    pub(crate) turn: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Detour {
    pub(crate) remaining: f32,
    pub(crate) turn: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Burst {
    pub(crate) left: u32,
    pub(crate) timer: f32,
    pub(crate) aim: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Weapon {
    pub(crate) kind: GunKind,
    pub(crate) cooldown: f32,
    pub(crate) clip: i32,
    pub(crate) ammo: i32,
    pub(crate) reloading: Option<f32>,
    pub(crate) burst: Option<Burst>,
    pub(crate) ammo_scale: f32,
    pub(crate) mods: GunMods,
    pub(crate) shots: u32,
}

impl Weapon {
    pub(crate) fn new(kind: GunKind) -> Self {
        let spec = kind.spec();
        Self {
            kind,
            cooldown: 0.0,
            clip: spec.clip_size,
            ammo: spec.max_ammo,
            reloading: None,
            burst: None,
            ammo_scale: 1.0,
            mods: GunMods::default(),
            shots: 0,
        }
    }

    /// Puts on the bonuses of the weapon tree; a full clip stays full when it grows.
    pub(crate) fn upgrade(&mut self, mods: GunMods) {
        let was_full = self.clip >= self.clip_size();
        self.mods = mods;
        self.clip = if was_full {
            self.clip_size()
        } else {
            self.clip.min(self.clip_size())
        };
    }

    /// Counts a pulled trigger and says whether Deadeye makes it a critical hit.
    pub(crate) fn count_shot(&mut self) -> bool {
        self.shots = self.shots.wrapping_add(1);
        self.spec().mutation == Some(Mutation::Deadeye)
            && self
                .shots
                .is_multiple_of(crate::weapon_upgrades::DEADEYE_EVERY)
    }

    pub(crate) fn scaled(kind: GunKind, ammo_scale: f32) -> Self {
        let mut weapon = Self::new(kind);
        weapon.ammo_scale = ammo_scale;
        weapon.ammo = weapon.max_ammo();
        weapon
    }

    pub(crate) fn max_ammo(&self) -> i32 {
        (self.spec().max_ammo as f32 * self.ammo_scale) as i32
    }

    #[cfg(test)]
    pub(crate) fn pistol() -> Self {
        Self::new(GunKind::Pistol)
    }

    pub(crate) fn spec(&self) -> GunSpec {
        self.mods.apply(self.kind.spec())
    }

    pub(crate) fn clip_size(&self) -> i32 {
        self.spec().clip_size
    }

    #[cfg(test)]
    pub(crate) fn can_fire(&self) -> bool {
        self.reloading.is_none() && self.cooldown <= 0.0 && self.clip > 0
    }

    pub(crate) fn has_full_ammo(&self) -> bool {
        self.ammo >= self.max_ammo()
    }

    pub(crate) fn add_percent(&mut self, percent: f32) {
        let max = self.max_ammo();
        self.ammo = (self.ammo + (max as f32 * percent) as i32).min(max);
    }

    pub(crate) fn start_reload(&mut self) -> bool {
        let can = self.burst.is_none()
            && self.reloading.is_none()
            && self.ammo > 0
            && self.clip < self.clip_size();
        if can {
            self.reloading = Some(self.spec().reload_time);
        }
        can
    }

    pub(crate) fn advance(&mut self, dt: f32) {
        self.cooldown = (self.cooldown - dt).max(0.0);
        if let Some(remaining) = self.reloading {
            let remaining = remaining - dt;
            if remaining > 0.0 {
                self.reloading = Some(remaining);
            } else {
                self.reloading = None;
                let missing = self.clip_size() - self.clip;
                let loaded = missing.min(self.ammo);
                self.clip += loaded;
                self.ammo -= loaded;
            }
        }
    }
}

const STARTING_SLOTS: usize = 2;

#[derive(Debug, Clone)]
pub(crate) struct Loadout {
    slots: Vec<Option<Weapon>>,
    current: usize,
    ammo_scale: f32,
    mods: WeaponMods,
}

impl Loadout {
    pub(crate) fn starting() -> Self {
        Self {
            slots: vec![
                Some(Weapon::new(GunKind::Pistol)),
                Some(Weapon::new(GunKind::Shotgun)),
            ],
            current: 0,
            ammo_scale: 1.0,
            mods: WeaponMods::default(),
        }
    }

    pub(crate) fn apply_weapon_mods(&mut self, mods: WeaponMods) {
        self.mods = mods;
        for weapon in self.slots.iter_mut().flatten() {
            weapon.upgrade(mods.of(weapon.kind));
        }
    }

    pub(crate) fn apply_perks(&mut self, extra_slots: usize, ammo_scale: f32) {
        while self.slots.len() < STARTING_SLOTS + extra_slots {
            self.slots.push(None);
        }
        self.ammo_scale = ammo_scale;
        for weapon in self.slots.iter_mut().flatten() {
            weapon.ammo_scale = ammo_scale;
        }
    }

    pub(crate) fn fresh(&self, kind: GunKind) -> Weapon {
        let mut weapon = Weapon::scaled(kind, self.ammo_scale);
        weapon.upgrade(self.mods.of(kind));
        weapon
    }

    pub(crate) fn current(&self) -> Option<&Weapon> {
        self.slots.get(self.current).and_then(Option::as_ref)
    }

    pub(crate) fn current_mut(&mut self) -> Option<&mut Weapon> {
        self.slots.get_mut(self.current).and_then(Option::as_mut)
    }

    #[cfg(test)]
    pub(crate) fn current_slot(&self) -> usize {
        self.current
    }

    fn takes(weapon: &Weapon, ammo: AmmoBox) -> bool {
        match ammo {
            AmmoBox::Universal => true,
            AmmoBox::Shotgun => weapon.kind.ammo_class() == AmmoClass::Shotgun,
            AmmoBox::Pistol | AmmoBox::SubMachine => weapon.kind.ammo_class() == AmmoClass::Pistol,
        }
    }

    pub(crate) fn wants_ammo(&self, ammo: AmmoBox) -> bool {
        self.slots
            .iter()
            .flatten()
            .any(|weapon| Self::takes(weapon, ammo) && !weapon.has_full_ammo())
    }

    pub(crate) fn add_ammo(&mut self, ammo: AmmoBox) {
        for weapon in self.slots.iter_mut().flatten() {
            if Self::takes(weapon, ammo) {
                weapon.add_percent(ammo.percent());
            }
        }
    }

    pub(crate) fn has(&self, kind: GunKind) -> bool {
        self.slots
            .iter()
            .flatten()
            .any(|weapon| weapon.kind == kind)
    }

    pub(crate) fn switch_to(&mut self, slot: usize) -> bool {
        if slot == self.current || slot >= self.slots.len() {
            return false;
        }
        self.current = slot;
        true
    }

    pub(crate) fn next(&mut self) {
        self.current = (self.current + 1) % self.slots.len();
    }

    pub(crate) fn previous(&mut self) {
        self.current = (self.current + self.slots.len() - 1) % self.slots.len();
    }

    pub(crate) fn drop_current(&mut self) -> Option<Weapon> {
        self.slots.get_mut(self.current).and_then(Option::take)
    }

    pub(crate) fn add(&mut self, mut weapon: Weapon) -> Option<Weapon> {
        weapon.ammo_scale = self.ammo_scale;
        weapon.upgrade(self.mods.of(weapon.kind));
        let displaced = if self.slots.iter().all(Option::is_some) {
            self.drop_current()
        } else {
            None
        };
        if self.slots[self.current].is_none() {
            self.slots[self.current] = Some(weapon);
        } else if let Some(free) = self.slots.iter().position(Option::is_none) {
            self.slots[free] = Some(weapon);
        }
        displaced
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct GunDrop {
    pub(crate) weapon: Weapon,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reload_refills_the_clip_from_the_reserve_after_the_reload_time() {
        let mut weapon = Weapon::pistol();
        weapon.clip = 2;
        weapon.ammo = 10;
        assert!(weapon.start_reload());
        assert!(!weapon.can_fire());

        weapon.advance(0.5);
        assert_eq!(weapon.clip, 2);
        weapon.advance(0.3);
        assert_eq!((weapon.clip, weapon.ammo), (7, 5));
        assert!(weapon.reloading.is_none());
    }

    #[test]
    fn reload_uses_whatever_reserve_is_left() {
        let mut weapon = Weapon::pistol();
        weapon.clip = 0;
        weapon.ammo = 3;
        weapon.start_reload();
        weapon.advance(1.0);
        assert_eq!((weapon.clip, weapon.ammo), (3, 0));
    }

    #[test]
    fn nothing_happens_with_a_full_clip_or_no_reserve() {
        let mut full = Weapon::pistol();
        assert!(!full.start_reload());
        assert!(full.reloading.is_none());

        let mut dry = Weapon::pistol();
        dry.clip = 0;
        dry.ammo = 0;
        assert!(!dry.start_reload());
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
        assert!(!m4.start_reload());
        m4.burst = None;
        assert!(m4.start_reload());
    }

    #[test]
    fn pistol_ammo_does_not_feed_the_shotgun_and_universal_feeds_everything() {
        let mut loadout = Loadout::starting();
        for weapon in loadout.slots.iter_mut().flatten() {
            weapon.ammo = 0;
        }
        loadout.add_ammo(AmmoBox::Pistol);
        let ammo: Vec<i32> = loadout.slots.iter().flatten().map(|w| w.ammo).collect();
        assert_eq!(ammo, vec![12, 0]);

        loadout.add_ammo(AmmoBox::Universal);
        let ammo: Vec<i32> = loadout.slots.iter().flatten().map(|w| w.ammo).collect();
        assert_eq!(ammo, vec![72, 24]);
    }

    #[test]
    fn full_guns_do_not_want_ammo_and_ammo_never_exceeds_the_maximum() {
        let mut loadout = Loadout::starting();
        assert!(!loadout.wants_ammo(AmmoBox::Universal));
        if let Some(weapon) = loadout.current_mut() {
            weapon.ammo = 118;
        }
        assert!(loadout.wants_ammo(AmmoBox::SubMachine));
        assert!(!loadout.wants_ammo(AmmoBox::Shotgun));
        loadout.add_ammo(AmmoBox::Universal);
        assert_eq!(loadout.current().map(|w| w.ammo), Some(120));
    }

    #[test]
    fn upgrades_add_a_slot_and_raise_the_ammo_capacity_of_every_gun() {
        let mut loadout = Loadout::starting();
        loadout.apply_perks(1, 1.1);
        assert!(loadout.switch_to(2));
        let fresh = loadout.fresh(GunKind::Uzi);
        assert_eq!((fresh.max_ammo(), fresh.ammo), (220, 220));
        assert_eq!(loadout.add(fresh).map(|w| w.kind), None);
        assert!(loadout.has(GunKind::Uzi));
        let pistol = loadout.slots[0].map(|w| (w.max_ammo(), w.ammo));
        assert_eq!(pistol, Some((132, 120)));
        assert!(!loadout.slots[0].is_some_and(|w| w.has_full_ammo()));
    }

    #[test]
    fn the_loadout_starts_with_a_pistol_and_a_shotgun() {
        let loadout = Loadout::starting();
        assert_eq!(loadout.current().map(|w| w.kind), Some(GunKind::Pistol));
        assert!(loadout.has(GunKind::Shotgun));
        assert!(!loadout.has(GunKind::Uzi));
    }

    #[test]
    fn switching_slots_ignores_the_same_or_missing_slot() {
        let mut loadout = Loadout::starting();
        assert!(!loadout.switch_to(0));
        assert!(!loadout.switch_to(5));
        assert!(loadout.switch_to(1));
        assert_eq!(loadout.current().map(|w| w.kind), Some(GunKind::Shotgun));
        loadout.next();
        assert_eq!(loadout.current_slot(), 0);
        loadout.previous();
        assert_eq!(loadout.current_slot(), 1);
    }

    #[test]
    fn dropping_empties_the_slot_and_the_next_gun_fills_it() {
        let mut loadout = Loadout::starting();
        let dropped = loadout.drop_current();
        assert_eq!(dropped.map(|w| w.kind), Some(GunKind::Pistol));
        assert!(loadout.current().is_none());

        let displaced = loadout.add(Weapon::new(GunKind::Uzi));
        assert!(displaced.is_none());
        assert_eq!(loadout.current().map(|w| w.kind), Some(GunKind::Uzi));
    }

    #[test]
    fn picking_up_with_full_slots_swaps_out_the_current_gun() {
        let mut loadout = Loadout::starting();
        loadout.switch_to(1);
        let displaced = loadout.add(Weapon::new(GunKind::M4));
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
        let displaced = loadout.add(Weapon::new(GunKind::Luger));
        assert!(displaced.is_none());
        assert_eq!(loadout.current().map(|w| w.kind), Some(GunKind::Pistol));
        loadout.switch_to(1);
        assert_eq!(loadout.current().map(|w| w.kind), Some(GunKind::Luger));
    }
}
