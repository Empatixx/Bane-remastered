//! The training crypt's lessons (see `dungeon::training`): the guide talks the player
//! through each room, the keys and a few words are written on the floor, and the gate to the
//! next room opens once the lesson is learnt. Each room is a
//! checkpoint: dying there brings the player back to its start, whole again, with the lesson set
//! out afresh; after a few deaths in one room the guide has a tip.

use glam::{Vec2, Vec4};
use hecs::{Entity, World};
use rapier2d::prelude::ColliderHandle;

use crate::artefacts::ArtefactKind;
use crate::assets::{Textures, tex};
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    ArtefactSlot, Armor, Body, BreakKind, Broken, Color, Dodge, Health, Invulnerable,
    Layer, Loadout, Player, Position, ScreenSpace, Size, SpeechBubble, SpeechText,
    Text, Velocity, Weapon,
};
use crate::dungeon::{Direction, Lesson, LessonRoom};
use crate::enemies::EnemyKind;
use crate::guns::GunKind;
use crate::items::{AmmoBox, ItemKind, PotionSize, Vital};
use crate::physics::Physics;
use crate::prefabs;
use crate::props::{Look, Prop};
use crate::render::FontSize;
use crate::tilemap::TILE_SIZE;

/// How close the player has to come to the far end of a room to have crossed it.
const GOAL_REACH: f32 = 90.0;
/// How long each of the guide's lines stays up, and the pause before the next.
const LINE_SECONDS: f32 = 4.2;
const LINE_GAP: f32 = 0.4;
/// Deaths in one room before the guide gives a tip.
const DEATHS_FOR_A_TIP: u32 = 3;
const RESPAWN_FADE: f32 = 0.7;
const RESPAWN_GRACE: f32 = 1.0;
/// The trial's foes come in a moment after its gate shuts.
const TRIAL_DELAY: f32 = 1.0;
const DONE_SECONDS: f32 = 1.5;
/// The training opens in the dark: the crypt fades in while the guide walks up with his lantern,
/// and he speaks once he is there.
const INTRO_FADE: f32 = 2.4;
const INTRO_WALK: f32 = 2.6;
/// How far below his place the guide sets out from, in tiles.
const INTRO_FROM: f32 = 5.5;
/// The light of the guide's lantern, held out in his right hand.
const LANTERN_AT: Vec2 = Vec2::new(15.0, 4.0);
/// Guns and artefacts lie up on the stand's top, not in its middle.
const STAND_LIFT: f32 = 26.0;

/// The keys drawn on the floor, the size the first room of a run draws them.
const FLOOR_KEY: Vec2 = Vec2::new(33.0, 36.0);
const FLOOR_SPACE: Vec2 = Vec2::new(99.0, 36.0);
const FLOOR_KEY_STEP: f32 = 50.0;
/// The words written on the floor, under the keys.
const FLOOR_TEXT: Vec4 = Vec4::new(0.92, 0.88, 0.8, 0.75);
const DONE_COLOR: Vec4 = Vec4::new(0.55, 1.0, 0.5, 1.0);
/// "Done!" stands this far above the player's head.
const OVER_HEAD: f32 = 70.0;
const OVER_HEAD_LAYER: i32 = 6;
/// How far above the guide's head his speech bubble floats.
const SPEECH_RISE: f32 = 26.0;


fn lines(lesson: Lesson) -> &'static [&'static str] {
    match lesson {
        Lesson::Move => &["You're awake! Good.", "WASD moves you. Follow the passage."],
        Lesson::Shoot => &["Aim with the mouse, click to shoot.", "Break all three pots!", "Your ammo is bottom right. R reloads."],
        Lesson::Loot => &["Shoot the barrel, see what falls out.", "Coins and ammo: just walk over them.", "Take the shotgun from the stand with E.", "Then switch between your guns with 1 and 2."],
        Lesson::Dodge => &["Spikes! Too many to walk over.", "Press Space to roll across them.", "While you roll, nothing can hurt you."],
        Lesson::Health => &["Top left: hearts, and armour under them.", "Armour takes the hits first, then hearts.", "Kill the rat. A potion heals you after."],
        Lesson::Traps => &["Arrow traps shoot when you come close.", "Water slows you, lava burns, ice slides.", "Spikes come and go. Time your steps!"],
        Lesson::Cover => &["Walk into a table to knock it over.", "A fallen table stops enemy shots.", "Fight from behind it!"],
        Lesson::Artefact => &["See that ring? It's an artefact. Take it with E.", "Press F to use it: a ring of bullets!", "One use per artefact, so pick your moment."],
        Lesson::Trial => &["Now a real fight.", "Show me what you've learnt!"],
        Lesson::Exit => &["Well done! You're ready.", "Step up to the portal and press E. Welcome home!"],
    }
}


fn tip(lesson: Lesson) -> &'static str {
    match lesson {
        Lesson::Dodge => "Tip: roll just before the spikes.",
        Lesson::Health => "Tip: shoot the rat before it reaches you.",
        Lesson::Traps => "Tip: keep out of the lava, roll the spikes.",
        Lesson::Cover => "Tip: hide behind the table, shoot round it.",
        Lesson::Trial => "Tip: keep moving, roll away from bites.",
        _ => "Tip: take it slowly.",
    }
}


/// What the training wants of the scene after a step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TrainingEvent {
    None,
    /// The player has gone through the portal: the training is done.
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Trial {
    Waiting,
    Arming(f32),
    Fighting,
}

#[derive(Debug)]
pub(crate) struct Training {
    lessons: Vec<LessonRoom>,
    current: usize,
    /// The current lesson is learnt and its gate open, the next not yet begun.
    learnt: bool,
    deaths: Vec<u32>,
    /// What the current lesson set out, cleared and set out again after a death.
    props: Vec<Entity>,
    exit_gate: Vec<(Entity, ColliderHandle)>,
    entry_gate: Vec<(Entity, ColliderHandle)>,
    guide: Option<Entity>,
    line: usize,
    line_timer: f32,
    /// A tip waiting to be said in place of the lesson's lines.
    tip: Option<&'static str>,
    pots: Vec<Entity>,
    barrel: Option<Entity>,
    spilt: bool,
    slot_at_pickup: Option<usize>,
    switched: bool,
    rat: Option<Entity>,
    potion: Option<Entity>,
    foes: Vec<Entity>,
    trial: Trial,
    /// The artefact lying on the floor, until the player takes it.
    artefact: Option<Entity>,
    finished: bool,
    /// The black over the screen fading out: the entity, how long it has faded, and over how
    /// long it fades.
    fade: Option<(Entity, f32, f32)>,
    /// The guide still walking up at the start: how long he has walked.
    intro: Option<f32>,
    /// Words over the player's head, and how long they stay.
    over_head: Option<(Entity, f32)>,
}

fn player(world: &World) -> Option<(Entity, Vec2)> {
    world
        .query::<(Entity, &Position)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(entity, position)| (entity, position.0))
}

/// A few words written on the floor, centred on `at`.
fn floor_text(world: &mut World, content: &str, at: Vec2) -> Entity {
    world.spawn((
        Position(at),
        Text::centered(content.to_owned(), FLOOR_TEXT, FontSize::Size22),
        Layer(-1),
    ))
}

/// A stand (the shop's display table) centred on `at`, for a gun or an artefact to lie on.
/// Returns the stand and where on it the thing lies.
fn stand(world: &mut World, at: Vec2, textures: &Textures) -> (Entity, Vec2) {
    let size = Prop::ShopTable.spec().size();
    let table = prefabs::prop(world, Prop::ShopTable, at - size / 2.0, Look::DEFAULT, textures);
    (table, at - Vec2::new(0.0, STAND_LIFT))
}

/// Draws what lies on a stand in front of it, as the shop does its goods.
fn on_stand(world: &mut World, thing: Entity, stand_at: Vec2) {
    let bottom = stand_at.y + Prop::ShopTable.spec().size().y / 2.0;
    if let Ok(position) = world.get::<&Position>(thing).map(|position| position.0) {
        let _ = world.insert_one(thing, crate::components::YSort { foot: bottom - position.y + 0.5 });
    }
}

/// A black sheet over the whole screen, for fading in from.
fn black(world: &mut World) -> Entity {
    world.spawn((
        Position(Vec2::new(480.0, 270.0)),
        Size(Vec2::new(960.0, 540.0)),
        Color(Vec4::new(0.0, 0.0, 0.0, 1.0)),
        Layer(9),
        ScreenSpace,
    ))
}

fn gone(world: &World, entity: Entity) -> bool {
    !world.contains(entity)
        || world.get::<&Broken>(entity).is_ok()
        || world.get::<&Health>(entity).is_ok_and(|health| health.current <= 0)
}

impl Training {
    pub(crate) fn new(lessons: Vec<LessonRoom>) -> Self {
        let count = lessons.len();
        Self {
            lessons,
            current: 0,
            learnt: false,
            deaths: vec![0; count],
            props: Vec::new(),
            exit_gate: Vec::new(),
            entry_gate: Vec::new(),
            guide: None,
            line: 0,
            line_timer: 0.0,
            tip: None,
            pots: Vec::new(),
            barrel: None,
            spilt: false,
            slot_at_pickup: None,
            switched: false,
            rat: None,
            potion: None,
            foes: Vec::new(),
            trial: Trial::Waiting,
            artefact: None,
            finished: false,
            fade: None,
            intro: None,
            over_head: None,
        }
    }

    /// Starts at the `index`-th lesson rather than the first, for working on the game.
    pub(crate) fn start_at(&mut self, index: usize) {
        self.current = index.min(self.lessons.len().saturating_sub(1));
    }

    pub(crate) fn start_position(&self) -> Option<Vec2> {
        self.lessons.get(self.current).map(|lesson| lesson.checkpoint)
    }

    /// The guide, his speech bubble, the panel at the top, and the first lesson.
    pub(crate) fn begin(&mut self, world: &mut World, physics: &mut Physics, textures: &Textures) {
        let guide = prefabs::prop(world, Prop::Guide, Vec2::ZERO, Look::DEFAULT, textures);
        let _ = world.insert_one(guide, crate::components::Light::rgb(LANTERN_AT, 255.0, 196.0, 120.0, 120.0, 1.3));
        self.guide = Some(guide);
        self.set_pieces(world, physics, textures);
        prefabs::minimal_speech(world, Vec2::ZERO);
        // The bars over the alcoves and the chains down the walls stay for good.
        for lesson in &self.lessons {
            for &tile in &lesson.bars {
                let _ = prefabs::gate(world, physics, tile, Direction::Up, textures);
            }
            for &(column, row) in &lesson.chains {
                let top_left = Vec2::new(column as f32, row as f32) * TILE_SIZE;
                prefabs::prop(world, Prop::Chains, top_left, Look::DEFAULT, textures);
            }
        }
        // The artefact comes in its own lesson and the second gun in the loot lesson, both found
        // on the floor: the player sets out with a pistol alone.
        for slot in world.query_mut::<&mut ArtefactSlot>().with::<&Player>() {
            slot.current = None;
        }
        for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
            loadout.clear();
            let _ = loadout.add(Weapon::new(GunKind::Pistol));
        }
        self.set_out(world, physics, textures);
        // Into the dark: the crypt fades in and the guide walks up from further down.
        self.fade = Some((black(world), 0.0, INTRO_FADE));
        self.intro = Some(0.0);
        self.line_timer = INTRO_WALK;
        self.place_guide(world, self.lesson().guide + Vec2::new(0.0, INTRO_FROM * TILE_SIZE));
    }

    /// Each hall's own piece, there for good: racks of swords in the armoury, crates and candles
    /// round the loot, bookshelves where the cover is, candles round the artefact, statues
    /// either side of the way out.
    fn set_pieces(&self, world: &mut World, physics: &mut Physics, textures: &Textures) {
        for lesson in &self.lessons {
            let (left, top) = (lesson.origin.0 as f32 * TILE_SIZE, lesson.origin.1 as f32 * TILE_SIZE);
            // Against the top wall, its foot a little out on the floor.
            let wall = |column: f32| Vec2::new(left + (column + 0.5) * TILE_SIZE, top + 0.6 * TILE_SIZE);
            // Standing on the floor, its foot at the given point.
            let floor = |column: f32, row: f32| Vec2::new(left + column * TILE_SIZE, top + row * TILE_SIZE);
            let pieces: Vec<(Prop, Vec2, bool)> = match lesson.lesson {
                Lesson::Shoot => vec![
                    (Prop::WeaponRack, wall(6.0), false),
                    (Prop::WeaponRack, wall(12.0), false),
                    (Prop::Armorstand, wall(8.0) + Vec2::new(0.0, 0.3 * TILE_SIZE), true),
                    (Prop::Armorstand, wall(10.0) + Vec2::new(0.0, 0.3 * TILE_SIZE), true),
                ],
                Lesson::Loot => vec![
                    (Prop::Box, floor(3.5, 6.0), true),
                    (Prop::Box, floor(13.5, 6.0), true),
                    (Prop::Candlestick, floor(5.5, 2.0), false),
                    (Prop::Candlestick, floor(11.5, 2.0), false),
                ],
                Lesson::Cover => vec![(Prop::Bookshelf, wall(6.0), false), (Prop::Bookshelf, wall(16.0), false)],
                Lesson::Artefact => vec![
                    (Prop::Candlestick, floor(5.5, 4.0), false),
                    (Prop::Candlestick, floor(9.5, 4.0), false),
                    (Prop::Candlestick, floor(5.5, 7.6), false),
                    (Prop::Candlestick, floor(9.5, 7.6), false),
                ],
                Lesson::Exit => vec![(Prop::AngelStatue, floor(3.0, 7.6), true), (Prop::AngelStatue, floor(12.0, 7.6), true)],
                _ => Vec::new(),
            };
            for (prop, foot, solid) in pieces {
                let size = prop.spec().size();
                let top_left = foot - Vec2::new(size.x / 2.0, size.y);
                let entity = prefabs::prop(world, prop, top_left, Look::DEFAULT, textures);
                let _ = world.insert_one(entity, crate::components::YSort::bottom(size));
                if solid {
                    let body = Vec2::new(size.x * 0.7, size.y.min(TILE_SIZE) * 0.6);
                    let _ = physics.add_static_box(foot - Vec2::new(0.0, body.y / 2.0), body);
                }
            }
        }
    }

    /// Puts the guide at `at`, his speech bubble over his head.
    fn place_guide(&self, world: &mut World, at: Vec2) {
        let Some(guide) = self.guide else { return };
        let size = Prop::Guide.spec().size();
        if let Ok(mut position) = world.get::<&mut Position>(guide) {
            position.0 = at;
        }
        let anchor = at - Vec2::new(0.0, size.y / 2.0 + SPEECH_RISE);
        for (position, _) in world.query_mut::<(&mut Position, &SpeechBubble)>() {
            position.0 = anchor;
        }
        for (position, _) in world.query_mut::<(&mut Position, &SpeechText)>() {
            position.0 = anchor;
        }
    }

    fn lesson(&self) -> &LessonRoom {
        &self.lessons[self.current]
    }

    /// Puts the guide by the way in, sets the panel and lays out the lesson.
    fn set_out(&mut self, world: &mut World, physics: &mut Physics, textures: &Textures) {
        let lesson = self.lesson().clone();
        self.place_guide(world, lesson.guide);
        self.line = 0;
        self.line_timer = 0.0;
        if self.exit_gate.is_empty() {
            self.exit_gate = lesson
                .exit
                .iter()
                .map(|&tile| prefabs::gate(world, physics, tile, Direction::Right, textures))
                .collect();
        }
        self.pots.clear();
        self.barrel = None;
        self.spilt = false;
        self.slot_at_pickup = None;
        self.switched = false;
        self.rat = None;
        self.potion = None;
        self.foes.clear();
        self.artefact = None;
        self.trial = Trial::Waiting;
        let start = lesson.checkpoint;
        match lesson.lesson {
            Lesson::Move => {
                // The movement keys on the floor before the player, blinking, as in the first
                // room of a run.
                let middle = start.x + 3.0 * TILE_SIZE;
                let top = start.y - FLOOR_KEY.y * 1.5;
                for (key, dx, dy) in [
                    (tex::KEYBOARD_W, 0.0, 0.0),
                    (tex::KEYBOARD_A, -FLOOR_KEY_STEP, FLOOR_KEY_STEP),
                    (tex::KEYBOARD_S, 0.0, FLOOR_KEY_STEP),
                    (tex::KEYBOARD_D, FLOOR_KEY_STEP, FLOOR_KEY_STEP),
                ] {
                    let at = Vec2::new(middle + dx - FLOOR_KEY.x / 2.0, top + dy);
                    self.props.push(prefabs::key_hint(world, key, at, FLOOR_KEY, textures));
                }
                self.props.push(floor_text(world, "Move", Vec2::new(middle, top + 2.0 * FLOOR_KEY_STEP + 10.0)));
            }
            Lesson::Shoot => {
                // Close together, a little group by the far wall.
                for (column, row) in [(14, 4), (15, 5), (14, 6)] {
                    let at = lesson.at(column, row);
                    let pot = prefabs::breakable(world, physics, BreakKind::Pot, at - Vec2::splat(TILE_SIZE / 2.0), textures);
                    self.pots.push(pot);
                    self.props.push(pot);
                }
            }
            Lesson::Loot => {
                let at = lesson.at(8, 4);
                let barrel = prefabs::breakable(world, physics, BreakKind::Barrel, at - Vec2::splat(TILE_SIZE / 2.0), textures);
                self.barrel = Some(barrel);
                self.props.push(barrel);
                // The shotgun waits on a stand at the far side of the carpet.
                let stand_at = lesson.at(8, 7);
                let (table, top) = stand(world, stand_at, textures);
                let gun = prefabs::gun_drop(world, Weapon::new(GunKind::Shotgun), top, Vec2::ZERO, textures);
                on_stand(world, gun, stand_at);
                self.props.extend([table, gun]);
            }
            Lesson::Dodge => {
                // Space on the floor just before the band of spikes.
                let at = Vec2::new(lesson.at(5, 0).x, start.y) - FLOOR_SPACE / 2.0;
                self.props.push(prefabs::key_hint(world, tex::KEYBOARD_SPACE, at, FLOOR_SPACE, textures));
                self.props.push(floor_text(world, "Roll", Vec2::new(lesson.at(5, 0).x, start.y + 40.0)));
            }
            Lesson::Health => {
                let rat = prefabs::enemy(world, EnemyKind::Rat, lesson.at(11, 5), 1, textures);
                self.rat = Some(rat);
                self.props.push(rat);
            }
            Lesson::Traps => {
            }
            Lesson::Cover => {
                // An Uzi on a stand by the way in, to try a gun that fires while held.
                let stand_at = lesson.at(6, 2);
                let (table, top) = stand(world, stand_at, textures);
                let uzi = prefabs::gun_drop(world, Weapon::new(GunKind::Uzi), top, Vec2::ZERO, textures);
                on_stand(world, uzi, stand_at);
                self.props.extend([table, uzi]);
                let slime = prefabs::enemy(world, EnemyKind::Slime, lesson.at(17, 5), 1, textures);
                self.foes.push(slime);
                self.props.push(slime);
                for (column, row) in [(14, 9), (17, 8), (20, 9)] {
                    let rat = prefabs::enemy(world, EnemyKind::Rat, lesson.at(column, row), 1, textures);
                    self.foes.push(rat);
                    self.props.push(rat);
                }
            }
            Lesson::Artefact => {
                // On the way through, where the player will walk.
                let at = lesson.at(7, 5);
                let (table, top) = stand(world, at, textures);
                let drop = prefabs::artefact_drop(world, ArtefactKind::RingOfDeath, top, Vec2::ZERO, textures);
                on_stand(world, drop, at);
                self.artefact = Some(drop);
                self.props.extend([table, drop]);
                let key = at + Vec2::new(-1.2 * TILE_SIZE, -1.6 * TILE_SIZE);
                self.props.push(prefabs::key_hint(world, tex::KEYBOARD_E, key - FLOOR_KEY / 2.0, FLOOR_KEY, textures));
                let use_key = key + Vec2::new(2.4 * TILE_SIZE, 0.0);
                self.props.push(prefabs::key_hint(world, tex::KEYBOARD_F, use_key - FLOOR_KEY / 2.0, FLOOR_KEY, textures));
                self.props.push(floor_text(world, "take", key + Vec2::new(0.0, 36.0)));
                self.props.push(floor_text(world, "use", use_key + Vec2::new(0.0, 36.0)));
            }
            Lesson::Trial => {
                // Something heavier before the fight.
                let stand_at = lesson.at(6, 2);
                let (table, top) = stand(world, stand_at, textures);
                let rifle = prefabs::gun_drop(world, Weapon::new(GunKind::M4), top, Vec2::ZERO, textures);
                on_stand(world, rifle, stand_at);
                self.props.extend([table, rifle]);
            }
            Lesson::Exit => {
                let size = Prop::Portal.spec().size();
                let portal = prefabs::portal(world, lesson.at(7, 5) - size / 2.0, textures);
                self.props.push(portal);
            }
        }
    }

    /// The player has died: back to the start of the room, whole again, the lesson set out
    /// afresh.
    pub(crate) fn respawn(&mut self, world: &mut World, physics: &mut Physics, textures: &Textures) {
        let checkpoint = self.lesson().checkpoint;
        if let Some((entity, _)) = player(world) {
            if let Ok(mut health) = world.get::<&mut Health>(entity) {
                health.current = health.max;
            }
            if let Ok(mut armor) = world.get::<&mut Armor>(entity) {
                armor.current = armor.max;
            }
            if let Ok(body) = world.get::<&Body>(entity).map(|body| *body) {
                physics.teleport(body, checkpoint);
                physics.set_velocity(body, Vec2::ZERO);
            }
            if let Ok(mut position) = world.get::<&mut Position>(entity) {
                position.0 = checkpoint;
            }
            if let Ok(mut velocity) = world.get::<&mut Velocity>(entity) {
                velocity.0 = Vec2::ZERO;
            }
            let _ = world.remove_one::<Dodge>(entity);
            let _ = world.insert_one(entity, Invulnerable { remaining: RESPAWN_GRACE });
        }
        self.clear_props(world, physics);
        // The trial's way in opens again until the player comes back into the room.
        for (entity, collider) in self.entry_gate.drain(..) {
            physics.remove_collider(collider);
            let _ = world.despawn(entity);
        }
        if let Some((old, _, _)) = self.fade.take() {
            let _ = world.despawn(old);
        }
        self.fade = Some((black(world), 0.0, RESPAWN_FADE));
        if self.learnt {
            return;
        }
        self.deaths[self.current] += 1;
        if self.deaths[self.current] >= DEATHS_FOR_A_TIP {
            self.tip = Some(tip(self.lesson().lesson));
        }
        self.set_out(world, physics, textures);
    }

    fn clear_props(&mut self, world: &mut World, physics: &mut Physics) {
        for entity in self.props.drain(..) {
            if world.contains(entity) {
                physics.detach(world, entity);
                let _ = world.despawn(entity);
            }
        }
    }

    /// The player stepped into the portal: on to the lobby straight away.
    pub(crate) fn enter_portal(&mut self) {
        if self.lesson().lesson == Lesson::Exit {
            self.finished = true;
        }
    }

    pub(crate) fn update(
        &mut self,
        world: &mut World,
        physics: &mut Physics,
        textures: &Textures,
        sounds: &mut Vec<AudioCommand>,
        dt: f32,
    ) -> TrainingEvent {
        if self.finished {
            return TrainingEvent::Finished;
        }
        if let Some((words, left)) = &mut self.over_head {
            *left -= dt;
            if *left <= 0.0 {
                let _ = world.despawn(*words);
                self.over_head = None;
            } else if let Some((_, at)) = player(world) {
                if let Ok(mut position) = world.get::<&mut Position>(*words) {
                    position.0 = at - Vec2::new(0.0, OVER_HEAD);
                }
            }
        }
        if let Some(walked) = &mut self.intro {
            *walked += dt;
            let done = (*walked / INTRO_WALK).min(1.0);
            // Easing in to a stop at his place.
            let eased = 1.0 - (1.0 - done) * (1.0 - done);
            let to = self.lessons[self.current].guide;
            let from = to + Vec2::new(0.0, INTRO_FROM * TILE_SIZE);
            let at = from.lerp(to, eased);
            if done >= 1.0 {
                self.intro = None;
            }
            self.place_guide(world, at);
        }
        if let Some((fade, time, over)) = &mut self.fade {
            *time += dt;
            let alpha = (1.0 - *time / *over).max(0.0);
            if let Ok(mut color) = world.get::<&mut Color>(*fade) {
                color.0.w = alpha;
            }
            if alpha <= 0.0 {
                let _ = world.despawn(*fade);
                self.fade = None;
            }
        }
        self.talk(world, dt);
        // A gun dropped with Q leaves the hand empty: the other one comes up, so the player is
        // never left unable to shoot.
        // Nor without ammo: the reserves never run dry here.
        for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
            loadout.arm();
            loadout.top_up_reserves();
        }
        let Some((_, at)) = player(world) else {
            return TrainingEvent::None;
        };
        if self.learnt {
            // On into the next room, the next lesson.
            let next = self.current + 1;
            if next < self.lessons.len() {
                let start = (self.lessons[next].origin.0 as f32 + 0.5) * TILE_SIZE;
                if at.x > start {
                    self.current = next;
                    self.learnt = false;
                    self.exit_gate.clear();
                    self.tip = None;
                    self.set_out(world, physics, textures);
                }
            }
            return TrainingEvent::None;
        }
        if self.progress(world, physics, textures, at, dt) {
            self.learn(world, physics, sounds);
        }
        TrainingEvent::None
    }

    /// Steps the lesson on; whether it is learnt.
    fn progress(&mut self, world: &mut World, physics: &mut Physics, textures: &Textures, at: Vec2, dt: f32) -> bool {
        let lesson = self.lesson().clone();
        let near_goal = at.distance(lesson.goal) < GOAL_REACH || at.x > lesson.goal.x;
        match lesson.lesson {
            Lesson::Move | Lesson::Dodge | Lesson::Traps => near_goal,
            Lesson::Shoot => {
                let broken = self.pots.iter().filter(|&&pot| gone(world, pot)).count();
                broken == self.pots.len()
            }
            Lesson::Loot => {
                if let Some(barrel) = self.barrel {
                    if !self.spilt && gone(world, barrel) {
                        self.spilt = true;
                        let center = lesson.at(8, 4);
                        for (index, kind) in [ItemKind::Coin, ItemKind::Coin, ItemKind::Coin, ItemKind::Ammo(AmmoBox::Shotgun)]
                            .into_iter()
                            .enumerate()
                        {
                            let spread = Vec2::from_angle(index as f32 * 1.6) * 90.0;
                            let item = prefabs::item(world, kind, center, false, spread, textures);
                            self.props.push(item);
                        }
                    }
                }
                let held = world
                    .query::<&Loadout>()
                    .with::<&Player>()
                    .iter()
                    .next()
                    .map(|loadout| (loadout.has(GunKind::Shotgun), loadout.current_slot()));
                if let Some((has, slot)) = held {
                    if has {
                        match self.slot_at_pickup {
                            None => {
                                self.slot_at_pickup = Some(slot);
                                let at = self.lessons[self.current].at(8, 2);
                                self.props.push(floor_text(world, "Press 1 or 2 to switch guns", at));
                            }
                            Some(before) if before != slot => self.switched = true,
                            _ => {}
                        }
                    }
                    return has && self.switched;
                }
                false
            }
            Lesson::Health => {
                let Some(rat) = self.rat else { return false };
                if !gone(world, rat) {
                    return false;
                }
                let hurt = world
                    .query::<&Health>()
                    .with::<&Player>()
                    .iter()
                    .next()
                    .is_some_and(|health| health.current < health.max);
                match self.potion {
                    None => {
                        let potion = prefabs::item(world, ItemKind::Potion(Vital::Health, PotionSize::Large), lesson.at(11, 5), false, Vec2::ZERO, textures);
                        self.potion = Some(potion);
                        self.props.push(potion);
                        false
                    }
                    // Taken, or never needed: the player was not hurt.
                    Some(potion) => !world.contains(potion) || !hurt,
                }
            }
            Lesson::Cover => self.foes.iter().all(|&foe| gone(world, foe)),
            Lesson::Artefact => {
                // Taking the ring wakes the rats it is for.
                if let Some(drop) = self.artefact {
                    if !world.contains(drop) {
                        self.artefact = None;
                        for (column, row) in [(12, 1), (13, 5), (13, 9)] {
                            let rat = prefabs::enemy(world, EnemyKind::Rat, lesson.at(column, row), 1, textures);
                            self.foes.push(rat);
                            self.props.push(rat);
                        }
                    }
                    return false;
                }
                let used = world
                    .query::<&ArtefactSlot>()
                    .with::<&Player>()
                    .iter()
                    .next()
                    .is_some_and(|slot| slot.current.is_none());
                used && self.foes.iter().all(|&foe| gone(world, foe))
            }
            Lesson::Trial => match self.trial {
                Trial::Waiting => {
                    let inside = (lesson.origin.0 as f32 + 2.5) * TILE_SIZE;
                    if at.x > inside {
                        self.entry_gate = lesson
                            .entry
                            .iter()
                            .map(|&tile| prefabs::gate(world, physics, tile, Direction::Left, textures))
                            .collect();
                        self.trial = Trial::Arming(TRIAL_DELAY);
                    }
                    false
                }
                Trial::Arming(left) => {
                    let left = left - dt;
                    if left <= 0.0 {
                        let foes = [
                            (EnemyKind::Rat, (15, 2)),
                            (EnemyKind::Rat, (17, 6)),
                            (EnemyKind::Rat, (15, 10)),
                            (EnemyKind::Bat, (19, 5)),
                        ];
                        for (kind, (column, row)) in foes {
                            let foe = prefabs::enemy(world, kind, lesson.at(column, row), 1, textures);
                            self.foes.push(foe);
                            self.props.push(foe);
                        }
                        prefabs::wave_banner(world, 1, 1);
                        self.trial = Trial::Fighting;
                    } else {
                        self.trial = Trial::Arming(left);
                    }
                    false
                }
                Trial::Fighting => {
                    let won = self.foes.iter().all(|&foe| gone(world, foe));
                    if won {
                        for (entity, collider) in self.entry_gate.drain(..) {
                            physics.remove_collider(collider);
                            super::open_gate(world, entity);
                        }
                    }
                    won
                }
            },
            // The portal ends it (see `enter_portal`).
            Lesson::Exit => false,
        }
    }

    fn learn(&mut self, world: &mut World, physics: &mut Physics, sounds: &mut Vec<AudioCommand>) {
        self.learnt = true;
        sounds.push(AudioCommand::Effect(SoundEffect::Buy));
        for (entity, collider) in self.exit_gate.drain(..) {
            physics.remove_collider(collider);
            super::open_gate(world, entity);
        }
        // "Done!" over the player's head, in the world rather than on the screen.
        if let Some((words, _)) = self.over_head.take() {
            let _ = world.despawn(words);
        }
        if let Some((_, at)) = player(world) {
            let words = world.spawn((
                Position(at - Vec2::new(0.0, OVER_HEAD)),
                Text::centered("Done!".to_owned(), DONE_COLOR, FontSize::Size32),
                Layer(OVER_HEAD_LAYER),
            ));
            self.over_head = Some((words, DONE_SECONDS));
        }
    }

    /// The guide says the lesson's lines one after another, round again, or a tip.
    fn talk(&mut self, world: &mut World, dt: f32) {
        self.line_timer -= dt;
        if self.line_timer > 0.0 {
            return;
        }
        self.line_timer = LINE_SECONDS + LINE_GAP;
        if let Some(tip) = self.tip.take() {
            super::say_for(world, tip, LINE_SECONDS);
            return;
        }
        let said = lines(self.lesson().lesson);
        if said.is_empty() {
            return;
        }
        super::say_for(world, said[self.line % said.len()], LINE_SECONDS);
        self.line += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::crypt;

    fn scene() -> (World, Physics, Training, Textures) {
        let (_, lessons) = crypt();
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        let mut training = Training::new(lessons);
        let start = training.start_position().expect("a start");
        prefabs::player(&mut world, &mut physics, start, &textures);
        training.begin(&mut world, &mut physics, &textures);
        (world, physics, training, textures)
    }

    fn step(world: &mut World, physics: &mut Physics, training: &mut Training, textures: &Textures) -> TrainingEvent {
        training.update(world, physics, textures, &mut Vec::new(), 1.0 / 60.0)
    }

    fn move_player(world: &mut World, to: Vec2) {
        for position in world.query_mut::<&mut Position>().with::<&Player>() {
            position.0 = to;
        }
    }

    #[test]
    fn walking_the_passage_learns_the_first_lesson_and_opens_its_gate() {
        let (mut world, mut physics, mut training, textures) = scene();
        assert_eq!(training.exit_gate.len(), 3);
        step(&mut world, &mut physics, &mut training, &textures);
        assert!(!training.learnt);
        let goal = training.lesson().goal;
        move_player(&mut world, goal);
        step(&mut world, &mut physics, &mut training, &textures);
        assert!(training.learnt && training.exit_gate.is_empty());
        // Walking into the next room starts the next lesson, with its own gate.
        let next = training.lessons[1].checkpoint;
        move_player(&mut world, next);
        step(&mut world, &mut physics, &mut training, &textures);
        assert_eq!(training.current, 1);
        assert_eq!(training.lesson().lesson, Lesson::Shoot);
        assert_eq!(training.pots.len(), 3);
        assert_eq!(training.exit_gate.len(), 3);
    }

    #[test]
    fn the_guide_walks_up_in_silence_and_speaks_once_he_is_there() {
        let (mut world, mut physics, mut training, textures) = scene();
        step(&mut world, &mut physics, &mut training, &textures);
        crate::systems::tick_speech(&mut world, 1.0 / 60.0);
        let silent = world
            .query::<&crate::components::SpeechBubble>()
            .iter()
            .all(|bubble| bubble.remaining <= 0.0);
        assert!(silent && training.intro.is_some());
        for _ in 0..((INTRO_WALK + 0.2) * 60.0) as usize {
            step(&mut world, &mut physics, &mut training, &textures);
        }
        assert!(training.intro.is_none());
        let guide = training.guide.expect("guide");
        let at = world.get::<&Position>(guide).map(|p| p.0).expect("position");
        assert_eq!(at, training.lesson().guide);
        crate::systems::tick_speech(&mut world, 1.0 / 60.0);
        let (said, hidden) = world
            .query::<(Entity, &Text)>()
            .with::<&crate::components::SpeechText>()
            .iter()
            .next()
            .map(|(entity, text)| (text.content.clone(), world.get::<&crate::components::Hidden>(entity).is_ok()))
            .expect("speech");
        assert!(!said.is_empty() && !hidden, "{said:?} {hidden}");
    }

    #[test]
    fn the_loot_lesson_starts_with_a_pistol_alone_and_is_learnt_by_taking_the_shotgun_and_switching() {
        let (mut world, mut physics, mut training, textures) = scene();
        let (player, _) = player(&world).expect("player");
        let has = |world: &World, kind| world.get::<&Loadout>(player).map(|l| l.has(kind)).expect("loadout");
        assert!(has(&world, GunKind::Pistol) && !has(&world, GunKind::Shotgun));
        training.current = 2;
        training.set_out(&mut world, &mut physics, &textures);
        let barrel = training.barrel.expect("barrel");
        let _ = world.insert_one(barrel, Broken);
        step(&mut world, &mut physics, &mut training, &textures);
        let drops = world.query::<&crate::components::GunDrop>().iter().count();
        assert_eq!(drops, 1);
        if let Ok(mut loadout) = world.get::<&mut Loadout>(player) {
            let _ = loadout.add(Weapon::new(GunKind::Shotgun));
        }
        step(&mut world, &mut physics, &mut training, &textures);
        assert!(!training.learnt);
        if let Ok(mut loadout) = world.get::<&mut Loadout>(player) {
            loadout.switch_to(1);
        }
        step(&mut world, &mut physics, &mut training, &textures);
        assert!(training.learnt);
    }

    #[test]
    fn an_empty_hand_takes_up_the_other_gun() {
        let (mut world, mut physics, mut training, textures) = scene();
        let (player, _) = player(&world).expect("player");
        if let Ok(mut loadout) = world.get::<&mut Loadout>(player) {
            let _ = loadout.add(Weapon::new(GunKind::Shotgun));
            let _ = loadout.drop_current();
        }
        step(&mut world, &mut physics, &mut training, &textures);
        let armed = world.get::<&Loadout>(player).map(|l| l.current().is_some()).expect("loadout");
        assert!(armed);
    }

    #[test]
    fn breaking_the_pots_learns_shooting() {
        let (mut world, mut physics, mut training, textures) = scene();
        move_player(&mut world, training.lesson().goal);
        step(&mut world, &mut physics, &mut training, &textures);
        move_player(&mut world, training.lessons[1].checkpoint);
        step(&mut world, &mut physics, &mut training, &textures);
        for &pot in &training.pots.clone() {
            let _ = world.insert_one(pot, Broken);
        }
        step(&mut world, &mut physics, &mut training, &textures);
        assert!(training.learnt);
    }

    #[test]
    fn dying_brings_the_player_back_whole_to_the_rooms_start_and_three_deaths_bring_a_tip() {
        let (mut world, mut physics, mut training, textures) = scene();
        let (player, _) = player(&world).expect("player");
        for _ in 0..DEATHS_FOR_A_TIP {
            if let Ok(mut health) = world.get::<&mut Health>(player) {
                health.current = 0;
            }
            move_player(&mut world, Vec2::new(5000.0, 5000.0));
            training.respawn(&mut world, &mut physics, &textures);
        }
        let health = world.get::<&Health>(player).map(|h| (h.current, h.max)).expect("health");
        assert_eq!(health.0, health.1);
        let at = world.get::<&Position>(player).map(|p| p.0).expect("position");
        assert_eq!(at, training.lesson().checkpoint);
        assert!(training.tip.is_some());
    }

    #[test]
    fn the_portal_ends_the_training_at_once() {
        let (mut world, mut physics, mut training, textures) = scene();
        training.current = training.lessons.len() - 1;
        training.set_out(&mut world, &mut physics, &textures);
        training.enter_portal();
        assert_eq!(step(&mut world, &mut physics, &mut training, &textures), TrainingEvent::Finished);
    }
}
