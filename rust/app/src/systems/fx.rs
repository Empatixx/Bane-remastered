use crate::audio::{AudioCommand, SoundEffect};

/// How hard an explosion shakes the screen.
pub(crate) const EXPLOSION_SHAKE: f32 = 0.8;
/// How hard the player's death blow shakes the screen.
pub(crate) const DEATH_SHAKE: f32 = 0.7;
/// How hard a hit on the player shakes the screen.
pub(crate) const HURT_SHAKE: f32 = 0.5;

/// What a scene's systems ask of the camera that is not an entity: the trauma of the hard hits
/// since the camera last took it. Owned by the scene, passed to the systems that shake.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub(crate) struct Fx {
    trauma: f32,
}

impl Fx {
    /// Adds a hit's trauma; the camera clamps the total when it takes it.
    pub(crate) fn shake(&mut self, trauma: f32) {
        self.trauma += trauma;
    }

    /// The trauma gathered so far, leaving none behind.
    pub(crate) fn take_trauma(&mut self) -> f32 {
        std::mem::take(&mut self.trauma)
    }
}

/// An explosion's bang, and the shake that comes with it.
pub(crate) fn explosion(sounds: &mut Vec<AudioCommand>, fx: &mut Fx) {
    sounds.push(AudioCommand::Effect(SoundEffect::Explosion));
    fx.shake(EXPLOSION_SHAKE);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trauma_adds_up_until_it_is_taken() {
        let mut fx = Fx::default();
        fx.shake(0.25);
        fx.shake(0.5);
        assert_eq!(fx.take_trauma(), 0.75);
        assert_eq!(fx.take_trauma(), 0.0);
    }

    #[test]
    fn an_explosion_bangs_and_shakes_as_hard_as_it_always_did() {
        let mut sounds = Vec::new();
        let mut fx = Fx::default();
        explosion(&mut sounds, &mut fx);
        assert_eq!(sounds, vec![AudioCommand::Effect(SoundEffect::Explosion)]);
        assert_eq!(fx.take_trauma().to_bits(), 0.8f32.to_bits());
    }
}
