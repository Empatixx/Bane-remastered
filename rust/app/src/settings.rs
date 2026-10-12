use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::controls::Bindings;
use crate::viewport::TARGET;

const FALLBACK_RESOLUTIONS: [(u32, u32); 4] = [(960, 540), (1280, 720), (1600, 900), (1920, 1080)];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GameSettings {
    pub(crate) resolution_width: u32,
    pub(crate) resolution_height: u32,
    pub(crate) fullscreen: bool,
    pub(crate) vsync: bool,
    pub(crate) overall_volume: f32,
    pub(crate) music_volume: f32,
    pub(crate) effects_volume: f32,
    #[serde(default)]
    pub(crate) controls: Bindings,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            resolution_width: TARGET.x as u32 * 2,
            resolution_height: TARGET.y as u32 * 2,
            fullscreen: false,
            vsync: true,
            overall_volume: 1.0,
            music_volume: 0.5,
            effects_volume: 0.5,
            controls: Bindings::default(),
        }
    }
}

impl GameSettings {
    fn sanitized(mut self) -> Self {
        let defaults = Self::default();
        if self.resolution_width == 0 || self.resolution_height == 0 {
            self.resolution_width = defaults.resolution_width;
            self.resolution_height = defaults.resolution_height;
        }
        self.overall_volume = self.overall_volume.clamp(0.0, 1.0);
        self.music_volume = self.music_volume.clamp(0.0, 1.0);
        self.effects_volume = self.effects_volume.clamp(0.0, 1.0);
        self
    }

    pub(crate) fn resolution(&self) -> (u32, u32) {
        (self.resolution_width, self.resolution_height)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Change {
    Fullscreen(bool),
    Resolution(u32, u32),
    Vsync(bool),
    Volume {
        overall: f32,
        music: f32,
        effects: f32,
    },
    Controls(Bindings),
}

#[derive(Debug)]
pub(crate) struct Settings {
    applied: GameSettings,
    pending: GameSettings,
    resolutions: Vec<(u32, u32)>,
    path: Option<PathBuf>,
    changes: Vec<Change>,
}

impl Settings {
    pub(crate) fn load(path: &Path) -> Self {
        let applied = match fs::read_to_string(path) {
            Ok(text) => match serde_json::from_str::<GameSettings>(&text) {
                Ok(settings) => {
                    tracing::info!(path = %path.display(), "loaded settings");
                    settings.sanitized()
                }
                Err(error) => {
                    tracing::error!(%error, "failed to parse settings, using defaults");
                    GameSettings::default()
                }
            },
            Err(_) => {
                tracing::info!("creating default settings");
                GameSettings::default()
            }
        };
        Self::with(applied, Some(path.to_path_buf()))
    }

    fn with(applied: GameSettings, path: Option<PathBuf>) -> Self {
        Self {
            pending: applied.clone(),
            applied,
            resolutions: FALLBACK_RESOLUTIONS.to_vec(),
            path,
            changes: Vec::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn in_memory() -> Self {
        Self::with(GameSettings::default(), None)
    }

    pub(crate) fn pending(&self) -> &GameSettings {
        &self.pending
    }

    pub(crate) fn has_changes(&self) -> bool {
        self.applied != self.pending
    }

    pub(crate) fn pending_mut(&mut self) -> &mut GameSettings {
        &mut self.pending
    }

    pub(crate) fn set_resolutions(&mut self, mut resolutions: Vec<(u32, u32)>) {
        resolutions.sort_unstable();
        resolutions.dedup();
        if resolutions.is_empty() {
            resolutions = FALLBACK_RESOLUTIONS.to_vec();
        }
        let current = self.pending.resolution();
        if !resolutions.contains(&current) {
            resolutions.push(current);
            resolutions.sort_unstable();
        }
        self.resolutions = resolutions;
    }

    pub(crate) fn next_resolution(&mut self) {
        self.step_resolution(1);
    }

    pub(crate) fn previous_resolution(&mut self) {
        self.step_resolution(self.resolutions.len() - 1);
    }

    fn step_resolution(&mut self, step: usize) {
        let current = self
            .resolutions
            .iter()
            .position(|resolution| *resolution == self.pending.resolution())
            .unwrap_or(0);
        let (width, height) = self.resolutions[(current + step) % self.resolutions.len()];
        self.pending.resolution_width = width;
        self.pending.resolution_height = height;
    }

    pub(crate) fn startup_changes(&mut self) {
        let settings = &self.applied;
        self.changes.push(Change::Fullscreen(settings.fullscreen));
        self.changes.push(Change::Resolution(
            settings.resolution_width,
            settings.resolution_height,
        ));
        self.changes.push(Change::Vsync(settings.vsync));
        self.changes.push(Change::Volume {
            overall: settings.overall_volume,
            music: settings.music_volume,
            effects: settings.effects_volume,
        });
        self.changes.push(Change::Controls(settings.controls));
    }

    pub(crate) fn apply(&mut self) {
        let (old, new) = (&self.applied, &self.pending);
        if old.fullscreen != new.fullscreen {
            self.changes.push(Change::Fullscreen(new.fullscreen));
        }
        if old.resolution() != new.resolution() {
            self.changes.push(Change::Resolution(
                new.resolution_width,
                new.resolution_height,
            ));
        }
        if old.vsync != new.vsync {
            self.changes.push(Change::Vsync(new.vsync));
        }
        if old.overall_volume != new.overall_volume
            || old.music_volume != new.music_volume
            || old.effects_volume != new.effects_volume
        {
            self.changes.push(Change::Volume {
                overall: new.overall_volume,
                music: new.music_volume,
                effects: new.effects_volume,
            });
        }
        if old.controls != new.controls {
            self.changes.push(Change::Controls(new.controls));
        }
        self.applied = self.pending.clone();
        self.save();
    }

    pub(crate) fn revert(&mut self) {
        self.pending = self.applied.clone();
    }

    pub(crate) fn take_changes(&mut self) -> Vec<Change> {
        std::mem::take(&mut self.changes)
    }

    fn save(&self) {
        let Some(path) = &self.path else {
            return;
        };
        match serde_json::to_string_pretty(&self.applied) {
            Ok(text) => {
                if let Err(error) = fs::write(path, text) {
                    tracing::error!(%error, "failed to save settings");
                }
            }
            Err(error) => tracing::error!(%error, "failed to serialize settings"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_the_java_game() {
        let settings = GameSettings::default();
        assert_eq!(settings.resolution(), (1920, 1080));
        assert!(!settings.fullscreen);
        assert!(settings.vsync);
        assert_eq!(
            (
                settings.overall_volume,
                settings.music_volume,
                settings.effects_volume
            ),
            (1.0, 0.5, 0.5)
        );
    }

    #[test]
    fn json_uses_the_same_field_names_as_the_java_game() {
        let json = serde_json::to_string(&GameSettings::default()).expect("serializes");
        for name in [
            "resolutionWidth",
            "resolutionHeight",
            "fullscreen",
            "vsync",
            "overallVolume",
            "musicVolume",
            "effectsVolume",
        ] {
            assert!(json.contains(name), "{name} missing in {json}");
        }
    }

    #[test]
    fn apply_reports_only_what_changed_and_revert_restores() {
        let mut settings = Settings::in_memory();
        settings.pending_mut().vsync = false;
        settings.pending_mut().music_volume = 0.9;

        settings.apply();
        let changes = settings.take_changes();
        assert_eq!(changes.len(), 2);
        assert!(changes.contains(&Change::Vsync(false)));

        settings.pending_mut().fullscreen = true;
        settings.revert();
        assert!(!settings.pending().fullscreen);
        assert!(settings.take_changes().is_empty());
    }

    #[test]
    fn resolutions_wrap_around_in_both_directions() {
        let mut settings = Settings::in_memory();
        settings.set_resolutions(vec![(800, 600), (1920, 1080), (1280, 720)]);
        assert_eq!(settings.pending().resolution(), (1920, 1080));

        settings.next_resolution();
        assert_eq!(settings.pending().resolution(), (800, 600));
        settings.previous_resolution();
        assert_eq!(settings.pending().resolution(), (1920, 1080));
        settings.previous_resolution();
        assert_eq!(settings.pending().resolution(), (1280, 720));
    }

    #[test]
    fn invalid_values_from_a_file_are_repaired() {
        let broken = GameSettings {
            resolution_width: 0,
            overall_volume: 7.0,
            ..GameSettings::default()
        }
        .sanitized();
        assert_eq!(broken.resolution(), (1920, 1080));
        assert_eq!(broken.overall_volume, 1.0);
    }

    #[test]
    fn startup_applies_every_setting() {
        let mut settings = Settings::in_memory();
        settings.startup_changes();
        assert_eq!(settings.take_changes().len(), 5);
    }

    #[test]
    fn applying_new_key_bindings_reports_them_and_old_files_without_keys_still_load() {
        use crate::controls::Action;
        use winit::keyboard::KeyCode;

        let mut settings = Settings::in_memory();
        settings
            .pending_mut()
            .controls
            .rebind(Action::Reload, KeyCode::KeyX);
        settings.apply();
        let changes = settings.take_changes();
        assert!(changes
            .iter()
            .any(|change| matches!(change, Change::Controls(bindings) if bindings.key(Action::Reload) == KeyCode::KeyX)));

        let old: GameSettings = serde_json::from_str(
            r#"{"resolutionWidth":1280,"resolutionHeight":720,"fullscreen":false,"vsync":true,"overallVolume":1.0,"musicVolume":0.5,"effectsVolume":0.5}"#,
        )
        .expect("an old settings file parses");
        assert_eq!(old.controls, crate::controls::Bindings::default());
    }

    #[test]
    fn applied_settings_are_saved_to_disk_and_loaded_again() {
        let path = std::env::temp_dir().join(format!("bane-settings-{}.json", std::process::id()));
        let _ = fs::remove_file(&path);

        let mut settings = Settings::load(&path);
        assert_eq!(settings.pending(), &GameSettings::default());
        settings.pending_mut().fullscreen = true;
        settings.pending_mut().overall_volume = 0.25;
        settings.apply();

        let reloaded = Settings::load(&path);
        let _ = fs::remove_file(&path);
        assert!(reloaded.pending().fullscreen);
        assert_eq!(reloaded.pending().overall_volume, 0.25);
    }

    #[test]
    fn a_corrupted_file_falls_back_to_defaults() {
        let path = std::env::temp_dir().join(format!("bane-broken-{}.json", std::process::id()));
        fs::write(&path, "{ not json").expect("writes the file");

        let settings = Settings::load(&path);
        let _ = fs::remove_file(&path);
        assert_eq!(settings.pending(), &GameSettings::default());
    }

    #[test]
    fn changes_are_reported_until_they_are_applied_or_reverted() {
        let mut settings = Settings::in_memory();
        assert!(!settings.has_changes());

        settings.pending_mut().vsync = !settings.pending().vsync;
        assert!(settings.has_changes());
        settings.revert();
        assert!(!settings.has_changes());

        settings.pending_mut().fullscreen = true;
        settings.apply();
        assert!(!settings.has_changes());
    }
}
