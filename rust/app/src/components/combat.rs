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
        }
    }
}
