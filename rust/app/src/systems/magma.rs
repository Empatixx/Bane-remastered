use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::components::{
    Color, Dodge, Health, HoundBoss, Layer, Light, Liquid, Magma, MagmaSpreader, Size,
};
use crate::dungeon::Surfaces;
use crate::enemies;
use crate::navigation::Navigation;
use crate::prefabs;
use crate::props::{Look, Prop};
use crate::systems::damage::hurt_player_through_armor;
use crate::systems::player::player_body;
use crate::systems::hostile::Hostile;
use crate::tilemap::TILE_SIZE;

const COLD: f32 = 0.32;
const COOL_DOWN: f32 = 1.2;

pub(crate) type TileRect = (usize, usize, usize, usize);

fn walkable(navigation: &Navigation, column: usize, row: usize) -> bool {
    let centre = (Vec2::new(column as f32, row as f32) + 0.5) * TILE_SIZE;
    !navigation.is_blocked(centre, Vec2::splat(TILE_SIZE * 0.6))
}

pub(crate) fn wall_distance(
    navigation: &Navigation,
    column: usize,
    row: usize,
    reach: usize,
) -> usize {
    for distance in 1..=reach {
        for dy in -(distance as isize)..=(distance as isize) {
            for dx in -(distance as isize)..=(distance as isize) {
                if dx.unsigned_abs().max(dy.unsigned_abs()) != distance {
                    continue;
                }
                let (c, r) = (column as isize + dx, row as isize + dy);
                if c < 0 || r < 0 || !walkable(navigation, c as usize, r as usize) {
                    return distance;
                }
            }
        }
    }
    reach + 1
}

pub(crate) fn plan(navigation: &Navigation, room: TileRect, seed: &mut u32) -> Vec<(usize, usize)> {
    let (origin_c, origin_r, columns, rows) = room;
    let mut tiles = Vec::new();
    for row in origin_r..origin_r + rows {
        for column in origin_c..origin_c + columns {
            if !walkable(navigation, column, row) {
                continue;
            }
            let distance = wall_distance(navigation, column, row, enemies::HOUND_MAGMA_DEPTH + 1);
            if distance <= enemies::HOUND_MAGMA_DEPTH {
                tiles.push((
                    distance,
                    (crate::rng::roll(seed) * 1000.0) as u32,
                    (column, row),
                ));
            }
        }
    }
    tiles.sort_by_key(|(distance, noise, _)| (*distance, *noise));
    tiles.into_iter().map(|(_, _, tile)| tile).collect()
}

pub(crate) fn tick_magma_spread(
    world: &mut World,
    hostile: &mut Hostile<'_>,
    room: Option<TileRect>,
) {
    let navigation = hostile.navigation;
    let textures = hostile.textures;
    let dt = hostile.dt;
    let Some(room) = room else {
        return;
    };
    let spreaders: Vec<(Entity, bool)> = world
        .query::<(Entity, &HoundBoss, &Health, &MagmaSpreader)>()
        .iter()
        .filter(|(_, _, health, _)| health.current > 0)
        .map(|(entity, hound, _, _)| (entity, hound.enraged))
        .collect();
    for (entity, enraged) in spreaders {
        let tile = {
            let Ok(mut spreader) = world.get::<&mut MagmaSpreader>(entity) else {
                continue;
            };
            if spreader.order.is_empty() && spreader.timer == 0.0 {
                spreader.timer = enemies::HOUND_MAGMA_START;
            }
            spreader.timer -= dt;
            if spreader.timer > 0.0 {
                continue;
            }
            if spreader.order.is_empty() {
                let mut seed = 0x1234_5679 ^ (room.0 as u32) << 7 ^ room.1 as u32;
                spreader.order = plan(navigation, room, &mut seed);
                if spreader.order.is_empty() {
                    spreader.timer = 1.0e9;
                    continue;
                }
            }
            spreader.timer += if enraged {
                enemies::HOUND_MAGMA_RAGE_EVERY
            } else {
                enemies::HOUND_MAGMA_EVERY
            };
            let tile = spreader.order[spreader.cursor % spreader.order.len()];
            spreader.cursor += 1;
            tile
        };
        let taken = world
            .query::<&Magma>()
            .iter()
            .any(|magma| magma.tile == tile);
        if !taken {
            spawn_magma(world, tile, textures);
        }
    }
}

pub(crate) fn spawn_magma(world: &mut World, tile: (usize, usize), textures: &Textures) -> Entity {
    let top_left = Vec2::new(tile.0 as f32, tile.1 as f32) * TILE_SIZE;
    let entity = prefabs::prop(
        world,
        Prop::LavaSurface,
        top_left,
        Look::row(Surfaces::interior_row(tile), false),
        textures,
    );
    let _ = world.insert(
        entity,
        (
            Layer(-2),
            Liquid {
                kind: 3.0,
                time: 0.0,
            },
            Magma { age: 0.0, tile },
            Color(Vec4::new(COLD, COLD, COLD, 1.0)),
            Light::rgb(Vec2::ZERO, 255.0, 110.0, 20.0, 70.0, 0.0),
        ),
    );
    entity
}

pub(crate) fn heat(magma: &Magma) -> f32 {
    (magma.age / enemies::HOUND_MAGMA_WARM).clamp(0.0, 1.0)
}

pub(crate) fn cooling(magma: &Magma) -> f32 {
    ((enemies::HOUND_MAGMA_LIFE - magma.age) / COOL_DOWN).clamp(0.0, 1.0)
}

pub(crate) fn burns(magma: &Magma) -> bool {
    heat(magma) >= 1.0 && cooling(magma) > 0.0
}

pub(crate) fn tick_magma(world: &mut World, hostile: &mut Hostile<'_>) {
    let sounds = &mut *hostile.sounds;
    let fx = &mut *hostile.fx;
    let dt = hostile.dt;
    let mut gone = Vec::new();
    for (entity, magma, color, light, size) in
        world.query_mut::<(Entity, &mut Magma, &mut Color, &mut Light, &Size)>()
    {
        magma.age += dt;
        let glow = heat(magma).min(cooling(magma));
        let level = COLD + (1.0 - COLD) * glow;
        color.0 = Vec4::new(level, level, level, 1.0);
        light.intensity = 0.9 * glow;
        let _ = size;
        if magma.age >= enemies::HOUND_MAGMA_LIFE {
            gone.push(entity);
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
    let Some((player, at, size)) = player_body(world) else {
        return;
    };
    if world.get::<&Dodge>(player).is_ok() {
        return;
    }
    let feet = Vec2::new(at.x, at.y + size.y * 0.3);
    let standing = world.query::<&Magma>().iter().any(|magma| {
        burns(magma)
            && (feet.x / TILE_SIZE).floor() as usize == magma.tile.0
            && (feet.y / TILE_SIZE).floor() as usize == magma.tile.1
    });
    if standing {
        hurt_player_through_armor(world, player, enemies::HOUND_MAGMA_DAMAGE, sounds, fx);
    }
}

pub(crate) fn clear_magma(world: &mut World) {
    let tiles: Vec<Entity> = world.query::<Entity>().with::<&Magma>().iter().collect();
    for tile in tiles {
        let _ = world.despawn(tile);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn room_navigation() -> Navigation {
        let mut solid = Vec::new();
        for column in 0..12 {
            for row in 0..12 {
                if column == 0 || row == 0 || column == 11 || row == 11 {
                    solid.push((column, row));
                }
            }
        }
        Navigation::new(&solid)
    }

    #[test]
    fn the_plan_starts_at_the_walls_and_stays_shallow() {
        let navigation = room_navigation();
        let mut seed = 5;
        let order = plan(&navigation, (0, 0, 12, 12), &mut seed);
        assert!(!order.is_empty());
        let depth = |tile: &(usize, usize)| wall_distance(&navigation, tile.0, tile.1, 8);
        assert_eq!(depth(&order[0]), 1);
        assert!(
            order
                .iter()
                .all(|tile| depth(tile) <= enemies::HOUND_MAGMA_DEPTH)
        );
        assert!(
            order
                .windows(2)
                .all(|pair| depth(&pair[0]) <= depth(&pair[1]))
        );
        assert!(!order.contains(&(5, 5)));
    }

    #[test]
    fn magma_warms_burns_and_then_cools() {
        let mut magma = Magma {
            age: 0.0,
            tile: (1, 1),
        };
        assert!(!burns(&magma));
        magma.age = enemies::HOUND_MAGMA_WARM + 0.1;
        assert!(burns(&magma));
        magma.age = enemies::HOUND_MAGMA_LIFE - 0.1;
        assert!(cooling(&magma) < 0.2);
        magma.age = enemies::HOUND_MAGMA_LIFE;
        assert!(!burns(&magma));
    }
}
