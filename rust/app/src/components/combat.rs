use super::animation::Clip;

pub(crate) const BULLET_COLUMNS: usize = 4;
pub(crate) const BULLET_ROWS: usize = 2;

pub(crate) const BULLET_FLIGHT: Clip = Clip {
    row: 0,
    frames: 4,
    frame_time: 0.07,
    looping: true,
};

pub(crate) const BULLET_IMPACT: Clip = Clip {
    row: 1,
    frames: 4,
    frame_time: 0.07,
    looping: false,
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Bullet;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Impact;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Weapon {
    pub(crate) fire_interval: f32,
    pub(crate) cooldown: f32,
    pub(crate) inaccuracy: f32,
    pub(crate) bullet_speed: f32,
}

impl Weapon {
    pub(crate) fn pistol() -> Self {
        Self {
            fire_interval: 0.25,
            cooldown: 0.0,
            inaccuracy: 0.1,
            bullet_speed: 700.0,
        }
    }
}
