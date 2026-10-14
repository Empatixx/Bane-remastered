use hecs::{Entity, World};

use crate::assets::Assets;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{PauseUi, UiAction};
use crate::input::Input;
use crate::prefabs;
use crate::settings::Settings;
use crate::systems;

use super::SettingsPanel;

#[derive(Debug, Default)]
pub(crate) struct Pause {
    active: bool,
    settings: SettingsPanel,
}

impl Pause {
    pub(crate) fn is_active(&self) -> bool {
        self.active
    }

    #[cfg(test)]
    pub(crate) fn settings_open(&self) -> bool {
        self.settings.is_open()
    }

    pub(crate) fn toggle(
        &mut self,
        world: &mut World,
        assets: &Assets,
        buttons: &[(&str, UiAction)],
    ) {
        self.active = !self.active;
        if self.active {
            prefabs::pause_ui(world, &assets.textures, &assets.fonts, buttons);
        } else {
            if self.settings.is_open() {
                self.settings.close(world);
            }
            for entity in pause_entities(world) {
                let _ = world.despawn(entity);
            }
        }
    }

    /// Escape steps back one level: out of the settings back to the pause menu, otherwise it
    /// opens or closes the pause menu. While a key slot waits for a new key, the settings
    /// panel itself takes the Escape to cancel the rebinding.
    pub(crate) fn escape(
        &mut self,
        world: &mut World,
        assets: &Assets,
        buttons: &[(&str, UiAction)],
    ) {
        if self.settings.is_open() {
            if !SettingsPanel::is_listening(world) {
                self.settings.close(world);
            }
        } else {
            self.toggle(world, assets, buttons);
        }
    }

    pub(crate) fn update(
        &mut self,
        world: &mut World,
        input: &Input,
        assets: &Assets,
        settings: &mut Settings,
        sounds: &mut Vec<AudioCommand>,
    ) -> Option<UiAction> {
        if !self.active {
            return None;
        }
        if self.settings.is_open() {
            if self.settings.update(world, input, settings) {
                sounds.push(AudioCommand::Effect(SoundEffect::Click));
            }
            return None;
        }
        let action = systems::update_text_buttons(world, input)?;
        sounds.push(AudioCommand::Effect(SoundEffect::Click));
        if action == UiAction::Settings {
            let menu = pause_entities(world);
            self.settings.open(world, assets, &menu);
            return None;
        }
        Some(action)
    }
}

fn pause_entities(world: &World) -> Vec<Entity> {
    world.query::<Entity>().with::<&PauseUi>().iter().collect()
}

#[cfg(test)]
mod tests {
    use glam::Vec2;
    use winit::event::{ElementState, MouseButton};
    use winit::keyboard::KeyCode;

    use crate::components::{
        Hidden, Hitbox, KeyBindSlot, Overlay, Position, SettingsUi, Text, TextButton,
    };
    use crate::controls::Action;

    use super::*;

    fn click(at: Vec2) -> Input {
        let mut input = Input::default();
        input.set_mouse(at);
        input.handle_click(MouseButton::Left, ElementState::Pressed);
        input
    }

    fn button_at(world: &World, label: &str) -> Vec2 {
        world
            .query::<(&Position, &Text)>()
            .with::<&TextButton>()
            .iter()
            .find(|(_, text)| text.content == label)
            .map(|(position, _)| position.0)
            .expect("the button is on the pause menu")
    }

    fn paused_with_settings_open() -> (World, Pause, Assets, Settings) {
        let assets = Assets::new().expect("the embedded tileset is valid");
        let mut world = World::new();
        let mut pause = Pause::default();
        let mut settings = Settings::in_memory();
        pause.toggle(&mut world, &assets, &prefabs::PAUSE_GAME);
        let at = button_at(&world, "Settings");
        let mut sounds = Vec::new();
        let action = pause.update(&mut world, &click(at), &assets, &mut settings, &mut sounds);
        assert_eq!(action, None);
        assert!(pause.settings_open());
        (world, pause, assets, settings)
    }

    #[test]
    fn settings_opens_the_settings_panel_over_the_hidden_pause_menu() {
        let (world, pause, _assets, _settings) = paused_with_settings_open();

        assert!(pause.is_active());
        assert!(world.query::<&SettingsUi>().iter().count() > 0);
        assert_eq!(
            world.query::<&SettingsUi>().without::<&Overlay>().iter().count(),
            0,
            "the panel draws above the blurred, paused game"
        );
        assert_eq!(
            world.query::<&PauseUi>().without::<&Hidden>().iter().count(),
            0
        );
    }

    #[test]
    fn escape_closes_the_settings_and_brings_the_pause_menu_back() {
        let (mut world, mut pause, assets, _settings) = paused_with_settings_open();

        pause.escape(&mut world, &assets, &prefabs::PAUSE_GAME);

        assert!(pause.is_active());
        assert!(!pause.settings_open());
        assert_eq!(world.query::<&SettingsUi>().iter().count(), 0);
        assert_eq!(world.query::<&PauseUi>().with::<&Hidden>().iter().count(), 0);
    }

    #[test]
    fn the_settings_exit_button_returns_to_the_pause_menu() {
        let (mut world, mut pause, assets, mut settings) = paused_with_settings_open();
        let mut sounds = Vec::new();

        let exit = click(Vec2::new(800.0, 322.0));
        pause.update(&mut world, &exit, &assets, &mut settings, &mut sounds);

        assert!(pause.is_active());
        assert!(!pause.settings_open());
        assert_eq!(world.query::<&PauseUi>().with::<&Hidden>().iter().count(), 0);
    }

    #[test]
    fn keys_can_be_rebound_from_the_pause_menu_settings() {
        let (mut world, mut pause, assets, mut settings) = paused_with_settings_open();
        let mut sounds = Vec::new();
        let controls_tab = click(Vec2::new(560.0 + 75.0, 128.0 + 24.0));
        pause.update(&mut world, &controls_tab, &assets, &mut settings, &mut sounds);
        let slot = world
            .query::<(&Position, &KeyBindSlot)>()
            .iter()
            .find(|(_, slot)| slot.action == Action::Interact)
            .map(|(position, _)| position.0)
            .expect("the interact slot");
        pause.update(&mut world, &click(slot), &assets, &mut settings, &mut sounds);
        assert!(SettingsPanel::is_listening(&world));

        pause.escape(&mut world, &assets, &prefabs::PAUSE_GAME);
        assert!(pause.settings_open(), "escape only cancels the waiting slot");
        let mut key = Input::default();
        key.press(KeyCode::KeyX);
        pause.update(&mut world, &key, &assets, &mut settings, &mut sounds);

        assert_eq!(settings.pending().controls.key(Action::Interact), KeyCode::KeyX);
    }

    #[test]
    fn resuming_closes_an_open_settings_panel_too() {
        let (mut world, mut pause, assets, _settings) = paused_with_settings_open();

        pause.toggle(&mut world, &assets, &prefabs::PAUSE_GAME);

        assert!(!pause.is_active());
        assert_eq!(world.query::<&SettingsUi>().iter().count(), 0);
        assert_eq!(world.query::<&PauseUi>().iter().count(), 0);
    }

    #[test]
    fn every_pause_menu_button_fits_on_the_background_without_overlapping() {
        let assets = Assets::new().expect("the embedded tileset is valid");
        for buttons in [&prefabs::PAUSE_GAME[..], &prefabs::PAUSE_LOBBY[..]] {
            let mut world = World::new();
            let mut pause = Pause::default();
            pause.toggle(&mut world, &assets, buttons);
            let (center, size) = world
                .query::<(&Position, &crate::components::Size)>()
                .with::<&PauseUi>()
                .iter()
                .next()
                .map(|(position, size)| (position.0, size.0))
                .expect("the background");
            let (top, bottom) = (center.y - size.y / 2.0, center.y + size.y / 2.0);
            let mut boxes: Vec<(f32, f32)> = world
                .query::<(&Position, &Hitbox)>()
                .with::<&PauseUi>()
                .iter()
                .map(|(position, hitbox)| {
                    let start = position.0.y + hitbox.offset.y;
                    (start, start + hitbox.size.y)
                })
                .collect();
            assert_eq!(boxes.len(), buttons.len());
            boxes.sort_by(|a, b| a.0.total_cmp(&b.0));
            for (start, end) in &boxes {
                assert!(*start >= top && *end <= bottom, "{start}..{end} in {top}..{bottom}");
            }
            for pair in boxes.windows(2) {
                assert!(pair[0].1 <= pair[1].0, "{pair:?} overlap");
            }
        }
    }
}
