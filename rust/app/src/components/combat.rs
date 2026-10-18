use glam::Vec2;

use super::animation::Clip;
use crate::guns::{ClipStyle, GunKind, GunSpec, Mutation, Round};
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
    pub(crate) round: Round,
    /// Seconds left before a short-lived round burns out in the air.
    pub(crate) lifetime: Option<f32>,
}

impl Bullet {
    pub(crate) fn plain(damage: i32) -> Self {
        Self {
            damage,
            pierce: 0,
            bounces: 0,
            mutation: None,
            last_hit: None,
            round: Round::Bullet,
            lifetime: None,
        }
    }

    /// The bullet as the given round: it keeps its mutation and adds what the round does.
    pub(crate) fn as_round(self, round: Round) -> Self {
        let spec = round.spec();
        Self {
            pierce: self.pierce.saturating_add(spec.pierce),
            round,
            lifetime: spec.lifetime,
            ..self
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

/// A one-tick tag on an enemy that was reaped on this fixed tick: its health reached zero and it
/// started its death animation. The death effects (bursts, clouds, heals, splits, ashes, the boss
/// clean-up) read it; the death phase removes it at the end of the same death phase.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Died;

/// A one-frame tag on an `Expiring` entity whose animation finished: it is about to leave the
/// world. The payouts of a finished corpse (drops, kill stats, bloodlust, midas coins) read it;
/// `systems::despawn_gone` despawns it later in the same frame.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Gone;

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
pub(crate) struct Toughened;

/// The enemy survives its first death as a puddle that re-forms after a while.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Revives;

/// A collapsed enemy waiting to re-form; it rides on `Spawning` so nothing can hit it.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Puddle;

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

/// Which of the places round the player an enemy closing in has taken.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Slot(pub(crate) usize);

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

    /// A laser gun keeps all its charge in one battery and never reloads.
    pub(crate) fn runs_on_battery(&self) -> bool {
        self.spec().clip_style == ClipStyle::Battery
    }

    pub(crate) fn has_full_ammo(&self) -> bool {
        if self.runs_on_battery() {
            self.clip >= self.clip_size()
        } else {
            self.ammo >= self.max_ammo()
        }
    }

    /// How much of its ammo the gun still has, from 0 to 1.
    pub(crate) fn fullness(&self) -> f32 {
        if self.runs_on_battery() {
            self.clip as f32 / self.clip_size().max(1) as f32
        } else {
            self.ammo as f32 / self.max_ammo().max(1) as f32
        }
    }

    /// A full clip and a full reserve, any reload called off.
    pub(crate) fn refill(&mut self) {
        self.clip = self.clip_size();
        self.ammo = self.max_ammo();
        self.reloading = None;
    }

    pub(crate) fn add_percent(&mut self, percent: f32) {
        if self.runs_on_battery() {
            let size = self.clip_size();
            self.clip = (self.clip + (size as f32 * percent) as i32).min(size);
            return;
        }
        let max = self.max_ammo();
        self.ammo = (self.ammo + (max as f32 * percent) as i32).min(max);
    }
}

pub(crate) const STARTING_SLOTS: usize = 2;

#[derive(Debug, Clone)]
pub(crate) struct Loadout {
    pub(crate) slots: Vec<Option<Weapon>>,
    pub(crate) current: usize,
    pub(crate) ammo_scale: f32,
    pub(crate) mods: WeaponMods,
    pub(crate) only: Option<GunKind>,
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
            only: None,
        }
    }

    pub(crate) fn allows(&self, kind: GunKind) -> bool {
        self.only.is_none_or(|only| only == kind)
    }

    pub(crate) fn apply_weapon_mods(&mut self, mods: WeaponMods) {
        self.mods = mods;
        for weapon in self.slots.iter_mut().flatten() {
            weapon.upgrade(mods.of(weapon.kind));
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

    pub(crate) fn current_slot(&self) -> usize {
        self.current
    }

    pub(crate) fn takes(weapon: &Weapon, ammo: AmmoBox) -> bool {
        ammo.class()
            .is_none_or(|class| weapon.kind.ammo_class() == class)
    }

    pub(crate) fn accepts_ammo(&self, ammo: AmmoBox) -> bool {
        match (self.only, ammo.class()) {
            (_, None) | (None, _) => true,
            (Some(only), Some(class)) => only.ammo_class() == class,
        }
    }

    pub(crate) fn feeds_a_held_gun(&self, ammo: AmmoBox) -> bool {
        self.slots
            .iter()
            .flatten()
            .any(|weapon| Self::takes(weapon, ammo))
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

    /// Empty hands: every slot free and the first one current.
    pub(crate) fn clear(&mut self) {
        self.slots.iter_mut().for_each(|slot| *slot = None);
        self.current = 0;
    }

    /// Every gun's reserve full again, for the training, where ammo is not the lesson.
    pub(crate) fn top_up_reserves(&mut self) {
        for weapon in self.slots.iter_mut().flatten() {
            if !weapon.runs_on_battery() {
                weapon.ammo = weapon.max_ammo();
            }
        }
    }

    /// With nothing in hand, takes up the first gun there is; whether it changed hands.
    pub(crate) fn arm(&mut self) -> bool {
        if self.slots.get(self.current).is_some_and(Option::is_some) {
            return false;
        }
        match self.slots.iter().position(Option::is_some) {
            Some(slot) => {
                self.current = slot;
                true
            }
            None => false,
        }
    }

    pub(crate) fn drop_current(&mut self) -> Option<Weapon> {
        self.slots.get_mut(self.current).and_then(Option::take)
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
}
