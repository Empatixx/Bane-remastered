use glam::Vec2;

use crate::components::BreakKind;

pub(crate) const OBJECT_SIZE: Vec2 = Vec2::new(192.0, 192.0);
pub(crate) const OBJECT_HEALTH: i32 = 4;
pub(crate) const EXPLOSION_DAMAGE: i32 = 3;
pub(crate) const EXPLOSION_REACH: Vec2 = Vec2::new(128.0, 96.0);
pub(crate) const POOL_SIZE: Vec2 = Vec2::new(256.0, 128.0);
pub(crate) const POOL_HITBOX: Vec2 = Vec2::new(210.0, 90.0);
pub(crate) const POOL_SECONDS: f32 = 8.0;
pub(crate) const POOL_FADE_SECONDS: f32 = 1.0;
pub(crate) const POISON_TICK_SECONDS: f32 = 1.0;
pub(crate) const POISON_DAMAGE: i32 = 1;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Collider {
    pub(crate) offset: Vec2,
    pub(crate) size: Vec2,
}

impl BreakKind {
    pub(crate) fn collider(self) -> Collider {
        match self {
            Self::Pot => Collider {
                offset: Vec2::new(75.0, 78.0),
                size: Vec2::new(45.0, 45.0),
            },
            Self::Barrel | Self::Explosive | Self::Poison | Self::Frost => Collider {
                offset: Vec2::new(66.0, 66.0),
                size: Vec2::new(57.0, 63.0),
            },
        }
    }

    pub(crate) fn texture(self) -> &'static str {
        match self {
            Self::Pot => "ingame/pot.tga",
            Self::Barrel => "ingame/barrel.tga",
            Self::Explosive => "ingame/explosive_barrel.tga",
            Self::Poison => "ingame/poison_barrel.tga",
            Self::Frost => "ingame/frost_barrel.tga",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colliders_sit_near_the_middle_of_the_sprite() {
        for kind in [
            BreakKind::Pot,
            BreakKind::Barrel,
            BreakKind::Explosive,
            BreakKind::Poison,
            BreakKind::Frost,
        ] {
            let collider = kind.collider();
            let center = collider.offset + collider.size / 2.0;
            assert!(
                (center - OBJECT_SIZE / 2.0).abs().max_element() < 10.0,
                "{kind:?}"
            );
        }
    }

    #[test]
    fn each_kind_has_its_own_texture() {
        assert_ne!(BreakKind::Pot.texture(), BreakKind::Barrel.texture());
        assert_ne!(BreakKind::Barrel.texture(), BreakKind::Explosive.texture());
        assert_ne!(BreakKind::Poison.texture(), BreakKind::Frost.texture());
        assert_ne!(BreakKind::Poison.texture(), BreakKind::Barrel.texture());
    }
}
