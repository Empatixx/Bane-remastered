//! The schedule of a scene's step, as named phases: plain functions over a [`Tick`] (and, for
//! the in-game ones, a [`Floor`]) that a scene calls in order. The scene keeps its own gates
//! (death screen, pause, big map, training, victory) and its own work (camera, rooms, room
//! events, interactions) between the phases, and lends the tick out again after each piece.
//!
//! The in-game fixed step: [`player_phase`], [`hostile_phase`], [`pets_phase`],
//! [`enemy_attacks_phase`], [`contraptions_phase`], [`physics_phase`], [`combat_phase`],
//! [`deaths_phase`], [`surfaces_phase`], [`contact_phase`], then the camera takes the trauma.
//!
//! The in-game frame: the shot, [`arms_phase`], [`lightning_phase`], [`pickups_phase`],
//! [`advance_clock`], [`scenery_phase`], [`hud_phase`], the rooms and their events, [`gone_phase`],
//! [`items_phase`], [`story_phase`], then the camera, the interactions and the trauma.
//!
//! The lobby shares [`move_player_phase`], [`pets_phase`], [`bullets_phase`] and
//! [`player_look_phase`] and runs the rest of its short step by hand.
//!
//! # Ordering contract
//!
//! What must precede what, and why. Inside a phase the calls keep the order they had when the
//! schedule was one long list; the phases keep it too.
//!
//! 1. **Bullets before deaths.** `simulate` moves the bullets, `bullet_hits` lands them
//!    ([`physics_phase`]), and only then are the deaths resolved ([`deaths_phase`]), so a
//!    killing shot kills in the step it lands.
//! 2. **Hits before deaths, flashes and numbers.** `hit_phase` runs after every `deal` of its
//!    loop and before anything that reads what a hit leaves behind: in the fixed step at the
//!    end of [`combat_phase`], before `death_phase` (a burst reads the hit count); in the frame
//!    in [`lightning_phase`], before `flash_hits` and `float_texts` tick in [`hud_phase`]; in the
//!    lobby right after [`bullets_phase`]. No system after it in the same loop deals damage.
//!    One reader sits before it on purpose: `tick_poison` checks `HitFlash` between `deal` and
//!    `hit_phase`, so a hit dealt this tick has not flashed yet. That is harmless, because
//!    `flash_hits` overwrites the outline in the frame anyway.
//! 3. **Died lasts the death phase.** `death_phase` tags the fallen `Died`, every death effect
//!    reads the tag, and the tag is cleared at its end; nothing outside it reads `Died`.
//! 4. **Gone readers before `despawn_gone`.** In the frame, [`gone_phase`] tags finished corpses
//!    `Gone`, pays out the bloodlust hearts, midas coins and drops, and only then despawns
//!    them, so a kill pays once whatever the frames between fixed steps.
//! 5. **Enrage before the bosses.** `tick_enrage` runs before the first boss tick in
//!    [`hostile_phase`], and nothing between the boss ticks hurts or freezes a boss, so every
//!    boss acts on this step's rage.
//! 6. **Trauma drains last.** The camera takes the trauma the systems gathered at the end of
//!    every fixed step and of every frame, so a hard hit shakes the frame it happened in.
//! 7. **The clock before the scenery.** [`advance_clock`] moves the scene's clock on after
//!    [`pickups_phase`] and before [`scenery_phase`], whose wind, trails, dust and swaying read
//!    it. The tick lends the clock (`&mut f32`) rather than copying it, so no phase can read a
//!    stale time.
//! 8. **Body requests before `simulate`.** The `Teleport`, `MoveTo` and `SetVelocity` requests
//!    of [`hostile_phase`] (Argus's blink, the orbiting eyes), [`pets_phase`] (the pet's jump
//!    to its owner) and [`contraptions_phase`] (carts, floating barrels) are carried out by
//!    `simulate` in [`physics_phase`] of the same step; a ricochet's `Teleport` from
//!    [`bullets_phase`] by the next step's. Nothing in between asks the physics where those
//!    bodies are or how fast they go; a write that must be seen at once uses `Physics::place`
//!    or `Physics::halt` instead (see `systems::simulation`).
//! 9. **Risers join the simulation in their step.** `tick_spawning` opens [`hostile_phase`]: an
//!    enemy whose rise ends gets its body there and acts in the same step.

use glam::Vec2;
use hecs::World;

use crate::artefacts::ArtefactKind;
use crate::assets::{Assets, BULLET, tex};
use crate::audio::AudioCommand;
use crate::components::{Berserk, Boss, Health, InfiniteAmmo, Player};
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
    /// The scene's clock, in seconds. Lent rather than copied, so a phase after
    /// [`advance_clock`] reads the advanced time and never a stale copy.
    pub(crate) clock: &'a mut f32,
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
            .field("clock", &*self.clock)
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
    let clock = *tick.clock;
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
    super::tick_orbiters(world, hostile_dt);
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
    super::tick_carts(tick.world, tick.dt);
    super::tick_waterworks(tick.world, tick.physics, floor.surfaces, textures, *tick.clock, tick.dt);
    super::tick_pendulums(tick.world, tick.sounds, tick.fx, *tick.clock, tick.hostile_dt);
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
    super::sync_liquids(tick.world, *tick.clock);
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

/// How the player looks in the frame: the walk animation and the dodge's look and cooldown.
/// Shared by the in-game frame and the lobby.
pub(crate) fn player_look_phase(tick: &mut Tick) {
    super::animate_player(tick.world, tick.input);
    super::show_dodge_visual(tick.world, &tick.assets.textures);
    super::show_dodge_cooldown(tick.world);
}

/// The player's arms in the frame, after the shot: spent casings fly, the player's look, a gun
/// switch and the artefact used. Returns the artefact used, if any, for the scene to act on.
pub(crate) fn arms_phase(tick: &mut Tick) -> Option<ArtefactKind> {
    super::tick_casings(tick.world, tick.dt);
    player_look_phase(tick);
    super::switch_guns(tick.world, tick.input, tick.sounds);
    super::activate_artefact(tick.world, tick.physics, tick.input, tick.assets.textures.get(BULLET))
}

/// The artefact lightning strikes, and its hits are handled in this frame, before the flashes
/// and numbers tick in [`hud_phase`].
pub(crate) fn lightning_phase(tick: &mut Tick) {
    let seed = crate::rng::cosmetic() * 1000.0;
    if super::cast_lightning(tick.world, &tick.assets.textures, seed) > 0 {
        tick.fx.shake(super::LIGHTNING_SHAKE);
    }
    super::hit_phase(tick.world);
}

/// A gun dropped, the drops glide, chests open and the player's invulnerability runs down.
pub(crate) fn pickups_phase(tick: &mut Tick, floor: &mut Floor) {
    let assets = tick.assets;
    super::drop_gun(tick.world, tick.input, tick.camera, &assets.textures);
    super::glide(tick.world, tick.dt);
    let loot = &mut floor.run.loot_rng;
    super::open_chests(tick.world, &assets.textures, loot.float(), |bound| loot.below(bound), tick.sounds);
    super::tick_invulnerability(tick.world, tick.dt);
}

/// The scene's clock moves on by the frame. Runs after [`pickups_phase`] and before
/// [`scenery_phase`], whose wind, trails, dust and swaying read the advanced time.
pub(crate) fn advance_clock(tick: &mut Tick) {
    *tick.clock += tick.dt;
}

/// What moves for the eye alone: the gaze wind (or the berserk's), bullet trails, dust, the
/// floor's weather, what hangs overhead, drips, the lightning's fade and the shadows.
pub(crate) fn scenery_phase(tick: &mut Tick, floor: &mut Floor) {
    let assets = tick.assets;
    let textures = &assets.textures;
    let (dt, clock) = (tick.dt, *tick.clock);
    let berserk = tick.world.query::<&Berserk>().iter().count() > 0;
    let (particle, phase) = if berserk {
        (tex::ARTEFACTS_BERSERKPOT_PARTICLE, crate::artefacts::BERSERK_PHASE)
    } else {
        (tex::GAZEWIND_PARTICLE, 0.0)
    };
    super::spawn_gaze_wind(tick.world, tick.physics, textures.get(particle), dt, clock + phase);
    super::shed_trails(tick.world, &super::TrailTextures::load(textures), dt, clock);
    super::tick_dust(tick.world, clock, dt);
    if let Some(air) = &mut floor.air.atmosphere {
        super::tick_atmosphere(
            tick.world,
            air,
            super::AtmosphereTextures::load(textures),
            tick.camera,
            dt,
        );
    }
    super::tick_overheads(tick.world, dt);
    super::tick_hanging(tick.world, clock, dt);
    super::tick_drips(tick.world, textures, dt);
    super::tick_lightning(tick.world, dt);
    super::update_shadow(tick.world, dt);
}

/// The bars and the weapon readout, then the frame's feedback: lights fade, hit flashes,
/// afterimages, drops hover and are outlined, and the damage numbers float.
pub(crate) fn hud_phase(tick: &mut Tick) {
    let dt = tick.dt;
    super::update_status_bars(tick.world, dt);
    let infinite = tick
        .world
        .query::<&InfiniteAmmo>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|_| *tick.clock);
    super::update_weapon_hud(tick.world, &tick.assets.textures, infinite);
    super::fade_lights(tick.world);
    super::flash_hits(tick.world, dt);
    super::fade_afterimages(tick.world, dt);
    super::hover_drops(tick.world, dt);
    super::outline_drops(tick.world);
    super::float_texts(tick.world, dt);
}

/// Finished corpses and effects are tagged `Gone`, paid out (bloodlust hearts, midas coins,
/// drops) and despawned, all in this frame.
pub(crate) fn gone_phase(tick: &mut Tick, floor: &mut Floor) {
    let textures = &tick.assets.textures;
    super::mark_gone(tick.world);
    super::bloodlust_heals_on_kill(tick.world);
    super::midas_kill_coins(tick.world, textures);
    let run = &mut *floor.run;
    super::enemy_drops(tick.world, &mut run.kills_without_drop, textures, |bound| {
        run.loot_rng.below(bound)
    });
    super::despawn_gone(tick.world);
}

/// Items are collected and age, and the artefacts, bloodlust, summons, shops and slot machines
/// tick; the flames flicker.
pub(crate) fn items_phase(tick: &mut Tick, floor: &mut Floor) {
    let assets = tick.assets;
    let textures = &assets.textures;
    let dt = tick.dt;
    super::collect_items(tick.world, tick.sounds);
    super::age_items(tick.world, dt);
    super::update_price_tags(tick.world);
    super::tick_berserk(tick.world, dt);
    super::apply_heal_percent(tick.world, textures, tick.sounds);
    super::convert_bullets(tick.world, tick.physics, textures);
    super::tick_artefact_effects(tick.world, dt);
    super::spawn_bloodlust_drops(tick.world, textures, dt);
    super::tick_blood_drops(tick.world, dt);
    super::pulse_bloodlust(tick.world);
    super::spawn_summons(tick.world, textures);
    super::update_shield_aura(tick.world, textures);
    super::update_artefact_hud(tick.world, textures);
    super::update_slot_prices(tick.world);
    let loot = &mut floor.run.loot_rng;
    super::tick_slot_machines(tick.world, textures, dt, tick.sounds, |bound| loot.below(bound));
    super::flicker_flames(tick.world);
}

/// Who speaks and what is told: speech, captions and souls tick, `BANE_SLAY` fells the boss
/// when its time is up, and the bubbles and price boxes are fitted. `view` is where the camera
/// draws from.
pub(crate) fn story_phase(tick: &mut Tick, floor: &mut Floor, view: Vec2) {
    let assets = tick.assets;
    let dt = tick.dt;
    super::tick_speech(tick.world, dt);
    super::tick_captions(tick.world, dt);
    super::tick_souls(tick.world, dt);
    if let Some(left) = &mut floor.run.slay_in {
        *left -= dt;
        if *left <= 0.0 {
            floor.run.slay_in = None;
            for health in tick.world.query_mut::<&mut Health>().with::<&Boss>() {
                health.current = 0;
            }
        }
    }
    super::fit_speech(tick.world, &assets.fonts);
    super::keep_speech_on_screen(tick.world, view);
    super::fit_price_boxes(tick.world, &assets.fonts);
}
