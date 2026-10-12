use hecs::World;

use crate::components::{Chatty, Player, Position, SpeechBubble, Wallet};
use crate::systems::shop::say;

pub(crate) const HEARING_RANGE: f32 = 360.0;
pub(crate) const LEAVING_RANGE: f32 = 800.0;
pub(crate) const PAUSE_SECONDS: f32 = 7.0;
pub(crate) const RICH_COINS: i32 = 100;

const GREETINGS: [&str; 2] = ["Welcome back!", "Ah, my test subject!"];
const HINTS: [&str; 6] = [
    "Upgrades, upgrades!",
    "Dodging is science!",
    "Slimes spit. Beware!",
    "Pots hide coins!",
    "Go on, test my work!",
    "Every run teaches us.",
];
const BROKE: &str = "No coins? Go loot!";
const RICH: &str = "Rich! Buy something!";
pub(crate) const PURCHASE: &str = "Excellent choice!";

fn pick(chatty: &mut Chatty, coins: i32) -> &'static str {
    let line = chatty.line;
    chatty.line += 1;
    if !chatty.greeted {
        chatty.greeted = true;
        return GREETINGS[line % GREETINGS.len()];
    }
    match line % 3 {
        1 if coins <= 0 => BROKE,
        2 if coins >= RICH_COINS => RICH,
        _ => HINTS[line % HINTS.len()],
    }
}

pub(crate) fn chat(world: &mut World, dt: f32) {
    let Some((player, coins)) = world
        .query::<(&Position, Option<&Wallet>)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(position, wallet)| (position.0, wallet.map_or(0, |wallet| wallet.coins)))
    else {
        return;
    };
    let talking = world
        .query::<&SpeechBubble>()
        .iter()
        .any(|bubble| bubble.remaining > 0.0);
    let mut line = None;
    for (position, chatty) in world.query_mut::<(&Position, &mut Chatty)>() {
        chatty.cooldown = (chatty.cooldown - dt).max(0.0);
        let distance = position.0.distance(player);
        if distance > LEAVING_RANGE {
            chatty.greeted = false;
        }
        if distance <= HEARING_RANGE && chatty.cooldown <= 0.0 && !talking {
            chatty.cooldown = PAUSE_SECONDS;
            line = Some(pick(chatty, coins));
        }
    }
    if let Some(line) = line {
        say(world, line);
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
}
