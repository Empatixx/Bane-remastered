use glam::Vec2;

use super::sound::{AudioCommand, Hearing, Placement, SoundEffect};

/// The furthest a world sound leans to one speaker; it never leaves the other one silent.
const PAN_WIDTH: f32 = 0.45;
/// How long a played world sound counts against the limit of its kind.
const THROTTLE_WINDOW: f32 = 0.12;
/// How many copies of one world sound may start within the window.
const THROTTLE_LIMIT: usize = 3;

/// Places a sound made at `source` for a listener standing at `listener`, or `None` when it
/// is too far away to be heard.
pub(crate) fn place(listener: Vec2, source: Vec2, hearing: Hearing) -> Option<Placement> {
    let offset = source - listener;
    let distance = offset.length();
    if distance >= hearing.silent {
        return None;
    }
    let fade = ((distance - hearing.full) / (hearing.silent - hearing.full)).clamp(0.0, 1.0);
    let volume = (1.0 - fade) * (1.0 - fade);
    let pan = (offset.x / (hearing.silent / 2.0)).clamp(-1.0, 1.0) * PAN_WIDTH;
    Some(Placement { volume, pan })
}

/// Keeps a crowd of identical enemies from stacking dozens of the same sound at once.
#[derive(Debug, Default)]
pub(crate) struct Throttle {
    played: Vec<(SoundEffect, f32)>,
}

impl Throttle {
    /// Whether another copy of the sound may start at `now`; counts it when it may.
    pub(crate) fn admit(&mut self, effect: SoundEffect, now: f32) -> bool {
        self.played
            .retain(|(_, at)| now - *at < THROTTLE_WINDOW && *at <= now);
        let playing = self.played.iter().filter(|(e, _)| *e == effect).count();
        if playing >= THROTTLE_LIMIT {
            return false;
        }
        self.played.push((effect, now));
        true
    }
}

/// Turns the world sounds of a step into placed ones heard from `listener`, nearest first,
/// dropping those out of earshot or over the throttle. Other commands pass through untouched.
pub(crate) fn resolve(
    commands: Vec<AudioCommand>,
    listener: Option<Vec2>,
    throttle: &mut Throttle,
    now: f32,
) -> Vec<AudioCommand> {
    let mut resolved = Vec::with_capacity(commands.len());
    let mut spatial = Vec::new();
    for command in commands {
        match command {
            AudioCommand::At(effect, at) => spatial.push((effect, at)),
            other => resolved.push(other),
        }
    }
    let Some(listener) = listener else {
        return resolved;
    };
    spatial.sort_by(|a, b| {
        a.1.distance_squared(listener)
            .total_cmp(&b.1.distance_squared(listener))
    });
    for (effect, at) in spatial {
        let Some(placement) = place(listener, at, effect.hearing()) else {
            continue;
        };
        if throttle.admit(effect, now) {
            resolved.push(AudioCommand::Placed(effect, placement));
        }
    }
    resolved
}

#[cfg(test)]
mod tests {
    use super::*;

    const TILE: f32 = crate::tilemap::TILE_SIZE;

    fn volume_at(tiles: f32) -> f32 {
        place(
            Vec2::ZERO,
            Vec2::new(tiles * TILE, 0.0),
            SoundEffect::SlimeSmash.hearing(),
        )
        .map_or(0.0, |placement| placement.volume)
    }

    #[test]
    fn an_enemy_sound_is_full_up_close_and_fades_to_silence_far_away() {
        assert_eq!(volume_at(0.0), 1.0);
        assert_eq!(volume_at(3.0), 1.0);
        assert!(volume_at(6.0) < 1.0 && volume_at(6.0) > volume_at(10.0));
        assert!(volume_at(13.5) > 0.0);
        assert_eq!(volume_at(14.0), 0.0);
        assert_eq!(volume_at(30.0), 0.0);
    }

    #[test]
    fn a_boss_carries_further_than_a_common_enemy() {
        let far = Vec2::new(16.0 * TILE, 0.0);
        assert!(place(Vec2::ZERO, far, SoundEffect::SlimeSmash.hearing()).is_none());
        assert!(place(Vec2::ZERO, far, SoundEffect::OgreRoar.hearing()).is_some());
    }

    #[test]
    fn a_sound_to_the_right_leans_right_and_one_to_the_left_leans_left() {
        let hearing = SoundEffect::SlimeSmash.hearing();
        let right = place(Vec2::ZERO, Vec2::new(5.0 * TILE, 0.0), hearing).map(|p| p.pan);
        let left = place(Vec2::ZERO, Vec2::new(-5.0 * TILE, 0.0), hearing).map(|p| p.pan);
        let above = place(Vec2::ZERO, Vec2::new(0.0, 5.0 * TILE), hearing).map(|p| p.pan);
        assert!(right.is_some_and(|pan| pan > 0.0 && pan <= PAN_WIDTH), "{right:?}");
        assert!(left.is_some_and(|pan| (-PAN_WIDTH..0.0).contains(&pan)));
        assert_eq!(above, Some(0.0));
    }

    #[test]
    fn the_throttle_lets_only_a_few_copies_of_a_sound_through_at_once() {
        let mut throttle = Throttle::default();
        let admitted = (0..10)
            .filter(|_| throttle.admit(SoundEffect::SlimeSmash, 1.0))
            .count();
        assert_eq!(admitted, THROTTLE_LIMIT);
        assert!(throttle.admit(SoundEffect::GhostDash, 1.0));
        assert!(throttle.admit(SoundEffect::SlimeSmash, 1.0 + THROTTLE_WINDOW));
    }

    #[test]
    fn resolving_places_the_nearest_world_sounds_and_keeps_the_rest() {
        let mut throttle = Throttle::default();
        let mut commands = vec![AudioCommand::Effect(SoundEffect::Coin)];
        for tiles in [12.0, 1.0, 40.0, 2.0, 5.0, 4.0] {
            commands.push(AudioCommand::At(
                SoundEffect::SlimeSmash,
                Vec2::new(tiles * TILE, 0.0),
            ));
        }

        let resolved = resolve(commands, Some(Vec2::ZERO), &mut throttle, 0.0);

        assert_eq!(resolved[0], AudioCommand::Effect(SoundEffect::Coin));
        let volumes: Vec<f32> = resolved[1..]
            .iter()
            .filter_map(|command| match command {
                AudioCommand::Placed(_, placement) => Some(placement.volume),
                _ => None,
            })
            .collect();
        assert_eq!(volumes.len(), THROTTLE_LIMIT);
        assert_eq!(volumes[0], 1.0);
        assert_eq!(volumes[1], 1.0);
        assert!(volumes[2] < 1.0);
    }

    #[test]
    fn world_sounds_are_dropped_when_nobody_is_listening() {
        let mut throttle = Throttle::default();
        let commands = vec![AudioCommand::At(SoundEffect::OgreRoar, Vec2::ZERO)];
        assert!(resolve(commands, None, &mut throttle, 0.0).is_empty());
    }
}
