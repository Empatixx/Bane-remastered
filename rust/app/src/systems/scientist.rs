use hecs::World;

use glam::Vec2;

use crate::components::{Chatty, Player, Position, Size, Speaker, SpeechBubble, SpeechPart, Wallet};
use crate::enemies::EnemyKind;
use crate::progress::{ChallengeId, Ending, Remark};
use crate::render::{FontSize, Fonts};
use crate::systems::shop::say;

pub(crate) const HEARING_RANGE: f32 = 360.0;
pub(crate) const LEAVING_RANGE: f32 = 800.0;
pub(crate) const PAUSE_SECONDS: f32 = 7.0;
pub(crate) const RICH_COINS: i32 = 100;

const GREETINGS: [&str; 2] = ["Welcome back!", "Ah, my test subject!"];
/// Before the first run.
const INTRO: [&str; 2] = [
    "A new subject! Welcome, welcome.",
    "The portal is ready. Just step in.",
];
/// After the first win: no more tests, only "our work".
const PARTNER: [&str; 2] = ["Ah, my finest work!", "Welcome back, partner!"];
const HINTS: [&str; 6] = [
    "Upgrades, upgrades!",
    "Dodging is science!",
    "Slimes spit. Beware!",
    "Pots hide coins!",
    "Go on, test my work!",
    "Every run teaches us.",
];
const BROKE: &str = "No talents? Go loot!";
const RICH: &str = "Talents to spend!";
const PAD_X: f32 = 14.0;
const PAD_Y: f32 = 8.0;
const BORDER: f32 = 2.0;
const TAIL: f32 = 8.0;
pub(crate) const PURCHASE: &str = "Excellent choice!";

/// Lines that join the hints once the profile has earned them.
const DIED: &str = "Death is only data, subject.";
const DEEP: &str = "Go deeper. Bring back what you see.";
const WATCHED: &str = "Argus watched you. So did I.";
const WOLF: &str = "It followed you. I did not plan that.";
const SHOWN: &str = "...What did it show you?";
const REST: &str = "Rest. Tomorrow we go down together.";
const BEGUN: &str = "Our work has only begun.";

/// Elian, the last of the order, keeps the training book in the laboratory. He believes the
/// hero is the guardian the order kept; the Scientist calls him subject 41. Before the first run.
const ELIAN_FIRST: [&str; 2] = [
    "This laboratory is built from our temple's stones.",
    "He calls you a subject. You are the guardian.",
];
/// After the true ending, if the wound was left open: Elian lives, and knows what for.
const ELIAN_LEFT_OPEN: [&str; 2] = ["You stopped me. He's still up there, smiling.", "I'll keep praying. Someone has to."];
/// The Scientist after it: silent if the wound was closed, pleased if not.
const SILENT: [&str; 2] = ["...", "My work. All of it."];
const UNDERSTOOD: [&str; 2] = ["Good. Good. We understand each other.", "Welcome back, partner!"];
const ELIAN_GREETINGS: [&str; 2] = ["Guardian! You're back.", "The light is with you still."];
const ELIAN_HINTS: [&str; 5] = [
    "The book has the lessons, if you need them again.",
    "The prayers didn't just stop that night. Someone stopped them.",
    "Who cut Oswin's bell rope? I keep asking myself.",
    "Teodor, Cassian, Oswin... they all wrote. Their pages went below.",
    "The Scientist asks how deep. Never how to close it.",
];
/// Lines that join Elian's once the profile has earned them.
const ELIAN_EARNED_DEEP: &str = "You smell of the deep. Like the night it opened.";
const ELIAN_EARNED_WOLF: &str = "The wolf came up from the wound. It chose you.";
const ELIAN_EARNED_WIN: &str = "He was glad you reached the bottom. Too glad.";
const ELIAN_PAGE: &str = "Teodor's hand! He wrote every night. Find the rest.";
const ELIAN_NAME: &str = "Silas Vane... the scholar in Teodor's pages. I know that face.";
const ELIAN_ALMONDS: &str = "Almonds in the wine. We weren't tired. We were put to sleep.";
const ELIAN_KNOWS: &str = "It was him. The Scientist. Don't let him see that you know.";
/// The Scientist, once pages have come up: he wants them.
const PAPERS: &str = "Papers from below? Bring them to me first, partner.";

/// What the Scientist says once, on the lobby visit after it happened.
pub(crate) fn remark_line(remark: Remark) -> &'static str {
    match remark {
        Remark::Win(1) => "You reached the bottom. Now we can begin.",
        Remark::Win(2) => "It is open. It is finally open!",
        Remark::Win(wins) if wins % 2 == 1 => "Again! Our work goes so well.",
        Remark::Win(_) => "The bottom again. Splendid, partner.",
        Remark::FirstBoss(boss) => boss_line(boss),
        Remark::Depth(2) => "Floor two! Most of them never got past two.",
        Remark::Depth(3) => "Floor three! Deeper than most. Tell me everything.",
        Remark::Depth(4) => "Floor four. It gets quiet down there, yes?",
        Remark::Depth(_) => "The last floor! The bottom is so close now.",
        Remark::Challenge(ChallengeId::Bane) => "You went without my work. And came back.",
        Remark::Challenge(_) => "A clean sample! I wrote it all down.",
        Remark::Deaths(1) => "Back already! Good. Good. Nothing is lost.",
        Remark::Deaths(5) => "Five times. You are holding together nicely.",
        Remark::Deaths(10) => "Ten! Twelve took forty. You are my finest work.",
        Remark::Deaths(20) => "Still you. Still whole. Remarkable.",
        Remark::Deaths(50) => "Fifty. I stopped counting. I did not.",
        Remark::Deaths(_) => "A hundred of you. And still the same one.",
        Remark::Kills(100) => "A hundred specimens! I am out of jars.",
        Remark::Kills(500) => "Five hundred. The wound grows them back, you know.",
        Remark::Kills(1000) => "A thousand. Do you dream of them? I do.",
        Remark::Kills(_) => "Five thousand. It must be getting hungry.",
    }
}

fn boss_line(boss: EnemyKind) -> &'static str {
    if let Some(identity) = crate::lore::identity(boss) {
        return identity.scientist;
    }
    match boss {
        EnemyKind::Spider => "You cut a stitch! The spiders hold the wound shut.",
        EnemyKind::KingSlime => "The king slime... Do not ask what it was made of.",
        EnemyKind::Ogre => "The ogre dug the drains. The wound took him first.",
        EnemyKind::Golem => "Temple stone. It guarded out of habit, poor thing.",
        EnemyKind::Argus => "You closed the Eye? It was... mine. Never mind.",
        EnemyKind::Hellhound => "The hound. Do not listen to it. Dogs lie.",
        EnemyKind::Mimic => "A chest with teeth! Bring me a tooth next time.",
        _ => "A new guardian down! Write it in my notes.",
    }
}

/// The parts of the profile the Scientist's lines depend on.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Standing {
    pub(crate) runs: u64,
    pub(crate) wins: u64,
    pub(crate) deaths: u64,
    pub(crate) best_floor: u32,
    pub(crate) bosses_slain: Vec<EnemyKind>,
    pub(crate) has_pet: bool,
    pub(crate) went_without: bool,
    /// One-time remarks waiting to be said.
    pub(crate) remarks: Vec<Remark>,
    /// Pages of Teodor's chronicle found.
    pub(crate) pages: usize,
    /// The choice at the true ending, if made.
    pub(crate) ending: Option<Ending>,
}

/// What the Scientist knows about the player, taken from the profile when the lobby opens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Repertoire {
    /// One-time remarks, said first, in order.
    pub(crate) remarks: Vec<Remark>,
    pub(crate) greetings: &'static [&'static str],
    pub(crate) hints: Vec<&'static str>,
    /// Whether the talker remarks on the player's purse (the Scientist sells; Elian does not).
    pub(crate) purse: bool,
}

impl Default for Repertoire {
    fn default() -> Self {
        Self {
            remarks: Vec::new(),
            greetings: &GREETINGS,
            hints: HINTS.to_vec(),
            purse: true,
        }
    }
}

impl Repertoire {
    pub(crate) fn of(profile: &Standing) -> Self {
        let greetings: &'static [&'static str] = if profile.ending == Some(Ending::Closed) {
            &SILENT
        } else if profile.ending == Some(Ending::LeftOpen) {
            &UNDERSTOOD
        } else if profile.runs == 0 {
            &INTRO
        } else if profile.wins > 0 {
            &PARTNER
        } else {
            &GREETINGS
        };
        let earned = [
            (profile.deaths > 0, DIED),
            (profile.best_floor >= 3, DEEP),
            (profile.bosses_slain.contains(&EnemyKind::Argus), WATCHED),
            (profile.has_pet, WOLF),
            (profile.went_without, SHOWN),
            (profile.wins > 0, REST),
            (profile.wins > 0, BEGUN),
            (profile.pages > 0, PAPERS),
        ];
        let mut hints = HINTS.to_vec();
        hints.extend(earned.iter().filter(|(has, _)| *has).map(|(_, line)| *line));
        Self {
            remarks: profile.remarks.clone(),
            greetings,
            hints,
            purse: true,
        }
    }

    /// Elian's lines: the order's lore, and what he makes of the hero's runs.
    pub(crate) fn elian(profile: &Standing) -> Self {
        let greetings: &'static [&'static str] = match profile.ending {
            Some(Ending::LeftOpen) => &ELIAN_LEFT_OPEN,
            _ if profile.runs == 0 => &ELIAN_FIRST,
            _ => &ELIAN_GREETINGS,
        };
        let earned = [
            (profile.best_floor >= 3, ELIAN_EARNED_DEEP),
            (profile.has_pet, ELIAN_EARNED_WOLF),
            (profile.wins > 0, ELIAN_EARNED_WIN),
            (profile.pages > 0, ELIAN_PAGE),
            (profile.pages >= 3, ELIAN_NAME),
            (profile.pages >= 6, ELIAN_ALMONDS),
            (profile.pages >= super::PAGES.len(), ELIAN_KNOWS),
        ];
        let mut hints = ELIAN_HINTS.to_vec();
        hints.extend(earned.iter().filter(|(has, _)| *has).map(|(_, line)| *line));
        // The bosses he knew, put to rest; and what each waking is costing him.
        hints.extend(profile.bosses_slain.iter().filter_map(|&boss| crate::lore::identity(boss)).map(|who| who.elian));
        hints.extend(crate::lore::elian_on_his_price(profile.deaths));
        Self { remarks: Vec::new(), greetings, hints, purse: false }
    }
}

/// The line to say next, and the one-time remark it was, if any.
fn pick(
    chatty: &mut Chatty,
    repertoire: &mut Repertoire,
    coins: i32,
) -> (&'static str, Option<Remark>) {
    if !repertoire.remarks.is_empty() {
        let remark = repertoire.remarks.remove(0);
        chatty.greeted = true;
        return (remark_line(remark), Some(remark));
    }
    let line = chatty.line;
    chatty.line += 1;
    if !chatty.greeted {
        chatty.greeted = true;
        let greetings = repertoire.greetings;
        return (greetings[line % greetings.len().max(1)], None);
    }
    let said = match line % 3 {
        1 if repertoire.purse && coins <= 0 => BROKE,
        2 if repertoire.purse && coins >= RICH_COINS => RICH,
        _ => repertoire
            .hints
            .get(line % repertoire.hints.len().max(1))
            .copied()
            .unwrap_or(HINTS[0]),
    };
    (said, None)
}

/// Lets the Scientist talk to a player nearby; returns the one-time remark he just made.
pub(crate) fn chat(world: &mut World, dt: f32) -> Option<Remark> {
    let (player, coins) = world
        .query::<(&Position, Option<&Wallet>)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(position, wallet)| (position.0, wallet.map_or(0, |wallet| wallet.coins)))?;
    let talking = world
        .query::<&SpeechBubble>()
        .iter()
        .any(|bubble| bubble.remaining > 0.0);
    let mut line = None;
    let mut remark = None;
    let mut anchor = None;
    for (position, chatty, repertoire, speaker) in
        world.query_mut::<(&Position, &mut Chatty, Option<&mut Repertoire>, Option<&Speaker>)>()
    {
        chatty.cooldown = (chatty.cooldown - dt).max(0.0);
        let distance = position.0.distance(player);
        if distance > LEAVING_RANGE {
            chatty.greeted = false;
        }
        if distance <= HEARING_RANGE && chatty.cooldown <= 0.0 && !talking {
            chatty.cooldown = PAUSE_SECONDS;
            let (said, made) = match repertoire {
                Some(repertoire) => pick(chatty, repertoire, coins),
                None => pick(chatty, &mut Repertoire::default(), coins),
            };
            line = Some(said);
            remark = made;
            anchor = speaker.map(|speaker| speaker.anchor);
            // One talker at a time: the others wait for the bubble to clear.
            break;
        }
    }
    match (line, anchor) {
        (Some(line), Some(anchor)) => super::shop::say_at(world, anchor, line, super::shop::SPEECH_SECONDS),
        (Some(line), None) => say(world, line),
        _ => {}
    }
    remark
}

/// The highest a speech bubble's top may sit on the screen: below the hearts and the armour.
const SPEECH_CEILING: f32 = 106.0;

/// Pushes the speech bubble down onto the screen when its talker stands so high that it would
/// be cut off (the merchant at his wall, the altar's angel) or would sit under the HUD.
pub(crate) fn keep_speech_on_screen(world: &mut World, camera: Vec2) {
    let Some((anchor, size)) = world
        .query::<(&Position, &Size)>()
        .with::<&SpeechBubble>()
        .iter()
        .next()
        .map(|(position, size)| (position.0, size.0))
    else {
        return;
    };
    let top = anchor.y - size.y / 2.0 - camera.y;
    let push = SPEECH_CEILING - top;
    if push <= 0.0 {
        return;
    }
    let moved: Vec<hecs::Entity> = world
        .query::<hecs::Entity>()
        .with::<hecs::Or<&SpeechBubble, hecs::Or<&SpeechPart, &crate::components::SpeechText>>>()
        .iter()
        .collect();
    for entity in moved {
        if let Ok(mut position) = world.get::<&mut Position>(entity) {
            position.0.y += push;
        }
    }
}

pub(crate) fn fit_speech(world: &mut World, fonts: &Fonts) {
    let font = fonts.get(FontSize::Size22);
    let Some((bubble, anchor, message)) = world
        .query::<(hecs::Entity, &Position, &SpeechBubble)>()
        .iter()
        .next()
        .map(|(entity, position, bubble)| (entity, position.0, bubble.message))
    else {
        return;
    };
    let size = Vec2::new(
        font.width(message) + PAD_X * 2.0,
        font.line_height() + PAD_Y * 2.0,
    );
    if let Ok(mut bubble_size) = world.get::<&mut Size>(bubble) {
        bubble_size.0 = size;
    }
    let parts: Vec<(hecs::Entity, SpeechPart)> = world
        .query::<(hecs::Entity, &SpeechPart)>()
        .iter()
        .map(|(entity, part)| (entity, *part))
        .collect();
    for (entity, part) in parts {
        let (center, extent) = match part {
            SpeechPart::Frame => (anchor, size + Vec2::splat(BORDER * 2.0)),
            SpeechPart::Tail => (
                anchor + Vec2::new(0.0, size.y / 2.0 + BORDER + TAIL / 2.0 - 1.0),
                Vec2::splat(TAIL),
            ),
        };
        if let Ok((position, part_size)) = world.query_one_mut::<(&mut Position, &mut Size)>(entity)
        {
            position.0 = center;
            part_size.0 = extent;
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use super::*;

    fn lobby(player_at: Vec2, coins: i32) -> World {
        let mut world = World::new();
        world.spawn((Player, Position(player_at), Wallet { coins }));
        world.spawn((Position(Vec2::ZERO), Chatty::default()));
        world.spawn((SpeechBubble {
            remaining: 0.0,
            message: "",
        },));
        world
    }

    fn spoken(world: &World) -> (f32, &'static str) {
        world
            .query::<&SpeechBubble>()
            .iter()
            .next()
            .map(|bubble| (bubble.remaining, bubble.message))
            .unwrap_or((0.0, ""))
    }

    #[test]
    fn elian_speaks_over_his_own_head_calls_the_hero_the_guardian_and_never_talks_money() {
        let mut world = World::new();
        world.spawn((Player, Position(Vec2::new(100.0, 0.0)), Wallet { coins: 0 }));
        let over_him = Vec2::new(0.0, -60.0);
        world.spawn((
            Position(Vec2::ZERO),
            Chatty::default(),
            Repertoire::elian(&Standing::default()),
            Speaker { anchor: over_him },
        ));
        world.spawn((SpeechBubble { remaining: 0.0, message: "" }, Position(Vec2::new(900.0, 0.0))));
        let mut said = Vec::new();
        for _ in 0..8 {
            for bubble in world.query_mut::<&mut SpeechBubble>() {
                bubble.remaining = 0.0;
            }
            chat(&mut world, PAUSE_SECONDS);
            said.push(spoken(&world).1);
        }
        assert_eq!(said[0], ELIAN_FIRST[0]);
        assert!(said.iter().all(|line| *line != BROKE && *line != RICH), "{said:?}");
        let at = world.query::<&Position>().with::<&SpeechBubble>().iter().next().map(|p| p.0);
        assert_eq!(at, Some(over_him));
    }

    #[test]
    fn the_scientist_greets_a_player_who_walks_up() {
        let mut world = lobby(Vec2::new(200.0, 0.0), 0);

        chat(&mut world, 0.016);

        let (remaining, message) = spoken(&world);
        assert!(remaining > 0.0);
        assert!(GREETINGS.contains(&message), "{message}");
    }

    #[test]
    fn a_player_out_of_earshot_hears_nothing() {
        let mut world = lobby(Vec2::new(HEARING_RANGE + 100.0, 0.0), 0);

        chat(&mut world, 0.016);

        assert_eq!(spoken(&world).0, 0.0);
    }

    #[test]
    fn the_scientist_does_not_talk_over_himself_or_spam() {
        let mut world = lobby(Vec2::new(200.0, 0.0), 0);
        chat(&mut world, 0.016);
        let first = spoken(&world).1;

        chat(&mut world, 0.016);
        assert_eq!(spoken(&world).1, first);

        for bubble in world.query_mut::<&mut SpeechBubble>() {
            bubble.remaining = 0.0;
        }
        chat(&mut world, 1.0);
        assert_eq!(spoken(&world).0, 0.0);

        chat(&mut world, PAUSE_SECONDS);
        assert!(spoken(&world).0 > 0.0);
        assert_ne!(spoken(&world).1, first);
    }

    #[test]
    fn he_comments_on_an_empty_and_a_full_purse() {
        let mut world = lobby(Vec2::new(200.0, 0.0), 0);
        let mut seen = Vec::new();
        for _ in 0..6 {
            for bubble in world.query_mut::<&mut SpeechBubble>() {
                bubble.remaining = 0.0;
            }
            chat(&mut world, PAUSE_SECONDS);
            seen.push(spoken(&world).1);
        }
        assert!(seen.contains(&BROKE), "{seen:?}");

        let mut world = lobby(Vec2::new(200.0, 0.0), RICH_COINS);
        let mut seen = Vec::new();
        for _ in 0..6 {
            for bubble in world.query_mut::<&mut SpeechBubble>() {
                bubble.remaining = 0.0;
            }
            chat(&mut world, PAUSE_SECONDS);
            seen.push(spoken(&world).1);
        }
        assert!(seen.contains(&RICH), "{seen:?}");
        assert!(!seen.contains(&BROKE), "{seen:?}");
    }

    #[test]
    fn walking_far_away_and_back_earns_a_new_greeting() {
        let mut world = lobby(Vec2::new(200.0, 0.0), 0);
        chat(&mut world, 0.016);
        for bubble in world.query_mut::<&mut SpeechBubble>() {
            bubble.remaining = 0.0;
        }
        for position in world.query_mut::<&mut Position>().with::<&Player>() {
            position.0 = Vec2::new(LEAVING_RANGE + 100.0, 0.0);
        }
        chat(&mut world, PAUSE_SECONDS);
        for position in world.query_mut::<&mut Position>().with::<&Player>() {
            position.0 = Vec2::new(100.0, 0.0);
        }
        chat(&mut world, PAUSE_SECONDS);

        assert!(GREETINGS.contains(&spoken(&world).1));
    }

    fn hear_all(world: &mut World, times: usize) -> Vec<&'static str> {
        let mut seen = Vec::new();
        for _ in 0..times {
            for bubble in world.query_mut::<&mut SpeechBubble>() {
                bubble.remaining = 0.0;
            }
            chat(world, PAUSE_SECONDS);
            seen.push(spoken(world).1);
        }
        seen
    }

    fn lobby_for(profile: &Standing) -> World {
        let mut world = lobby(Vec2::new(200.0, 0.0), 50);
        let scientist = world
            .query::<hecs::Entity>()
            .with::<&Chatty>()
            .iter()
            .next()
            .expect("the scientist");
        world
            .insert_one(scientist, Repertoire::of(profile))
            .expect("a repertoire");
        world
    }

    #[test]
    fn a_player_who_never_went_down_gets_the_introduction() {
        let mut world = lobby_for(&Standing::default());

        chat(&mut world, 0.016);

        assert!(INTRO.contains(&spoken(&world).1), "{}", spoken(&world).1);
    }

    #[test]
    fn a_winner_is_greeted_as_a_partner_and_hears_about_our_work() {
        let profile = Standing {
            runs: 3,
            wins: 1,
            deaths: 2,
            ..Standing::default()
        };
        let mut world = lobby_for(&profile);

        let seen = hear_all(&mut world, 24);

        assert!(PARTNER.contains(&seen[0]), "{seen:?}");
        assert!(seen.contains(&REST) && seen.contains(&BEGUN), "{seen:?}");
        assert!(seen.contains(&DIED), "{seen:?}");
    }

    #[test]
    fn earned_lines_stay_silent_until_the_profile_earns_them() {
        let fresh = Repertoire::of(&Standing {
            runs: 1,
            deaths: 1,
            best_floor: 1,
            ..Standing::default()
        });
        for line in [DEEP, WATCHED, WOLF, SHOWN, REST, BEGUN] {
            assert!(!fresh.hints.contains(&line), "{line}");
        }
        assert!(fresh.hints.contains(&DIED));

        let mut veteran = Standing {
            runs: 9,
            deaths: 9,
            best_floor: 4,
            ..Standing::default()
        };
        veteran.bosses_slain.push(EnemyKind::Argus);
        let veteran = Repertoire::of(&veteran);
        assert!(veteran.hints.contains(&DEEP) && veteran.hints.contains(&WATCHED));
        assert!(HINTS.iter().all(|hint| veteran.hints.contains(hint)));
    }

    #[test]
    fn waiting_remarks_come_first_once_each_and_are_reported_as_heard() {
        let profile = Standing {
            runs: 2,
            deaths: 1,
            remarks: vec![
                Remark::FirstBoss(EnemyKind::Spider),
                Remark::Depth(2),
            ],
            ..Standing::default()
        };
        let mut world = lobby_for(&profile);

        let first = chat(&mut world, 0.016);
        assert_eq!(first, Some(Remark::FirstBoss(EnemyKind::Spider)));
        assert_eq!(spoken(&world).1, boss_line(EnemyKind::Spider));
        for bubble in world.query_mut::<&mut SpeechBubble>() {
            bubble.remaining = 0.0;
        }
        assert_eq!(chat(&mut world, PAUSE_SECONDS), Some(Remark::Depth(2)));
        assert_eq!(spoken(&world).1, remark_line(Remark::Depth(2)));

        let rest = hear_all(&mut world, 12);
        assert!(!rest.contains(&remark_line(Remark::Depth(2))), "{rest:?}");
        assert!(!rest.contains(&boss_line(EnemyKind::Spider)), "{rest:?}");
    }

    #[test]
    fn every_boss_has_its_own_first_kill_line() {
        let bosses: Vec<EnemyKind> = EnemyKind::ALL
            .into_iter()
            .filter(|kind| kind.is_boss())
            .collect();
        let lines: std::collections::BTreeSet<&str> =
            bosses.iter().map(|boss| boss_line(*boss)).collect();

        assert_eq!(lines.len(), bosses.len());
        assert!(!lines.contains(boss_line(EnemyKind::Rat)));
    }

    #[test]
    fn each_milestone_has_a_line_of_its_own() {
        use crate::progress::remarks::{DEATH_MILESTONES, DEPTH_MILESTONES, KILL_MILESTONES};
        let mut lines: Vec<&str> = Vec::new();
        lines.extend(DEATH_MILESTONES.map(|n| remark_line(Remark::Deaths(n))));
        lines.extend(KILL_MILESTONES.map(|n| remark_line(Remark::Kills(n))));
        lines.extend(DEPTH_MILESTONES.map(|n| remark_line(Remark::Depth(n))));
        lines.extend((1..=4).map(|n| remark_line(Remark::Win(n))));
        let unique: std::collections::BTreeSet<&str> = lines.iter().copied().collect();

        assert_eq!(unique.len(), lines.len(), "{lines:?}");
        assert_eq!(
            remark_line(Remark::Win(1)),
            "You reached the bottom. Now we can begin."
        );
        assert_ne!(
            remark_line(Remark::Challenge(ChallengeId::Bane)),
            remark_line(Remark::Challenge(ChallengeId::Horde))
        );
    }

    #[test]
    fn the_minimal_bubble_wraps_the_text_with_an_edge_and_a_small_tail() {
        let mut world = World::new();
        crate::prefabs::minimal_speech(&mut world, Vec2::new(500.0, 300.0));
        for bubble in world.query_mut::<&mut SpeechBubble>() {
            bubble.message = "Hello";
        }

        fit_speech(&mut world, &Fonts::default());

        let bubble = world
            .query::<(&SpeechBubble, &Size)>()
            .iter()
            .next()
            .map(|(_, size)| size.0)
            .unwrap_or_default();
        assert!(bubble.y >= PAD_Y * 2.0 && bubble.x >= PAD_X * 2.0);
        let part = |wanted: SpeechPart| {
            world
                .query::<(&SpeechPart, &Position, &Size)>()
                .iter()
                .find(|(part, _, _)| **part == wanted)
                .map(|(_, position, size)| (position.0, size.0))
                .unwrap_or_default()
        };
        let (frame_at, frame) = part(SpeechPart::Frame);
        assert_eq!(frame, bubble + Vec2::splat(BORDER * 2.0));
        let (tail_at, tail) = part(SpeechPart::Tail);
        assert_eq!(tail, Vec2::splat(TAIL));
        assert!(tail_at.y > frame_at.y + bubble.y / 2.0);
    }

    #[test]
    fn the_bubble_parts_hide_and_show_together() {
        let mut world = World::new();
        crate::prefabs::minimal_speech(&mut world, Vec2::new(500.0, 300.0));
        let hidden = |world: &World| world.query::<&crate::components::Hidden>().iter().count();
        crate::systems::shop::tick_speech(&mut world, 0.016);
        assert_eq!(hidden(&world), 4);

        crate::systems::shop::say(&mut world, "Hi");
        crate::systems::shop::tick_speech(&mut world, 0.016);
        assert_eq!(hidden(&world), 0);
    }
}
