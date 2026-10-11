use super::animation::Clip;

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
    pub(crate) active_frames: usize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Wallet {
    pub(crate) coins: i32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Enemy {
    pub(crate) death: Clip,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Chase {
    pub(crate) speed: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Weapon {
    pub(crate) fire_interval: f32,
    pub(crate) cooldown: f32,
    pub(crate) inaccuracy: f32,
    pub(crate) min_damage: i32,
    pub(crate) max_damage: i32,
    pub(crate) bullet_speed: f32,
    pub(crate) clip_size: i32,
    pub(crate) clip: i32,
    pub(crate) ammo: i32,
    pub(crate) reload_time: f32,
    pub(crate) reloading: Option<f32>,
}

impl Weapon {
    pub(crate) fn pistol() -> Self {
        Self {
            fire_interval: 0.25,
            cooldown: 0.0,
            inaccuracy: 0.1,
            min_damage: 1,
            max_damage: 3,
            bullet_speed: 700.0,
            clip_size: 7,
            clip: 7,
            ammo: 120,
            reload_time: 0.7,
            reloading: None,
        }
    }

    pub(crate) fn can_fire(&self) -> bool {
        self.reloading.is_none() && self.cooldown <= 0.0 && self.clip > 0
    }

    pub(crate) fn start_reload(&mut self) {
        if self.reloading.is_none() && self.ammo > 0 && self.clip < self.clip_size {
            self.reloading = Some(self.reload_time);
        }
    }

    pub(crate) fn advance(&mut self, dt: f32) {
        self.cooldown = (self.cooldown - dt).max(0.0);
        if let Some(remaining) = self.reloading {
            let remaining = remaining - dt;
            if remaining > 0.0 {
                self.reloading = Some(remaining);
            } else {
                self.reloading = None;
                let missing = self.clip_size - self.clip;
                let loaded = missing.min(self.ammo);
                self.clip += loaded;
                self.ammo -= loaded;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reload_refills_the_clip_from_the_reserve_after_the_reload_time() {
        let mut weapon = Weapon::pistol();
        weapon.clip = 2;
        weapon.ammo = 10;
        weapon.start_reload();
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
        full.start_reload();
        assert!(full.reloading.is_none());

        let mut dry = Weapon::pistol();
        dry.clip = 0;
        dry.ammo = 0;
        dry.start_reload();
        assert!(dry.reloading.is_none());
        assert!(!dry.can_fire());
    }
}
