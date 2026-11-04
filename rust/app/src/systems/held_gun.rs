//! The hero's gun in his floating hand: hand and gun circle him towards the cursor, the gun
//! turned to the aim (mirrored aiming left so it is never upside down), behind him aiming up.
//! A shot throws it back a little. Nothing is held while dodging, dead or unarmed.

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::components::{
    Color, Died, Dodge, GunKick, Heading, HeldGun, HeldHand, Hidden, Lit, Loadout, Player,
    Position, Previous, Size, Sprite, YSort,
};
use crate::held::{HAND_SIZE, pose};
use crate::input::Input;
use crate::physics::Physics;
use crate::prefabs::PLAYER_LOOK;

/// How fast the kick of a shot settles, per second.
const KICK_SETTLE: f32 = 9.0;
/// Hand and gun sort just in front of the hero (or just behind him aiming up).
const SORT_NUDGE: f32 = 0.5;
/// Steps of the check that a muzzle is not on the far side of a wall.
const WALL_STEP: f32 = 6.0;

struct Holder {
    centre: Vec2,
    previous: Option<Vec2>,
    kind: Option<crate::guns::GunKind>,
    kick: f32,
    shown: bool,
}

fn holder(world: &mut World, dt: f32) -> Option<Holder> {
    let query = world.query::<(
        &Position,
        Option<&Previous>,
        &Loadout,
        Option<&mut GunKick>,
        Option<&Dodge>,
        Option<&Died>,
        Option<&Hidden>,
        &Color,
    )>();
    let mut query = query.with::<&Player>();
    let (position, previous, loadout, kick, dodge, died, hidden, color) = query.iter().next()?;
    let kick = kick.map_or(0.0, |kick| {
        kick.0 = (kick.0 - KICK_SETTLE * dt).max(0.0);
        kick.0
    });
    let kind = loadout.current().map(|weapon| weapon.kind);
    Some(Holder {
        centre: position.0,
        previous: previous.map(|previous| previous.0),
        kind,
        kick,
        shown: kind.is_some() && dodge.is_none() && died.is_none() && hidden.is_none() && color.0.w > 0.0,
    })
}

fn spawn(world: &mut World, textures: &Textures) -> (Entity, Entity) {
    let look = || (Color(Vec4::ONE), Lit, YSort { foot: 0.0 }, Hidden);
    let hand = world.spawn((
        HeldHand,
        Position(Vec2::ZERO),
        Size(HAND_SIZE),
        Sprite::whole(textures.get(tex::HELD_HAND)),
    ));
    let _ = world.insert(hand, look());
    let gun = world.spawn((
        HeldGun,
        Position(Vec2::ZERO),
        Size(Vec2::ONE),
        Sprite::whole(textures.get(tex::HELD_PISTOL)),
        Heading(0.0),
    ));
    let _ = world.insert(gun, look());
    (hand, gun)
}

fn parts(world: &World) -> Option<(Entity, Entity)> {
    let hand = world.query::<Entity>().with::<&HeldHand>().iter().next()?;
    let gun = world.query::<Entity>().with::<&HeldGun>().iter().next()?;
    Some((hand, gun))
}

fn place(world: &mut World, entity: Entity, at: Vec2, was: Option<Vec2>, foot: f32) {
    if let Ok(mut position) = world.get::<&mut Position>(entity) {
        position.0 = at;
    }
    if let Ok(mut ysort) = world.get::<&mut YSort>(entity) {
        ysort.foot = foot;
    }
    match was {
        Some(was) => {
            let _ = world.insert_one(entity, Previous(was));
        }
        None => {
            let _ = world.remove_one::<Previous>(entity);
        }
    }
}

pub(crate) fn hold_gun(world: &mut World, input: &Input, camera: Vec2, textures: &Textures, dt: f32) {
    let Some(holder) = holder(world, dt) else {
        return;
    };
    let (hand, gun) = parts(world).unwrap_or_else(|| spawn(world, textures));
    let Some(kind) = holder.kind.filter(|_| holder.shown) else {
        let _ = world.insert_one(hand, Hidden);
        let _ = world.insert_one(gun, Hidden);
        return;
    };
    let _ = world.remove_one::<Hidden>(hand);
    let _ = world.remove_one::<Hidden>(gun);
    let aim = input.mouse() + camera - holder.centre;
    let now = pose(holder.centre, aim, kind, holder.kick);
    let was = holder.previous.map(|previous| pose(previous, aim, kind, holder.kick));
    // Sorted by the hero's feet: just in front of him, or just behind aiming up.
    let feet = holder.centre.y + PLAYER_LOOK.y / 2.0 + if now.behind { -SORT_NUDGE } else { SORT_NUDGE };
    place(world, hand, now.hand, was.map(|p| p.hand), feet - now.hand.y);
    place(world, gun, now.gun, was.map(|p| p.gun), feet - now.gun.y + SORT_NUDGE * 0.5);
    let held = kind.held();
    if let Ok((sprite, size, heading)) = world.query_one_mut::<(&mut Sprite, &mut Size, &mut Heading)>(gun) {
        let mut look = Sprite::whole(textures.get(held.texture));
        if now.flipped {
            look.uv = [look.uv[0], look.uv[3], look.uv[2], look.uv[1]];
        }
        *sprite = look;
        size.0 = now.size;
        heading.0 = now.angle;
    }
}

/// Where a shot aimed along `aim` leaves the hero at `centre`: the muzzle of his gun, or the
/// last clear point before a wall that stands between him and the muzzle.
pub(crate) fn shot_origin(physics: &Physics, centre: Vec2, aim: Vec2, kind: crate::guns::GunKind) -> Vec2 {
    let muzzle = pose(centre, aim, kind, 0.0).muzzle;
    let solids = physics.solids_within(centre.min(muzzle), centre.max(muzzle));
    if solids.is_empty() {
        return muzzle;
    }
    let inside = |point: Vec2| {
        solids.iter().any(|(at, size)| {
            let half = *size / 2.0;
            (point.x - at.x).abs() < half.x && (point.y - at.y).abs() < half.y
        })
    };
    let length = (muzzle - centre).length();
    let steps = (length / WALL_STEP).ceil().max(1.0) as usize;
    let mut clear = centre;
    for step in 1..=steps {
        let point = centre.lerp(muzzle, step as f32 / steps as f32);
        if inside(point) {
            return clear;
        }
        clear = point;
    }
    muzzle
}

/// A shot was fired: the gun jumps back.
pub(crate) fn kick_gun(world: &mut World, player: Entity) {
    let _ = world.insert_one(player, GunKick(1.0));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guns::GunKind;

    fn armed(world: &mut World) -> Entity {
        world.spawn((Player, Position(Vec2::new(100.0, 100.0)), Loadout::starting(), Color(Vec4::ONE)))
    }

    fn shown<T: hecs::Component>(world: &World) -> bool {
        world.query::<Option<&Hidden>>().with::<&T>().iter().all(|hidden| hidden.is_none())
            && world.query::<&T>().iter().count() == 1
    }

    #[test]
    fn an_armed_hero_holds_his_gun_towards_the_cursor() {
        let mut world = World::new();
        armed(&mut world);
        let textures = Textures::default();
        let mut input = Input::default();
        input.set_mouse(Vec2::new(300.0, 100.0));
        hold_gun(&mut world, &input, Vec2::ZERO, &textures, 1.0 / 60.0);
        assert!(shown::<HeldHand>(&world) && shown::<HeldGun>(&world));
        let gun = world.query::<&Position>().with::<&HeldGun>().iter().next().map(|p| p.0).unwrap();
        assert!(gun.x > 100.0);
    }

    #[test]
    fn nothing_is_held_while_dodging() {
        let mut world = World::new();
        let player = armed(&mut world);
        let _ = world.insert_one(player, Dodge { direction: Vec2::X, remaining: 0.2, trail: 0.0 });
        hold_gun(&mut world, &Input::default(), Vec2::ZERO, &Textures::default(), 1.0 / 60.0);
        assert!(!shown::<HeldGun>(&world));
    }

    #[test]
    fn a_shot_by_a_wall_leaves_from_this_side_of_it() {
        let mut physics = Physics::new();
        // A wall a tile right of the hero, nearer than an M4's muzzle.
        physics.add_static_box(Vec2::new(130.0, 100.0), Vec2::new(20.0, 200.0));
        let origin = shot_origin(&physics, Vec2::new(100.0, 100.0), Vec2::X, GunKind::M4);
        assert!(origin.x < 120.0, "{origin:?}");
        let open = shot_origin(&Physics::new(), Vec2::new(100.0, 100.0), Vec2::X, GunKind::M4);
        assert!(open.x > 140.0, "{open:?}");
    }
}
