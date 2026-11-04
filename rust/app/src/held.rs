//! The gun in the hero's hand: each gun's small held picture (pointing right, drawn in
//! `tools/art/hand_guns.py`), where the hand grips it and where its rounds leave, and the
//! geometry that puts hand and gun round the hero towards the aim.

use glam::Vec2;

use crate::assets::{Tex, tex};
use crate::guns::GunKind;

/// Screen pixels per pixel of the hero's sheet (20x26 frames drawn 60x78).
pub(crate) const PIXEL: f32 = 3.0;
/// The hand circles this far from the chest, in sheet pixels; flatter up and down, as seen from
/// above at an angle.
const ORBIT: f32 = 6.0;
const ORBIT_SQUASH: f32 = 0.7;
/// Where the hand rests: the waist, from the centre of the hero's frame (centre row 13, waist
/// row 20), low enough that the gun clears his big head.
const CHEST: Vec2 = Vec2::new(0.0, 7.0 * PIXEL);
/// How far a shot throws the gun back, in screen pixels, at full kick.
const KICK: f32 = 6.0;
pub(crate) const HAND_SIZE: Vec2 = Vec2::new(4.0 * PIXEL, 4.0 * PIXEL);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Held {
    pub(crate) texture: Tex,
    /// Size of the picture in its own pixels.
    pub(crate) pixels: (u32, u32),
    /// The pixel the hand holds; the gun turns about it.
    pub(crate) grip: (u32, u32),
    /// The pixel at the end of the barrel, where rounds leave.
    pub(crate) muzzle: (u32, u32),
}

const fn held(texture: Tex, pixels: (u32, u32), grip: (u32, u32), muzzle: (u32, u32)) -> Held {
    Held { texture, pixels, grip, muzzle }
}

impl GunKind {
    pub(crate) fn held(self) -> Held {
        match self {
            Self::Pistol => held(tex::HELD_PISTOL, (9, 6), (2, 4), (8, 2)),
            Self::Luger => held(tex::HELD_LUGER, (11, 6), (2, 4), (10, 2)),
            Self::Revolver => held(tex::HELD_REVOLVER, (11, 6), (2, 4), (10, 2)),
            Self::Uzi => held(tex::HELD_UZI, (11, 7), (3, 4), (10, 2)),
            Self::Blaster => held(tex::HELD_BLASTER, (10, 6), (2, 4), (9, 2)),
            Self::Shotgun => held(tex::HELD_SHOTGUN, (18, 6), (5, 4), (17, 2)),
            Self::M4 => held(tex::HELD_M4, (16, 7), (4, 4), (15, 3)),
            Self::Thompson => held(tex::HELD_THOMPSON, (17, 7), (5, 4), (16, 2)),
            Self::Sniper => held(tex::HELD_SNIPER, (21, 7), (6, 4), (20, 3)),
            Self::Rpg => held(tex::HELD_RPG, (21, 7), (6, 4), (20, 2)),
            Self::Minigun => held(tex::HELD_MINIGUN, (17, 8), (4, 5), (16, 3)),
            Self::Flamethrower => held(tex::HELD_FLAMETHROWER, (17, 8), (4, 4), (16, 2)),
            Self::PlasmaRifle => held(tex::HELD_PLASMA_RIFLE, (18, 7), (4, 4), (16, 3)),
            Self::TeslaGun => held(tex::HELD_TESLA_GUN, (17, 7), (4, 4), (15, 2)),
            Self::ArcCaster => held(tex::HELD_ARC_CASTER, (17, 7), (4, 4), (16, 2)),
            Self::Railgun => held(tex::HELD_RAILGUN, (23, 7), (5, 4), (21, 3)),
        }
    }
}

/// Where hand and gun are for one aim.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Pose {
    pub(crate) hand: Vec2,
    pub(crate) gun: Vec2,
    pub(crate) size: Vec2,
    /// The gun's turn, as for [`crate::components::Heading`].
    pub(crate) angle: f32,
    /// Aiming left the picture is mirrored top to bottom so the gun is not upside down.
    pub(crate) flipped: bool,
    pub(crate) muzzle: Vec2,
    /// Aiming up, the gun is behind the hero.
    pub(crate) behind: bool,
}

/// A pixel's centre, from the picture's centre, in screen pixels, mirrored when `flipped`.
fn from_centre(held: &Held, (x, y): (u32, u32), flipped: bool) -> Vec2 {
    let (w, h) = (held.pixels.0 as f32, held.pixels.1 as f32);
    let offset = Vec2::new(x as f32 + 0.5 - w / 2.0, y as f32 + 0.5 - h / 2.0);
    let offset = if flipped { Vec2::new(offset.x, -offset.y) } else { offset };
    offset * PIXEL
}

/// Hand and gun for a hero at `centre` aiming along `aim`, the gun thrown back by `kick`
/// (0 to 1) after a shot.
pub(crate) fn pose(centre: Vec2, aim: Vec2, kind: GunKind, kick: f32) -> Pose {
    let held = kind.held();
    let direction = aim.try_normalize().unwrap_or(Vec2::X);
    let angle = direction.to_angle();
    let flipped = direction.x < 0.0;
    let turn = Vec2::from_angle(angle);
    let orbit = Vec2::new(direction.x * ORBIT, direction.y * ORBIT * ORBIT_SQUASH) * PIXEL;
    let hand = centre + CHEST + orbit - direction * KICK * kick.clamp(0.0, 1.0);
    let grip = from_centre(&held, held.grip, flipped);
    let muzzle = from_centre(&held, held.muzzle, flipped);
    Pose {
        hand,
        gun: hand - turn.rotate(grip),
        size: Vec2::new(held.pixels.0 as f32, held.pixels.1 as f32) * PIXEL,
        angle,
        flipped,
        muzzle: hand + turn.rotate(muzzle - grip),
        behind: direction.y < -0.38,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_held_gun_has_its_grip_and_muzzle_inside_the_picture_and_the_muzzle_ahead() {
        for kind in GunKind::ALL {
            let held = kind.held();
            for (x, y) in [held.grip, held.muzzle] {
                assert!(x < held.pixels.0 && y < held.pixels.1, "{kind:?}");
            }
            assert!(held.muzzle.0 > held.grip.0, "{kind:?}");
        }
    }

    #[test]
    fn aiming_right_puts_the_gun_and_its_muzzle_to_the_right_and_in_front() {
        let pose = pose(Vec2::ZERO, Vec2::X, GunKind::M4, 0.0);
        assert!(pose.hand.x > 0.0 && pose.gun.x > pose.hand.x && pose.muzzle.x > pose.gun.x);
        assert!(!pose.flipped && !pose.behind);
    }

    #[test]
    fn aiming_left_mirrors_the_gun_and_aiming_up_puts_it_behind() {
        let left = pose(Vec2::ZERO, -Vec2::X, GunKind::Pistol, 0.0);
        assert!(left.flipped && left.muzzle.x < left.hand.x);
        let up = pose(Vec2::ZERO, -Vec2::Y, GunKind::Pistol, 0.0);
        assert!(up.behind && up.muzzle.y < up.hand.y);
    }

    #[test]
    fn the_muzzle_lies_on_the_line_of_the_aim_through_the_grip() {
        for aim in [Vec2::X, Vec2::new(1.0, 1.0), Vec2::new(-1.0, 0.3), Vec2::new(0.2, -1.0)] {
            let pose = pose(Vec2::ZERO, aim, GunKind::Sniper, 0.0);
            let along = (pose.muzzle - pose.hand).normalize();
            let side = along.perp_dot(aim.normalize()).abs();
            // The barrel sits a pixel or two above the grip; seen from the grip it points
            // nearly the way of the aim.
            assert!(side < 0.15, "{aim:?} {side}");
        }
    }

    #[test]
    fn a_shot_throws_the_gun_back_against_the_aim() {
        let still = pose(Vec2::ZERO, Vec2::X, GunKind::Pistol, 0.0);
        let kicked = pose(Vec2::ZERO, Vec2::X, GunKind::Pistol, 1.0);
        assert!(kicked.hand.x < still.hand.x);
    }
}
