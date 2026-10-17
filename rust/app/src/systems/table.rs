//! Rugs and tables in some combat rooms. A table, lying across the room or turned lengthwise down
//! it, stands in everyone's way while bullets fly over it. The player walking (or rolling) into
//! it knocks it over to the far side (up or down for one lying across, left or right for a
//! turned one), and then it stops enemy bullets (the player's still fly over). Enemy bullets
//! chip a flipped table and enemies pushing against any table batter it, until it bursts apart
//! into scrap. Each floor has its own look of table.

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Body, Color, Enemy, Expiring, Layer, Lit, Player, Position, Size, Spawning, Sprite, Velocity,
    YSort,
};
use crate::dungeon::Dungeon;
use crate::physics::Physics;
use crate::tilemap::TILE_SIZE;

const COLUMNS: usize = 9;
/// One row of pictures per floor's look (see `Theme::style`).
const ROWS: usize = 7;
/// The frames of a table bursting apart, then the scrap it leaves.
const BURST: [usize; 4] = [6, 7, 8, BROKEN];
const BURST_FRAME: f32 = 0.09;
/// An enemy pushing against a table batters it this often.
const BATTER_EVERY: f32 = 0.5;
/// How close an enemy has to be to the table to be pushing against it.
const BATTER_REACH: f32 = 6.0;
const STANDING: usize = 0;
/// Fallen with its top facing up the screen: we see the plate, the legs stick up behind it.
const TOP_UP: usize = 1;
/// Fallen with its top facing down the screen: we see the underside and the legs pointing at us.
const UNDERSIDE: usize = 2;
const BROKEN: usize = 3;
const UPRIGHT: usize = 4;
/// A turned table fallen with its top facing left, the legs out to the right; mirrored for the
/// other way.
const FELL_SIDEWAYS: usize = 5;
/// The 32x32 picture at the tiles' scale.
const SIZE: Vec2 = Vec2::splat(TILE_SIZE * 2.0);
/// The standing table's footprint: its top and legs.
const STANDING_BODY: Vec2 = Vec2::new(116.0, 48.0);
const STANDING_OFFSET: Vec2 = Vec2::new(0.0, 4.0);
/// A flipped table is a thin wall of boards.
const COVER_BODY: Vec2 = Vec2::new(122.0, 26.0);
/// How far up the screen the underside of a table fallen away lies.
const AWAY_COVER: f32 = 25.0;
/// A table fallen towards us slides down the screen, its plate this far below the middle.
const TOWARDS_MOVE: f32 = 16.0;
const TOWARDS_COVER: f32 = 5.0;
const UPRIGHT_BODY: Vec2 = Vec2::new(56.0, 108.0);
const UPRIGHT_LIFT: f32 = 8.0;
const SIDE_COVER_BODY: Vec2 = Vec2::new(30.0, 118.0);
/// A turned table fallen sideways slides that way, its board this far from the middle.
const SIDE_MOVE: f32 = 8.0;
const SIDE_COVER: f32 = 13.0;
/// Enemy bullets a flipped table takes before it breaks.
pub(crate) const TABLE_HITS: u32 = 6;
/// How fast the player has to walk into a table to knock it over.
const SHOVE_SPEED: f32 = 40.0;
/// How close the player has to be to count as walking into it.
const SHOVE_REACH: f32 = 8.0;
const DEBRIS_LAYER: i32 = -1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TableState {
    Standing,
    Flipped,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Table {
    pub(crate) state: TableState,
    pub(crate) hits_left: u32,
    /// Turned lengthwise down the room.
    pub(crate) upright: bool,
    /// The middle of the two tiles it stands on.
    pub(crate) center: Vec2,
    /// Its floor's look: its row in the sheet.
    pub(crate) style: usize,
    /// Where its body is, and how big, standing or fallen.
    pub(crate) body: (Vec2, Vec2),
    /// Until the enemies pushing against it can batter it again.
    pub(crate) battered: f32,
}

/// What is left of a table bursting apart, flying out and settling into scrap.
#[derive(Debug, Clone, Copy)]
pub(crate) struct TableBurst {
    style: usize,
    time: f32,
}

pub(crate) fn spawn_furniture(
    world: &mut World,
    physics: &mut Physics,
    dungeon: &Dungeon,
    textures: &Textures,
) {
    let texture = textures.get(tex::TABLE);
    let style = dungeon.theme.style();
    for room in &dungeon.rooms {
        if let Some(rug) = &room.rug {
            crate::prefabs::carpet(world, rug, textures);
        }
        for spot in &room.tables {
            let (column, row) = (spot.tile.0 as f32, spot.tile.1 as f32);
            let (center, at, cell, body_size) = if spot.upright {
                let center = Vec2::new(column + 0.5, row + 1.0) * TILE_SIZE;
                (center, center - Vec2::new(0.0, UPRIGHT_LIFT), UPRIGHT, UPRIGHT_BODY)
            } else {
                let center = Vec2::new(column + 1.0, row + 0.5) * TILE_SIZE;
                (center, center - Vec2::new(0.0, SIZE.y / 4.0), STANDING, STANDING_BODY)
            };
            let offset = if spot.upright { Vec2::ZERO } else { STANDING_OFFSET };
            let table = world.spawn((
                Table {
                    state: TableState::Standing,
                    hits_left: TABLE_HITS,
                    upright: spot.upright,
                    center,
                    style,
                    body: (center + offset, body_size),
                    battered: 0.0,
                },
                Position(at),
                Size(SIZE),
                Color(Vec4::ONE),
                Sprite::cell(texture, cell, style, COLUMNS, ROWS),
                Layer(Layer::ACTORS.0),
                Lit,
                YSort {
                    foot: center.y - at.y + if spot.upright { TILE_SIZE } else { TILE_SIZE / 2.0 },
                },
            ));
            let body = physics.add_table(table, center + offset, body_size, false);
            let _ = world.insert_one(table, body);
        }
    }
}

fn standing_body(table: &Table) -> (Vec2, Vec2) {
    if table.upright {
        (table.center, UPRIGHT_BODY)
    } else {
        (table.center + STANDING_OFFSET, STANDING_BODY)
    }
}

/// Which way a table knocked by the player falls: to the far side from where the push came,
/// along the way the table can fall (up or down when it lies across the room, left or right when
/// it is turned); a push from the end it cannot fall towards tips it away from the player.
fn fall_from(table: &Table, player: Vec2, push: Vec2) -> Fall {
    if table.upright {
        let right = if push.x.abs() >= push.y.abs() {
            push.x > 0.0
        } else {
            player.x <= table.center.x
        };
        if right { Fall::Right } else { Fall::Left }
    } else {
        let up = if push.y.abs() >= push.x.abs() {
            push.y < 0.0
        } else {
            player.y >= table.center.y
        };
        if up { Fall::Away } else { Fall::Towards }
    }
}

/// Knocks over the standing table the player walks into, to its far side. `intent` is the way
/// the player is trying to walk, which counts even while the table holds them up.
pub(crate) fn shove_tables(
    world: &mut World,
    physics: &mut Physics,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    intent: Vec2,
) {
    let Some((player, player_size, push)) = world
        .query::<(&Position, &Size, &Velocity)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(position, size, velocity)| (position.0, size.0, velocity.0))
    else {
        return;
    };
    let push = if intent.length() > 0.1 {
        intent * SHOVE_SPEED * 2.0
    } else {
        push
    };
    if push.length() < SHOVE_SPEED {
        return;
    }
    let knocked = world
        .query::<(Entity, &Table)>()
        .iter()
        .filter(|(_, table)| table.state == TableState::Standing)
        .find_map(|(entity, table)| {
            let (center, size) = standing_body(table);
            let gap = center - player;
            let reach = (size + player_size) / 2.0 + Vec2::splat(SHOVE_REACH);
            let touching = gap.abs().cmple(reach).all();
            (touching && push.dot(gap) > 0.0).then_some((entity, *table))
        });
    let Some((entity, table)) = knocked else {
        return;
    };
    flip(world, physics, textures, entity, table.center, fall_from(&table, player, push));
    sounds.push(AudioCommand::Effect(SoundEffect::WallHit));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fall {
    Away,
    Towards,
    Left,
    Right,
}

fn flip(
    world: &mut World,
    physics: &mut Physics,
    textures: &Textures,
    entity: Entity,
    center: Vec2,
    fall: Fall,
) {
    if let Ok(body) = world.remove_one::<Body>(entity) {
        physics.remove(body);
    }
    // It tips over its far edge: the top ends up facing away from whoever pushed it, the legs
    // pointing back at them. `moved` slides the picture onto the far side, `shift` is where
    // its board lies from the table's middle.
    let (cell, mirrored, moved, shift, size) = match fall {
        Fall::Away => (UNDERSIDE, false, Vec2::ZERO, Vec2::new(0.0, -AWAY_COVER), COVER_BODY),
        Fall::Towards => (TOP_UP, false, Vec2::new(0.0, TOWARDS_MOVE), Vec2::new(0.0, TOWARDS_COVER), COVER_BODY),
        Fall::Right => (FELL_SIDEWAYS, true, Vec2::new(SIDE_MOVE, 0.0), Vec2::new(SIDE_COVER, 0.0), SIDE_COVER_BODY),
        Fall::Left => (FELL_SIDEWAYS, false, Vec2::new(-SIDE_MOVE, 0.0), Vec2::new(-SIDE_COVER, 0.0), SIDE_COVER_BODY),
    };
    if let Ok(mut position) = world.get::<&mut Position>(entity) {
        position.0 += moved;
    }
    let body = physics.add_table(entity, center + shift, size, true);
    let style = world.get::<&Table>(entity).map_or(0, |table| table.style);
    let mut sprite = Sprite::cell(textures.get(tex::TABLE), cell, style, COLUMNS, ROWS);
    if mirrored {
        sprite.uv = Sprite::flipped(sprite.uv);
    }
    let _ = world.insert(entity, (body, sprite));
    if let Ok(mut table) = world.get::<&mut Table>(entity) {
        table.state = TableState::Flipped;
        table.body = (center + shift, size);
    }
}

/// An enemy bullet stopped by a flipped table chips it; returns whether `entity` was one.
pub(crate) fn chip_table(world: &mut World, entity: Entity) -> bool {
    match world.get::<&mut Table>(entity) {
        Ok(mut table) if table.state == TableState::Flipped => {
            table.hits_left = table.hits_left.saturating_sub(1);
            true
        }
        _ => false,
    }
}

/// Enemies pushing against a table, standing or knocked over, batter it, a knock every half
/// second or so whoever is doing it.
pub(crate) fn batter_tables(world: &mut World, dt: f32) {
    let enemies: Vec<(Vec2, Vec2)> = world
        .query::<(&Position, &Size)>()
        .with::<&Enemy>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .map(|(position, size)| (position.0, size.0))
        .collect();
    for table in world.query_mut::<&mut Table>() {
        table.battered = (table.battered - dt).max(0.0);
        if table.battered > 0.0 || table.hits_left == 0 {
            continue;
        }
        let (at, size) = table.body;
        let pushing = enemies.iter().any(|(enemy, enemy_size)| {
            ((*enemy - at).abs() * 2.0).cmple(size + *enemy_size + Vec2::splat(BATTER_REACH * 2.0)).all()
        });
        if pushing {
            table.hits_left -= 1;
            table.battered = BATTER_EVERY;
        }
    }
}

/// Tables battered or shot to pieces burst apart, bits flying, and settle into scrap on the
/// floor.
pub(crate) fn break_tables(
    world: &mut World,
    physics: &mut Physics,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    dt: f32,
) {
    let broken: Vec<(Entity, Vec2, usize)> = world
        .query::<(Entity, &Table, &Position)>()
        .iter()
        .filter(|(_, table, _)| table.hits_left == 0)
        .map(|(entity, table, position)| (entity, position.0, table.style))
        .collect();
    let texture = textures.get(tex::TABLE);
    for (entity, at, style) in broken {
        if let Ok(body) = world.remove_one::<Body>(entity) {
            physics.remove(body);
        }
        let _ = world.despawn(entity);
        world.spawn((
            TableBurst { style, time: 0.0 },
            Position(at),
            Size(SIZE),
            Color(Vec4::ONE),
            Sprite::cell(texture, BURST[0], style, COLUMNS, ROWS),
            Layer(DEBRIS_LAYER),
            Lit,
        ));
        sounds.push(AudioCommand::Effect(SoundEffect::WallHit));
    }
    let mut settled = Vec::new();
    for (entity, burst, sprite) in world.query_mut::<(Entity, &mut TableBurst, &mut Sprite)>() {
        burst.time += dt;
        let frame = ((burst.time / BURST_FRAME) as usize).min(BURST.len() - 1);
        sprite.uv = Sprite::cell_uv(BURST[frame], burst.style, COLUMNS, ROWS);
        if frame == BURST.len() - 1 {
            settled.push(entity);
        }
    }
    for entity in settled {
        let _ = world.remove_one::<TableBurst>(entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Bullet, EnemyBullet};

    const CENTER: Vec2 = Vec2::new(400.0, 400.0);

    fn room_with(upright: bool) -> (World, Physics, Entity, Textures) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        let mut table = Table {
            state: TableState::Standing,
            hits_left: TABLE_HITS,
            upright,
            center: CENTER,
            style: 0,
            body: (CENTER, Vec2::ZERO),
            battered: 0.0,
        };
        table.body = standing_body(&table);
        let (body_at, body_size) = table.body;
        let entity = world.spawn((
            table,
            Position(CENTER),
            Size(SIZE),
            Sprite::whole(crate::render::TextureId::WHITE),
        ));
        let body = physics.add_table(entity, body_at, body_size, false);
        let _ = world.insert_one(entity, body);
        (world, physics, entity, textures)
    }

    /// The player walking into the table from `from` (relative to its middle) at `velocity`.
    fn bump(upright: bool, from: Vec2, velocity: Vec2) -> (World, Physics, Entity) {
        let (mut world, mut physics, table, textures) = room_with(upright);
        world.spawn((
            Player,
            Position(CENTER + from),
            Size(Vec2::splat(56.0)),
            Velocity(velocity),
        ));
        shove_tables(&mut world, &mut physics, &textures, &mut Vec::new(), Vec2::ZERO);
        (world, physics, table)
    }

    fn picture(world: &World, table: Entity) -> [f32; 4] {
        world.get::<&Sprite>(table).map(|s| s.uv).expect("sprite")
    }

    fn state(world: &World, table: Entity) -> Option<TableState> {
        world.get::<&Table>(table).ok().map(|table| table.state)
    }

    #[test]
    fn walking_into_a_table_from_below_knocks_it_over_away_and_from_above_towards() {
        let (world, _, table) = bump(false, Vec2::new(0.0, 60.0), Vec2::new(0.0, -200.0));
        assert_eq!(state(&world, table), Some(TableState::Flipped));
        assert_eq!(picture(&world, table), Sprite::cell_uv(UNDERSIDE, 0, COLUMNS, ROWS));

        let (world, _, table) = bump(false, Vec2::new(0.0, -50.0), Vec2::new(0.0, 200.0));
        assert_eq!(picture(&world, table), Sprite::cell_uv(TOP_UP, 0, COLUMNS, ROWS));
    }

    #[test]
    fn a_turned_table_falls_to_the_far_side_from_the_push() {
        let (world, _, table) = bump(true, Vec2::new(-50.0, 0.0), Vec2::new(200.0, 0.0));
        // Pushed right, its top faces right and its legs back left at the player.
        assert_eq!(
            picture(&world, table),
            Sprite::flipped(Sprite::cell_uv(FELL_SIDEWAYS, 0, COLUMNS, ROWS))
        );

        let (world, _, table) = bump(true, Vec2::new(50.0, 0.0), Vec2::new(-200.0, 0.0));
        assert_eq!(picture(&world, table), Sprite::cell_uv(FELL_SIDEWAYS, 0, COLUMNS, ROWS));
    }

    #[test]
    fn pushing_against_a_table_that_holds_the_player_up_still_knocks_it_over() {
        let (mut world, mut physics, table, textures) = room_with(true);
        world.spawn((
            Player,
            Position(CENTER + Vec2::new(56.0, 0.0)),
            Size(Vec2::splat(56.0)),
            Velocity(Vec2::ZERO),
        ));
        shove_tables(&mut world, &mut physics, &textures, &mut Vec::new(), Vec2::new(-1.0, 0.0));
        assert_eq!(state(&world, table), Some(TableState::Flipped));
    }

    #[test]
    fn standing_still_or_walking_away_leaves_it_standing() {
        let (world, _, table) = bump(false, Vec2::new(0.0, 60.0), Vec2::ZERO);
        assert_eq!(state(&world, table), Some(TableState::Standing));
        let (world, _, table) = bump(false, Vec2::new(0.0, 60.0), Vec2::new(0.0, 200.0));
        assert_eq!(state(&world, table), Some(TableState::Standing));
        let (world, _, table) = bump(false, Vec2::new(0.0, 300.0), Vec2::new(0.0, -200.0));
        assert_eq!(state(&world, table), Some(TableState::Standing));
    }

    #[test]
    fn a_flipped_table_stops_enemy_bullets_but_not_the_players() {
        let (mut world, mut physics, table) = bump(false, Vec2::new(0.0, 60.0), Vec2::new(0.0, -200.0));
        let enemy_round = world.spawn((EnemyBullet, Position(Vec2::ZERO)));
        let enemy_body = physics.add_enemy_bullet(
            enemy_round,
            Vec2::new(400.0, 300.0),
            Vec2::new(0.0, 600.0),
            Vec2::splat(8.0),
        );
        let _ = world.insert_one(enemy_round, enemy_body);
        let round = world.spawn((Bullet::plain(1), Position(Vec2::ZERO), Velocity(Vec2::ZERO)));
        let body = physics.add_bullet(
            round,
            Vec2::new(400.0, 480.0),
            Vec2::new(0.0, -600.0),
            Vec2::splat(8.0),
        );
        let _ = world.insert_one(round, body);
        let (mut enemy_hit, mut player_hit) = (false, false);
        for _ in 0..30 {
            physics.step(1.0 / 60.0);
            enemy_hit |= physics
                .touching(enemy_body)
                .contains(&crate::physics::Owner::Entity(table));
            player_hit |= physics
                .touching(body)
                .contains(&crate::physics::Owner::Entity(table));
        }
        assert!(enemy_hit);
        assert!(!player_hit);
    }

    #[test]
    fn enough_enemy_bullets_break_a_flipped_table_into_scrap() {
        let (mut world, mut physics, table, textures) = room_with(false);
        // A standing table is not chipped.
        assert!(!chip_table(&mut world, table));
        flip(&mut world, &mut physics, &textures, table, CENTER, Fall::Away);
        for _ in 0..TABLE_HITS {
            assert!(chip_table(&mut world, table));
        }
        break_tables(&mut world, &mut physics, &textures, &mut Vec::new(), 0.0);
        assert!(world.get::<&Table>(table).is_err());
        assert_eq!(world.query::<&Sprite>().iter().count(), 1, "the scrap");
        // It bursts apart and then lies as scrap.
        assert_eq!(world.query::<&TableBurst>().iter().count(), 1);
        for _ in 0..30 {
            break_tables(&mut world, &mut physics, &textures, &mut Vec::new(), 1.0 / 60.0);
        }
        assert_eq!(world.query::<&TableBurst>().iter().count(), 0);
    }

    #[test]
    fn an_enemy_pushing_against_a_table_batters_it_to_pieces() {
        let (mut world, mut physics, table, textures) = room_with(false);
        world.spawn((
            Enemy {
                death: crate::components::Clip {
                    row: 0,
                    frames: 1,
                    frame_time: 1.0,
                    looping: false,
                    flipped: false,
                },
                kind: crate::enemies::EnemyKind::Rat,
            },
            Position(CENTER + Vec2::new(0.0, 50.0)),
            Size(Vec2::splat(48.0)),
        ));
        for _ in 0..(TABLE_HITS as f32 * BATTER_EVERY * 60.0 + 10.0) as usize {
            batter_tables(&mut world, 1.0 / 60.0);
        }
        break_tables(&mut world, &mut physics, &textures, &mut Vec::new(), 1.0 / 60.0);
        assert!(world.get::<&Table>(table).is_err());
    }
}
