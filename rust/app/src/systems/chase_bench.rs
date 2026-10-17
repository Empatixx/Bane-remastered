//! A measure of how well enemies find their way to the player through real rooms: walking
//! enemies of every size are let loose in the combat rooms of generated floors, the player
//! standing still, and the share that reach the player, how long they take and how long they
//! spend stuck are printed. `cargo test chase_bench -- --ignored --nocapture`.

use glam::Vec2;
use hecs::World;

use crate::assets::Textures;
use crate::components::{Position, Size};
use crate::dungeon::{Dungeon, RoomKind, Theme};
use crate::enemies::EnemyKind;
use crate::flow::FlowField;
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::prefabs;
use crate::rng::Rng;
use crate::tilemap::{TILE_SIZE, Tileset};

const STEP: f32 = 1.0 / 60.0;
const SECONDS: f32 = 12.0;
const KINDS: [EnemyKind; 8] = [
    EnemyKind::Rat,
    EnemyKind::Snake,
    EnemyKind::Spider,
    EnemyKind::StiltSpider,
    EnemyKind::MiniGolem,
    EnemyKind::RedSpiderling,
    EnemyKind::FrostRat,
    EnemyKind::BloatedSpider,
];
const FAR: f32 = 380.0;

fn solid_cells(dungeon: &Dungeon) -> Vec<(usize, usize)> {
    let tileset = Tileset::dungeon().expect("the embedded tileset");
    let grid = &dungeon.grid;
    let mut cells = Vec::new();
    for row in 0..grid.rows() {
        for column in 0..grid.columns() {
            if !grid.is_interior(column, row) && tileset.is_solid(tileset.tile_number(grid, column, row)) {
                cells.push((column, row));
            }
        }
    }
    cells
}

/// One step of the enemies' thinking and moving, in the order the game runs it.
pub(crate) fn ai_step(world: &mut World, physics: &mut Physics, navigation: &Navigation, flow: &mut FlowField) {
    super::update_flow(flow, world, physics, navigation, STEP);
    super::chase_player(world, flow, STEP);
    super::simulate(world, physics, STEP);
    super::watch_progress(world, STEP);
}

#[derive(Debug, Default)]
struct Tally {
    kinds: std::collections::BTreeMap<String, (usize, usize)>,
    /// Of those that didn't get there: no way in their size, too slow for the time, stuck.
    no_way: usize,
    steps: usize,
    step_seconds: f32,
    too_slow: usize,
    failed_stuck: Vec<(String, f32, f32)>,
    enemies: usize,
    reached: usize,
    seconds_to_reach: f32,
    stuck_seconds: f32,
}

fn run_room(dungeon: &Dungeon, cells: &[(usize, usize)], room: usize, rng: &mut Rng, tally: &mut Tally) {
    let room = &dungeon.rooms[room];
    let navigation = Navigation::new(cells);
    let open = |at: Vec2, size: Vec2| !navigation.is_blocked(at, size);
    let tiles: Vec<Vec2> = (room.origin.1..room.origin.1 + room.rows)
        .flat_map(|row| (room.origin.0..room.origin.0 + room.columns).map(move |column| (column, row)))
        .map(|(column, row)| (Vec2::new(column as f32, row as f32) + 0.5) * TILE_SIZE)
        .collect();
    let Some(&player_at) = tiles
        .iter()
        .filter(|&&at| open(at, Vec2::splat(TILE_SIZE * 2.0)))
        .min_by(|a, b| {
            a.distance(room.pixel_center()).total_cmp(&b.distance(room.pixel_center()))
        })
    else {
        return;
    };
    let mut world = World::new();
    let mut physics = Physics::new();
    physics.add_walls(cells, TILE_SIZE);
    prefabs::player(&mut world, &mut physics, player_at, &Textures::default());
    let mut enemies = Vec::new();
    for kind in KINDS {
        let size = kind.spec().size();
        let spots: Vec<Vec2> = tiles
            .iter()
            .copied()
            .filter(|&at| at.distance(player_at) > FAR && open(at, size + Vec2::splat(8.0)))
            .filter(|&at| enemies.iter().all(|&(_, other): &(hecs::Entity, Vec2)| other.distance(at) > 140.0))
            .collect();
        if let Some(&at) = rng.pick(&spots) {
            let enemy = prefabs::ready_enemy(&mut world, &mut physics, kind, at);
            enemies.push((enemy, at));
        }
    }
    let mut flow = FlowField::default();
    let mut reached = vec![None; enemies.len()];
    let mut had_way = vec![false; enemies.len()];
    let mut last: Vec<Vec2> = enemies.iter().map(|&(_, at)| at).collect();
    let mut still = vec![0.0f32; enemies.len()];
    let steps = (SECONDS / STEP) as usize;
    for step in 0..steps {
        let started = std::time::Instant::now();
        ai_step(&mut world, &mut physics, &navigation, &mut flow);
        tally.step_seconds += started.elapsed().as_secs_f32();
        tally.steps += 1;
        if step == 0 {
            for (index, &(enemy, at)) in enemies.iter().enumerate() {
                let size = world.get::<&Size>(enemy).map_or(Vec2::ZERO, |s| s.0);
                had_way[index] = flow.leads_from(at, size);
            }
        }
        for (index, &(enemy, _)) in enemies.iter().enumerate() {
            if reached[index].is_some() {
                continue;
            }
            let Ok(at) = world.get::<&Position>(enemy).map(|p| p.0) else {
                continue;
            };
            let size = world.get::<&Size>(enemy).map_or(Vec2::ZERO, |s| s.0);
            // Into the fight: within a short lunge of the player, crowding round them.
            let touch = size.max_element() / 2.0 + 200.0;
            if at.distance(player_at) < touch {
                reached[index] = Some(step as f32 * STEP);
            }
            if at.distance(last[index]) < 0.5 {
                still[index] += STEP;
            }
            last[index] = at;
        }
    }
    for (index, done) in reached.iter().enumerate() {
        let kind = world
            .get::<&crate::components::Enemy>(enemies[index].0)
            .map(|enemy| format!("{:?}", enemy.kind))
            .unwrap_or_default();
        if done.is_none() {
            let speed = world.get::<&crate::components::Chase>(enemies[index].0).map_or(0.0, |c| c.speed);
            let left = last[index].distance(player_at);
            if !had_way[index] {
                tally.no_way += 1;
            } else if speed * SECONDS < enemies[index].1.distance(player_at) * 1.3 {
                tally.too_slow += 1;
            } else {
                tally.failed_stuck.push((kind.clone(), still[index], left));
            }
        }
        let entry = tally.kinds.entry(kind).or_default();
        entry.0 += 1;
        entry.1 += usize::from(done.is_some());
        tally.enemies += 1;
        tally.stuck_seconds += still[index];
        if let Some(seconds) = done {
            tally.reached += 1;
            tally.seconds_to_reach += seconds;
        }
    }
}

#[test]
#[ignore = "a measure to read, not a check; run it with --ignored --nocapture"]
fn chase_bench() {
    let mut tally = Tally::default();
    let mut rng = Rng::new(7);
    for theme in [Theme::Stone, Theme::Temple, Theme::Sewer, Theme::Spider] {
        for seed in 0..4 {
            let dungeon = Dungeon::generate_themed(&mut Rng::new(seed), theme).expect("a dungeon");
            let cells = solid_cells(&dungeon);
            for (index, room) in dungeon.rooms.iter().enumerate() {
                if matches!(room.kind, RoomKind::Normal | RoomKind::Ambush) {
                    run_room(&dungeon, &cells, index, &mut rng, &mut tally);
                }
            }
        }
    }
    let share = tally.reached as f32 / tally.enemies.max(1) as f32 * 100.0;
    println!(
        "chase_bench: {} enemies, {:.0}% reached the player, {:.1} s on average to get there, {:.1} s stuck on average",
        tally.enemies,
        share,
        tally.seconds_to_reach / tally.reached.max(1) as f32,
        tally.stuck_seconds / tally.enemies.max(1) as f32,
    );
    println!(
        "  a step of thinking and moving takes {:.2} ms on average",
        tally.step_seconds / tally.steps.max(1) as f32 * 1000.0
    );
    println!(
        "  not there: {} had no way in their size, {} too slow for the time, {} should have made it",
        tally.no_way,
        tally.too_slow,
        tally.failed_stuck.len()
    );
    for (kind, still, left) in tally.failed_stuck.iter().take(25) {
        println!("    {kind:>14}: still {still:.1} s, {left:.0} px short");
    }
    for (kind, (all, reached)) in &tally.kinds {
        println!("  {kind:>14}: {:.0}% of {all}", *reached as f32 / *all as f32 * 100.0);
    }
}

/// Eight enemies closing in on a player standing in the open settle: those that reach the
/// player stand there, each in a place of its own, and the rest prowl slowly round at a
/// distance; none of them shoves on at full speed.
#[test]
fn a_crowd_settles_round_the_player_instead_of_shoving() {
    let mut world = World::new();
    let mut physics = Physics::new();
    let player_at = Vec2::new(2000.0, 2000.0);
    prefabs::player(&mut world, &mut physics, player_at, &Textures::default());
    let navigation = Navigation::new(&[]);
    let mut flow = FlowField::default();
    let crowd: Vec<hecs::Entity> = (0..8)
        .map(|i| {
            let at = player_at + Vec2::from_angle(i as f32 * 0.785) * 420.0;
            let kind = if i % 2 == 0 { EnemyKind::Rat } else { EnemyKind::Snake };
            prefabs::ready_enemy(&mut world, &mut physics, kind, at)
        })
        .collect();
    for _ in 0..(6.0 / STEP) as usize {
        ai_step(&mut world, &mut physics, &navigation, &mut flow);
    }
    let mut standing = 0;
    for enemy in crowd {
        let at = world.get::<&Position>(enemy).map_or(Vec2::ZERO, |p| p.0);
        let speed = world.get::<&crate::components::Velocity>(enemy).map_or(0.0, |v| v.0.length());
        let full = world.get::<&crate::components::Chase>(enemy).map_or(0.0, |c| c.speed);
        assert!(speed < full * 0.5, "{speed} of {full} at {}", at.distance(player_at));
        standing += usize::from(speed == 0.0);
    }
    assert!(standing >= 3, "{standing}");
}
