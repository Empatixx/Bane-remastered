use glam::Vec2;
use hecs::World;

use crate::assets::Assets;
use crate::audio::AudioCommand;
use crate::dungeon::{Dungeon, RoomKind, Surfaces};
use crate::flow::FlowField;
use crate::input::Input;
use crate::navigation::Navigation;
use crate::physics::Physics;

use super::run_state::RunState;
use super::{FloorAir, Fx, Hostile};

/// What every phase of a scene's step needs: the world and the physics, the floor's paths, the
/// assets, the input, the frame's signals (sounds and camera trauma) and the clocks. Built by
/// the scene for a run of phases and dropped before the scene does its own work in between.
pub(crate) struct Tick<'a> {
    pub(crate) world: &'a mut World,
    pub(crate) physics: &'a mut Physics,
    pub(crate) navigation: &'a Navigation,
    pub(crate) assets: &'a Assets,
    pub(crate) input: &'a Input,
    /// The sounds asked for in this step, for the scene machine to take.
    pub(crate) sounds: &'a mut Vec<AudioCommand>,
    /// The camera trauma of the hard hits in this step, for the camera to take.
    pub(crate) fx: &'a mut Fx,
    /// Where the camera stands (without its shake), which the mouse is aimed from.
    pub(crate) camera: Vec2,
    /// The scene's clock, in seconds.
    pub(crate) clock: f32,
    /// The length of the step.
    pub(crate) dt: f32,
    /// The length of the step for the hostiles: `dt` slowed by the hourglass.
    pub(crate) hostile_dt: f32,
}

impl Tick<'_> {
    /// The world beside the enemy-side context for this step, both borrowed from the tick.
    pub(crate) fn hostile(&mut self) -> (&mut World, Hostile<'_>) {
        (
            &mut *self.world,
            Hostile {
                physics: &mut *self.physics,
                navigation: self.navigation,
                textures: &self.assets.textures,
                sounds: &mut *self.sounds,
                fx: &mut *self.fx,
                dt: self.hostile_dt,
            },
        )
    }
}

impl std::fmt::Debug for Tick<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tick")
            .field("dt", &self.dt)
            .field("hostile_dt", &self.hostile_dt)
            .field("clock", &self.clock)
            .finish()
    }
}

/// What only the in-game phases need besides the [`Tick`]: the run's state, the floor's air,
/// its surfaces and chase paths, the dungeon, and whether the player was hurt.
pub(crate) struct Floor<'a> {
    pub(crate) run: &'a mut RunState,
    pub(crate) air: &'a mut FloorAir,
    pub(crate) surfaces: &'a Surfaces,
    pub(crate) flow: &'a mut FlowField,
    pub(crate) dungeon: Option<&'a Dungeon>,
    /// Set when lava or a touch hurts the player, for the hurt flash.
    pub(crate) hit: &'a mut bool,
}

impl std::fmt::Debug for Floor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Floor").field("hit", &self.hit).finish()
    }
}

/// The player's own moves: a dodge started, the steering (slowed by `surfaces`) and the
/// dodges ticked. Shared by the in-game step and the lobby, which has no surfaces.
pub(crate) fn move_player_phase(tick: &mut Tick, surfaces: &Surfaces) {
    super::start_dodge(tick.world, tick.input, tick.input.mouse() + tick.camera, tick.sounds);
    super::steer_player_on(tick.world, tick.input, surfaces, tick.dt);
    super::tick_dodges(tick.world, tick.dt);
}

/// The player's step in a run: the dice for this step's hits are cast, then the player moves.
pub(crate) fn player_phase(tick: &mut Tick, floor: &mut Floor) {
    super::cast_hit_roll(tick.world, floor.run.combat_rng.float());
    move_player_phase(tick, floor.surfaces);
}

/// The hostiles move and act: the risers rise, the chasers follow the flow field, then every
/// enemy kind and boss takes its turn. `tick_enrage` comes before the first boss.
pub(crate) fn hostile_phase(tick: &mut Tick, floor: &mut Floor) {
    let hostile_dt = tick.hostile_dt;
    super::tick_spawning(tick.world, tick.physics, hostile_dt);
    super::dash(tick.world, tick.sounds, hostile_dt);
    super::update_flow(floor.flow, tick.world, tick.physics, tick.navigation, tick.dt);
    super::chase_player(tick.world, floor.flow, hostile_dt);
    let clock = tick.clock;
    let (world, mut hostile) = tick.hostile();
    super::tick_enrage(world, &mut *hostile.sounds);
    super::tick_golem(world, &mut hostile);
    super::tick_rock_falls(world, &mut hostile);
    super::tick_icicle_bats(world, &mut hostile);
    super::tick_ember_rats(world, &mut *hostile.sounds, clock);
    super::tick_swoopers(world, &mut hostile);
    super::tick_magma_slimes(world, &mut hostile);
    super::tick_worms(world, &mut hostile);
    super::tick_ashes(world, &mut hostile);
    super::tick_hoppers(world, hostile_dt);
    super::tick_baits(world, &mut hostile);
    super::tick_mimic(world, &mut hostile);
    super::tick_hound(world, &mut hostile);
    let boss_room = floor.dungeon.and_then(|dungeon| {
        dungeon
            .rooms
            .iter()
            .find(|room| room.kind == RoomKind::Boss)
            .map(|room| (room.origin.0, room.origin.1, room.columns, room.rows))
    });
    super::tick_magma_spread(world, &mut hostile, boss_room);
    super::tick_magma(world, &mut hostile);
    super::tick_ogre(world, &mut hostile);
    super::tick_argus(world, &mut hostile);
    super::tick_laser(world, &mut hostile);
    super::tick_flames(world, &mut hostile);
    super::tick_orbiters(world, &mut *hostile.physics, hostile_dt);
}

/// The pet picks its target, plans its path and steers. Shared by the in-game step and the
/// lobby.
pub(crate) fn pets_phase(tick: &mut Tick) {
    super::pick_pet_targets(tick.world);
    super::plan_pet_paths(tick.world, tick.physics, tick.navigation);
    super::steer_pets(tick.world);
}

/// The enemies turn to the player and attack: smashes, eye shots, homing rounds, fuses,
/// hatchers, web spitters, leapers and the armageddon.
pub(crate) fn enemy_attacks_phase(tick: &mut Tick) {
    let assets = tick.assets;
    let textures = &assets.textures;
    let hostile_dt = tick.hostile_dt;
    super::face_player(tick.world);
    super::tick_smashes(tick.world, tick.physics, textures, tick.sounds, hostile_dt);
    super::tick_eye_shooters(tick.world, tick.physics, textures, tick.sounds, hostile_dt);
    super::steer_homing(tick.world, hostile_dt);
    super::burn_fuses(tick.world, tick.physics, hostile_dt);
    super::tick_hatchers(tick.world, tick.physics, textures, tick.sounds, hostile_dt);
    super::tick_web_spitters(tick.world, textures, tick.sounds, hostile_dt);
    super::tick_leapers(tick.world, tick.sounds, tick.fx, hostile_dt);
    super::tick_armageddon(tick.world, tick.physics, textures, tick.sounds, hostile_dt);
}

/// The floor's machinery moves: webs slow the bullets, carts roll, the waterworks flow,
/// pendulums swing and the player shoves tables.
pub(crate) fn contraptions_phase(tick: &mut Tick, floor: &mut Floor) {
    let assets = tick.assets;
    let textures = &assets.textures;
    super::slow_bullets_in_webs(tick.world, tick.physics, tick.dt);
    super::tick_carts(tick.world, tick.physics, tick.dt);
    super::tick_waterworks(tick.world, tick.physics, floor.surfaces, textures, tick.clock, tick.dt);
    super::tick_pendulums(tick.world, tick.sounds, tick.fx, tick.clock, tick.hostile_dt);
    super::shove_tables(tick.world, tick.physics, textures, tick.sounds, tick.input.direction());
}

/// Rounds burn out and the bullets that touched something hit it. Shared by the in-game step
/// and the lobby (whose camera does not shake, so it passes trauma nobody takes).
pub(crate) fn bullets_phase(tick: &mut Tick) {
    super::burn_out_rounds(tick.world, tick.physics, tick.dt);
    super::bullet_hits(tick.world, tick.physics, &tick.assets.textures, tick.sounds, tick.fx);
}

/// The physics steps everything that moves, the stuck chasers are noticed, then the bullets
/// hit what they touched.
pub(crate) fn physics_phase(tick: &mut Tick) {
    super::simulate(tick.world, tick.physics, tick.dt);
    super::watch_progress(tick.world, tick.hostile_dt);
    bullets_phase(tick);
}

/// Everything else that hurts the hostiles lands (pet bites, ghosts, turrets, traps, breaking
/// objects, poison, frost), the run's rules toughen the new enemies, and then every hit dealt
/// in this step is handled.
pub(crate) fn combat_phase(tick: &mut Tick, floor: &mut Floor) {
    let assets = tick.assets;
    let textures = &assets.textures;
    let dt = tick.dt;
    super::pet_bites(tick.world, dt);
    super::tick_ghosts(tick.world, dt);
    super::tick_turrets(tick.world, tick.physics, textures, dt);
    super::trap_hits_enemies(tick.world, dt);
    let (world, mut hostile) = tick.hostile();
    super::tick_arrow_traps(world, &mut hostile);
    super::break_objects(
        tick.world,
        tick.physics,
        textures,
        tick.sounds,
        tick.fx,
        |bound| floor.run.loot_rng.below(bound),
    );
    super::leave_poison_trails(tick.world, textures);
    super::tick_poison(tick.world, dt, tick.sounds, tick.fx);
    super::tick_frozen(tick.world, dt);
    super::tick_chilled(tick.world, dt);
    super::toughen_enemies(tick.world, floor.run.rules.enemy_health, floor.run.rules.enemy_damage);
    // Every hit dealt above flashes, counts and floats its number before the deaths read the
    // hit counts.
    super::hit_phase(tick.world);
}

/// What dies in this step dies: the death phase, after every hit is handled.
pub(crate) fn deaths_phase(tick: &mut Tick) {
    super::death_phase(tick.world, tick.physics, &tick.assets.textures, tick.sounds);
}

/// The surfaces act on what stands on them: lava burns, liquids settle, water ripples, sand
/// and the like are kicked up and the surfaces in view stir. `view` is where the camera draws
/// from.
pub(crate) fn surfaces_phase(tick: &mut Tick, floor: &mut Floor, view: Vec2) {
    if super::burn_on_lava(tick.world, floor.surfaces, tick.sounds, tick.fx) {
        *floor.hit = true;
    }
    super::sync_liquids(tick.world, tick.clock);
    super::ripple_on_water(tick.world, floor.surfaces, &tick.assets.textures, tick.dt);
    super::kick_up_surfaces(tick.world, floor.surfaces, tick.dt);
    super::stir_surfaces(tick.world, &mut floor.air.stir, floor.surfaces, view, crate::viewport::TARGET, tick.dt);
}

/// The hostiles' touches and bullets hurt the player, and the tables take their knocks.
pub(crate) fn contact_phase(tick: &mut Tick, floor: &mut Floor) {
    if super::contact_damage(tick.world, tick.sounds, tick.fx) {
        *floor.hit = true;
    }
    super::enemy_bullet_hits(tick.world, tick.physics);
    super::batter_tables(tick.world, tick.hostile_dt);
    super::break_tables(tick.world, tick.physics, &tick.assets.textures, tick.sounds, tick.dt);
}
