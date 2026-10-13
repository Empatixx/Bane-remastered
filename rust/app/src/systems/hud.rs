use glam::{Vec2, Vec3, Vec4};
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::assets::tex;
use crate::components::{
    AmmoPip, AmmoText, Animation, Armor, BarKind, Boss, CoinText, GoldCount, GunIcon, Health,
    Hidden, Loadout, PIP_EMPTY, PIP_FULL, Player, Position, Size, Sprite, StatusBar, TalentCount,
    Text, Wallet, Weapon,
};
use crate::guns::{ClipStyle, PIP_SIZE, pip_layout};
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

pub(crate) fn update_weapon_hud(world: &mut World, textures: &Textures) {
    let weapon = world
        .query::<&Loadout>()
        .with::<&Player>()
        .iter()
        .next()
        .and_then(|loadout| loadout.current().copied());
    let coins = world
        .query::<&Wallet>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|wallet| wallet.coins);
    show_weapon(world, weapon, textures);
    if let Some(coins) = coins {
        for text in world.query_mut::<&mut Text>().with::<&CoinText>() {
            text.content = coins.to_string();
        }
    }
}

pub(crate) fn update_lobby_counters(world: &mut World, talents: u64, gold: u64) {
    for text in world.query_mut::<&mut Text>().with::<&TalentCount>() {
        text.content = talents.to_string();
    }
    for text in world.query_mut::<&mut Text>().with::<&GoldCount>() {
        text.content = gold.to_string();
    }
}

fn show_weapon(world: &mut World, weapon: Option<Weapon>, textures: &Textures) {
    let icons: Vec<Entity> = world.query::<Entity>().with::<&GunIcon>().iter().collect();
    for text in world.query_mut::<&mut Text>().with::<&AmmoText>() {
        text.content = weapon.map_or_else(String::new, |w| format!("{}/{}", w.clip, w.ammo));
    }
    for icon in icons {
        match weapon {
            Some(weapon) => {
                let spec = weapon.spec();
                let _ = world.remove_one::<Hidden>(icon);
                if let Ok((sprite, size)) = world.query_one_mut::<(&mut Sprite, &mut Size)>(icon) {
                    sprite.texture = textures.get(spec.texture);
                    size.0 = spec.icon;
                }
            }
            None => {
                let _ = world.insert_one(icon, Hidden);
            }
        }
    }
    let layout = weapon.map(|w| {
        let spec = w.spec();
        (
            spec.clip_style,
            pip_layout(spec.clip_style, spec.clip_size as usize),
        )
    });
    let mut hide = Vec::new();
    let mut show = Vec::new();
    for (entity, pip, position, sprite, animation) in
        world.query_mut::<(Entity, &AmmoPip, &mut Position, &mut Sprite, &mut Animation)>()
    {
        let placed = weapon
            .zip(layout.as_ref())
            .and_then(|(w, (style, points))| {
                points
                    .get(pip.index as usize)
                    .map(|top_left| (w, *style, *top_left))
            });
        match placed {
            Some((w, style, top_left)) => {
                position.0 = top_left + PIP_SIZE / 2.0;
                sprite.texture = textures.get(match style {
                    ClipStyle::Shotgun => tex::SHOTGUN_AMMO_HUD,
                    _ => tex::AMMO_HUD,
                });
                animation.switch_to(if pip.index < w.clip {
                    PIP_FULL
                } else {
                    PIP_EMPTY
                });
                show.push(entity);
            }
            None => hide.push(entity),
        }
    }
    for entity in hide {
        let _ = world.insert_one(entity, Hidden);
    }
    for entity in show {
        let _ = world.remove_one::<Hidden>(entity);
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
