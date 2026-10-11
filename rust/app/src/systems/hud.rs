use glam::{Vec2, Vec3, Vec4};
use hecs::World;

use crate::components::{
    AmmoPip, AmmoText, Animation, Armor, BarKind, Boss, CoinText, Health, Player, StatusBar, Text,
    Wallet, Weapon, PIP_EMPTY, PIP_FULL,
};
use crate::render::{Draw, Quad, TextureId};

const DELAY_TWEEN: f32 = 0.05;
#[allow(clippy::approx_constant)]
const STRIPES: [(f32, f32, Vec3); 3] = [
    (0.0, 0.5, Vec3::ZERO),
    (0.5, 0.25, Vec3::new(0.149, 0.068, 0.104)),
    (0.75, 0.25, Vec3::new(0.318, 0.168, 0.161)),
];

pub(crate) fn update_status_bars(world: &mut World, dt: f32) {
    let health = world
        .query::<&Health>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|health| ratio(health.current, health.max));
    let armor = world
        .query::<&Armor>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|armor| ratio(armor.current, armor.max));
    let boss = world
        .query::<&Health>()
        .with::<&Boss>()
        .iter()
        .next()
        .map(|health| ratio(health.current, health.max));
    for bar in world.query_mut::<&mut StatusBar>() {
        let value = match bar.kind {
            BarKind::Health => health,
            BarKind::Armor => armor,
            BarKind::Boss => boss,
        };
        if let Some(value) = value {
            bar.fill = value;
            bar.delayed = (bar.delayed - value) * DELAY_TWEEN.powf(dt) + value;
        }
    }
}

pub(crate) fn update_weapon_hud(world: &mut World) {
    let weapon = world
        .query::<&Weapon>()
        .with::<&Player>()
        .iter()
        .next()
        .copied();
    let coins = world
        .query::<&Wallet>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|wallet| wallet.coins);
    if let Some(weapon) = weapon {
        for text in world.query_mut::<&mut Text>().with::<&AmmoText>() {
            text.content = format!("{}/{}", weapon.clip, weapon.ammo);
        }
        for (pip, animation) in world.query_mut::<(&AmmoPip, &mut Animation)>() {
            animation.switch_to(if pip.index < weapon.clip {
                PIP_FULL
            } else {
                PIP_EMPTY
            });
        }
    }
    if let Some(coins) = coins {
        for text in world.query_mut::<&mut Text>().with::<&CoinText>() {
            text.content = coins.to_string();
        }
    }
}

fn ratio(current: i32, max: i32) -> f32 {
    if max == 0 {
        1.0
    } else {
        current as f32 / max as f32
    }
}

pub(crate) fn bar_draws(center: Vec2, size: Vec2, bar: &StatusBar) -> Vec<Draw> {
    let spec = bar.kind.spec();
    let frame_top_left = center - size / 2.0;
    let area_top_left =
        frame_top_left + Vec2::new(spec.hole[0] as f32, spec.hole[1] as f32) * spec.scale;
    let area = Vec2::new(spec.hole[2] as f32, spec.hole[3] as f32) * spec.scale;

    let low = bar.fill.min(bar.delayed).clamp(0.0, 1.0);
    let high = bar.fill.max(bar.delayed).clamp(0.0, 1.0);
    let decreasing = bar.delayed > bar.fill;
    let trail = if decreasing {
        Vec3::new(0.9, spec.color.y, spec.color.z)
    } else {
        Vec3::new(spec.color.x, 0.9, spec.color.z)
    };

    let mut draws = Vec::new();
    for (from, to, color) in [(0.0, low, spec.color), (low, high, trail)] {
        if to <= from {
            continue;
        }
        for (start, height, add) in STRIPES {
            let top_left = area_top_left + Vec2::new(area.x * from, area.y * start);
            let stripe = Vec2::new(area.x * (to - from), area.y * height);
            let rgb = (color + add).min(Vec3::ONE);
            draws.push(Draw {
                texture: TextureId::WHITE,
                quad: Quad::new(
                    top_left + stripe / 2.0,
                    stripe,
                    Vec4::new(rgb.x, rgb.y, rgb.z, 1.0),
                ),
            });
        }
    }
    draws
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bar(fill: f32, delayed: f32) -> StatusBar {
        StatusBar {
            kind: BarKind::Health,
            fill,
            delayed,
        }
    }

    #[test]
    fn full_bar_draws_three_stripes_across_the_whole_area() {
        let draws = bar_draws(
            Vec2::new(200.0, 100.0),
            Vec2::new(228.0, 57.0),
            &bar(1.0, 1.0),
        );
        assert_eq!(draws.len(), 3);
    }

    #[test]
    fn a_trailing_part_adds_three_more_stripes() {
        let draws = bar_draws(
            Vec2::new(200.0, 100.0),
            Vec2::new(228.0, 57.0),
            &bar(0.4, 0.7),
        );
        assert_eq!(draws.len(), 6);
    }

    #[test]
    fn empty_bar_draws_nothing() {
        let draws = bar_draws(
            Vec2::new(200.0, 100.0),
            Vec2::new(228.0, 57.0),
            &bar(0.0, 0.0),
        );
        assert!(draws.is_empty());
    }
}
