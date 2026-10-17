mod sound;
mod spatial;

use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::time::Duration;

use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle};
use kira::track::{TrackBuilder, TrackHandle};
use kira::{AudioManager, AudioManagerSettings, Decibels, DefaultBackend, Panning, Tween};

pub(crate) use sound::{AudioCommand, Placement, SoundEffect, Soundtrack};
pub(crate) use spatial::{Throttle, resolve};

const CROSSFADE: Duration = Duration::from_secs(2);
const VOLUME_TWEEN: Duration = Duration::from_millis(50);
/// How far a world sound's pitch may wander either way, so a crowd does not sound robotic.
const PITCH_SPREAD: f64 = 0.07;
const CENTERED: Placement = Placement {
    volume: 1.0,
    pan: 0.0,
};

struct Backend {
    _manager: AudioManager<DefaultBackend>,
    music: TrackHandle,
    effects: TrackHandle,
    sounds: HashMap<&'static str, StaticSoundData>,
    current: Option<(Soundtrack, StaticSoundHandle)>,
}

pub(crate) struct Audio {
    backend: Option<Backend>,
    volume: (f32, f32, f32),
}

impl fmt::Debug for Audio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Audio")
            .field("available", &self.backend.is_some())
            .finish()
    }
}

impl Audio {
    pub(crate) fn new() -> Self {
        Self {
            backend: Backend::start(),
            volume: (1.0, 0.5, 0.5),
        }
    }

    pub(crate) fn silent() -> Self {
        Self {
            backend: None,
            volume: (1.0, 0.5, 0.5),
        }
    }

    pub(crate) fn files() -> Vec<&'static str> {
        let effects = SoundEffect::ALL
            .iter()
            .flat_map(|effect| effect.files().iter().copied());
        let music = Soundtrack::ALL.iter().map(|track| track.file());
        music.chain(effects).collect()
    }

    pub(crate) fn preload(&mut self, file: &'static str) {
        let Some(backend) = &mut self.backend else {
            return;
        };
        backend.load(file);
    }

    pub(crate) fn play(&mut self, command: AudioCommand) {
        let Some(backend) = &mut self.backend else {
            return;
        };
        match command {
            AudioCommand::Effect(effect) => backend.play_effect(effect),
            AudioCommand::Music(track) => backend.play_music(track),
            AudioCommand::At(effect, _) => backend.play_placed(effect, CENTERED),
            AudioCommand::Placed(effect, placement) => backend.play_placed(effect, placement),
        }
    }

    pub(crate) fn set_volume(&mut self, overall: f32, music: f32, effects: f32) {
        self.volume = (overall, music, effects);
        let Some(backend) = &mut self.backend else {
            return;
        };
        let tween = Tween {
            duration: VOLUME_TWEEN,
            ..Tween::default()
        };
        backend.music.set_volume(decibels(overall * music), tween);
        backend
            .effects
            .set_volume(decibels(overall * effects), tween);
    }
}

pub(crate) fn decibels(amplitude: f32) -> Decibels {
    if amplitude <= 0.0 {
        Decibels::SILENCE
    } else {
        Decibels((20.0 * amplitude.log10()).max(Decibels::SILENCE.0))
    }
}

impl Backend {
    fn start() -> Option<Self> {
        let mut manager = match AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
        {
            Ok(manager) => manager,
            Err(error) => {
                tracing::warn!(%error, "audio is not available");
                return None;
            }
        };
        let music = manager.add_sub_track(TrackBuilder::new()).ok()?;
        let effects = manager.add_sub_track(TrackBuilder::new()).ok()?;
        Some(Self {
            _manager: manager,
            music,
            effects,
            sounds: HashMap::new(),
            current: None,
        })
    }

    fn load(&mut self, file: &'static str) -> Option<StaticSoundData> {
        if let Some(sound) = self.sounds.get(file) {
            return Some(sound.clone());
        }
        match StaticSoundData::from_file(audio_root().join(file)) {
            Ok(sound) => {
                self.sounds.insert(file, sound.clone());
                Some(sound)
            }
            Err(error) => {
                tracing::warn!(file, %error, "cannot load the sound");
                None
            }
        }
    }

    fn pick(&mut self, effect: SoundEffect) -> Option<StaticSoundData> {
        let files = effect.files();
        let file = files[(crate::rng::cosmetic() * files.len() as f32) as usize % files.len()];
        self.load(file)
    }

    fn play_effect(&mut self, effect: SoundEffect) {
        let Some(sound) = self.pick(effect) else {
            return;
        };
        if let Err(error) = self.effects.play(sound) {
            tracing::warn!(%error, "cannot play the effect");
        }
    }

    fn play_placed(&mut self, effect: SoundEffect, placement: Placement) {
        let Some(sound) = self.pick(effect) else {
            return;
        };
        let rate = 1.0 + (f64::from(crate::rng::cosmetic()) - 0.5) * 2.0 * PITCH_SPREAD;
        let sound = sound
            .volume(decibels(placement.volume))
            .panning(Panning(placement.pan))
            .playback_rate(rate);
        if let Err(error) = self.effects.play(sound) {
            tracing::warn!(%error, "cannot play the effect");
        }
    }

    fn play_music(&mut self, track: Soundtrack) {
        if self
            .current
            .as_ref()
            .is_some_and(|(current, _)| *current == track)
        {
            return;
        }
        let Some(sound) = self.load(track.file()) else {
            return;
        };
        let fade = Tween {
            duration: CROSSFADE,
            ..Tween::default()
        };
        let first = self.current.is_none();
        if let Some((_, mut old)) = self.current.take() {
            old.stop(fade);
        }
        let sound = sound
            .loop_region(..)
            .fade_in_tween(if first { None } else { Some(fade) });
        match self.music.play(sound) {
            Ok(handle) => self.current = Some((track, handle)),
            Err(error) => tracing::warn!(%error, "cannot play the soundtrack"),
        }
    }
}

fn audio_root() -> PathBuf {
    crate::storage::assets_root().join("audio")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silence_and_full_volume_map_to_the_decibel_extremes() {
        assert_eq!(decibels(0.0), Decibels::SILENCE);
        assert!((decibels(1.0).0).abs() < 0.001);
        assert!((decibels(0.5).0 + 6.02).abs() < 0.05);
    }

    #[test]
    fn every_sound_file_listed_in_the_code_exists_on_disk() {
        for file in Audio::files() {
            assert!(audio_root().join(file).is_file(), "missing {file}");
        }
    }

    #[test]
    fn a_silent_audio_ignores_all_commands() {
        let mut audio = Audio::silent();
        audio.play(AudioCommand::Effect(SoundEffect::Click));
        audio.play(AudioCommand::Music(Soundtrack::MainMenu));
        audio.play(AudioCommand::At(SoundEffect::OgreRoar, glam::Vec2::ZERO));
        audio.play(AudioCommand::Placed(SoundEffect::SlimeSmash, CENTERED));
        audio.set_volume(1.0, 0.5, 0.5);
        audio.preload("menuclick.ogg");
    }
}

#[cfg(test)]
mod cinematic_track {
    #[test]
    fn the_cinematic_track_decodes_to_its_end() {
        let sound = kira::sound::static_sound::StaticSoundData::from_file(super::audio_root().join(super::Soundtrack::Cinematic.file()))
            .expect("the cinematic track loads");
        assert!(sound.duration().as_secs_f32() > 49.0, "{:?}", sound.duration());
    }

}
