use glam::Vec4;
use hecs::{Entity, World};

use winit::keyboard::KeyCode;

use crate::components::{
    Button, Checkbox, Hidden, KeyBindSlot, LabelOf, NeedsChanges, Position, ResolutionText,
    SettingsTabKind, Size, SliderHandle, SliderValue, Sprite, TabButton, TabOnly, Text, Toggle,
    Volume,
};
use crate::controls;
use crate::geometry;
use crate::input::Input;
use crate::palette;
use crate::settings::{GameSettings, Settings};

const ROW_COUNT: usize = 4;
const KEY_PROMPT: &str = "Press key";
const KEY_COLOR: Vec4 = palette::SALMON;
const KEY_ACTIVE_COLOR: Vec4 = Vec4::new(1.0, 1.0, 0.0, 1.0);
const LABEL_IDLE: Vec4 = palette::ORANGE;
const LABEL_DISABLED: Vec4 = Vec4::new(112.0 / 255.0, 100.0 / 255.0, 100.0 / 255.0, 1.0);
const DISABLED_ROW: usize = 2;
const SIDE_ROWS: usize = 3;

pub(crate) fn update_side_buttons(world: &mut World, input: &Input, dirty: bool) {
    let mouse = input.mouse();
    for (button, position, size, needs) in world
        .query::<(Entity, &Position, &Size, Option<&NeedsChanges>)>()
        .with::<&Button>()
        .iter()
        .map(|(entity, position, size, needs)| (entity, *position, *size, needs.is_some()))
        .collect::<Vec<_>>()
    {
        let disabled = needs && !dirty;
        if disabled {
            if let Ok(mut sprite) = world.get::<&mut Sprite>(button) {
                sprite.uv = Sprite::cell_uv(0, DISABLED_ROW, 1, SIDE_ROWS);
            }
        }
        let hovered = !disabled && geometry::contains(position.0, size.0, mouse);
        let label_color = if disabled {
            LABEL_DISABLED
        } else if hovered {
            KEY_ACTIVE_COLOR
        } else {
            LABEL_IDLE
        };
        let labels: Vec<Entity> = world
            .query::<(Entity, &LabelOf)>()
            .iter()
            .filter(|(_, label)| label.0 == button)
            .map(|(entity, _)| entity)
            .collect();
        for label in labels {
            if let Ok(mut text) = world.get::<&mut Text>(label) {
                text.color = label_color;
            }
        }
    }
}

pub(crate) fn update_settings_ui(
    world: &mut World,
    input: &Input,
    settings: &mut Settings,
    tab: &mut SettingsTabKind,
) {
    select_tab(world, input, tab);
    show_only(world, *tab);
    update_checkboxes(world, input, settings);
    update_sliders(world, input, settings);
    update_resolution_text(world, settings);
    update_key_binds(world, input, settings);
}

fn select_tab(world: &mut World, input: &Input, tab: &mut SettingsTabKind) {
    let mouse = input.mouse();
    for (position, size, button) in world
        .query::<(&Position, &Size, &TabButton)>()
        .without::<&Hidden>()
        .iter()
    {
        if input.clicked() && geometry::contains(position.0, size.0, mouse) {
            *tab = button.0;
        }
    }
    for (button, sprite) in world.query_mut::<(&TabButton, &mut Sprite)>() {
        let row = usize::from(button.0 == *tab);
        sprite.uv = Sprite::cell_uv(0, row, 1, 2);
    }
}

fn show_only(world: &mut World, tab: SettingsTabKind) {
    let scoped: Vec<(Entity, SettingsTabKind)> = world
        .query::<(Entity, &TabOnly)>()
        .iter()
        .map(|(entity, only)| (entity, only.0))
        .collect();
    for (entity, only) in scoped {
        if only == tab {
            let _ = world.remove_one::<Hidden>(entity);
        } else {
            let _ = world.insert_one(entity, Hidden);
        }
    }
}

fn update_checkboxes(world: &mut World, input: &Input, settings: &mut Settings) {
    let mouse = input.mouse();
    for (position, size, checkbox, sprite) in world
        .query_mut::<(&Position, &Size, &Checkbox, &mut Sprite)>()
        .without::<&Hidden>()
    {
        let hovered = geometry::contains(position.0, size.0, mouse);
        let state = settings.pending_mut();
        let value = match checkbox.toggle {
            Toggle::Vsync => &mut state.vsync,
            Toggle::Fullscreen => &mut state.fullscreen,
        };
        if hovered && input.clicked() {
            *value = !*value;
        }
        let row = usize::from(*value) + if hovered { 2 } else { 0 };
        sprite.uv = Sprite::cell_uv(0, row, 1, ROW_COUNT);
    }
}

fn volume_of(settings: &GameSettings, volume: Volume) -> f32 {
    match volume {
        Volume::Overall => settings.overall_volume,
        Volume::Music => settings.music_volume,
        Volume::Effects => settings.effects_volume,
    }
}

fn set_volume(settings: &mut GameSettings, volume: Volume, value: f32) {
    match volume {
        Volume::Overall => settings.overall_volume = value,
        Volume::Music => settings.music_volume = value,
        Volume::Effects => settings.effects_volume = value,
    }
}

fn update_sliders(world: &mut World, input: &Input, settings: &mut Settings) {
    let mouse = input.mouse();
    for (position, size, handle, sprite) in world
        .query_mut::<(&mut Position, &Size, &mut SliderHandle, &mut Sprite)>()
        .without::<&Hidden>()
    {
        if input.clicked() && geometry::contains(position.0, size.0, mouse) {
            handle.dragging = true;
        }
        if !input.mouse_down() {
            handle.dragging = false;
        }
        let travel = handle.track_width - size.0.x;
        if handle.dragging && travel > 0.0 {
            let left = (mouse.x - size.0.x / 2.0).clamp(handle.left, handle.left + travel);
            set_volume(
                settings.pending_mut(),
                handle.volume,
                (left - handle.left) / travel,
            );
        }
        let value = volume_of(settings.pending(), handle.volume);
        position.0.x = handle.left + value * travel + size.0.x / 2.0;
        sprite.uv = Sprite::cell_uv(0, usize::from(handle.dragging), 1, 2);
    }
    for (value, text) in world.query_mut::<(&SliderValue, &mut Text)>() {
        let percent = (volume_of(settings.pending(), value.0) * 100.0) as i32;
        text.content = format!("{percent}%");
    }
}

fn update_resolution_text(world: &mut World, settings: &Settings) {
    let (width, height) = settings.pending().resolution();
    for text in world.query_mut::<&mut Text>().with::<&ResolutionText>() {
        text.content = format!("{width}x{height}");
    }
}

fn update_key_binds(world: &mut World, input: &Input, settings: &mut Settings) {
    let mouse = input.mouse();
    let clicked = input.clicked();
    let mut hit = None;
    let mut listening = None;
    for (position, size, slot) in world
        .query::<(&Position, &Size, &KeyBindSlot)>()
        .without::<&Hidden>()
        .iter()
    {
        if clicked && geometry::contains(position.0, size.0, mouse) {
            hit = Some(slot.action);
        }
        if slot.listening {
            listening = Some(slot.action);
        }
    }
    if let (Some(action), Some(code)) = (listening, input.first_pressed()) {
        if code == KeyCode::Escape {
            listening = None;
        } else if controls::is_bindable(code) {
            settings.pending_mut().controls.rebind(action, code);
            listening = None;
        }
    }
    if clicked {
        listening = hit.filter(|action| Some(*action) != listening);
    }
    for (position, size, slot, text) in
        world.query_mut::<(&Position, &Size, &mut KeyBindSlot, &mut Text)>()
    {
        slot.listening = Some(slot.action) == listening;
        let hovered = geometry::contains(position.0, size.0, mouse);
        if slot.listening {
            KEY_PROMPT.clone_into(&mut text.content);
        } else {
            let key = settings.pending().controls.key(slot.action);
            controls::key_label(key).clone_into(&mut text.content);
        }
        text.color = if slot.listening || hovered {
            KEY_ACTIVE_COLOR
        } else {
            KEY_COLOR
        };
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;
    use winit::event::{ElementState, MouseButton};

    use crate::components::Color;
    use crate::controls::Action;

    use super::*;

    fn press_at(input: &mut Input, at: Vec2) {
        input.set_mouse(at);
        input.handle_click(MouseButton::Left, ElementState::Pressed);
    }

    fn checkbox_world(toggle: Toggle) -> World {
        let mut world = World::new();
        world.spawn((
            Position(Vec2::new(100.0, 100.0)),
            Size(Vec2::splat(24.0)),
            Color(glam::Vec4::ONE),
            Sprite::cell(crate::render::TextureId::WHITE, 0, 0, 1, 4),
            Checkbox { toggle },
        ));
        world
    }

    #[test]
    fn clicking_a_checkbox_flips_its_setting() {
        let mut world = checkbox_world(Toggle::Fullscreen);
        let mut settings = Settings::in_memory();
        let mut tab = SettingsTabKind::Video;
        let mut input = Input::default();

        press_at(&mut input, Vec2::new(105.0, 105.0));
        update_settings_ui(&mut world, &input, &mut settings, &mut tab);

        assert!(settings.pending().fullscreen);
    }

    #[test]
    fn clicking_next_to_a_checkbox_changes_nothing() {
        let mut world = checkbox_world(Toggle::Vsync);
        let mut settings = Settings::in_memory();
        let mut tab = SettingsTabKind::Video;
        let mut input = Input::default();

        press_at(&mut input, Vec2::new(400.0, 400.0));
        update_settings_ui(&mut world, &input, &mut settings, &mut tab);

        assert!(settings.pending().vsync);
    }

    #[test]
    fn dragging_a_slider_sets_the_volume_from_the_mouse_position() {
        let mut world = World::new();
        world.spawn((
            Position(Vec2::new(441.0, 233.0)),
            Size(Vec2::splat(32.0)),
            Sprite::cell(crate::render::TextureId::WHITE, 0, 0, 1, 2),
            SliderHandle {
                volume: Volume::Music,
                left: 425.0,
                track_width: 256.0,
                dragging: false,
            },
        ));
        let mut settings = Settings::in_memory();
        let mut tab = SettingsTabKind::Audio;
        let mut input = Input::default();

        press_at(&mut input, Vec2::new(441.0, 233.0));
        update_settings_ui(&mut world, &input, &mut settings, &mut tab);
        input.end_frame();

        input.set_mouse(Vec2::new(425.0 + 16.0 + 112.0, 233.0));
        update_settings_ui(&mut world, &input, &mut settings, &mut tab);

        assert!((settings.pending().music_volume - 0.5).abs() < 0.01);

        input.handle_click(MouseButton::Left, ElementState::Released);
        input.set_mouse(Vec2::new(900.0, 233.0));
        update_settings_ui(&mut world, &input, &mut settings, &mut tab);
        assert!((settings.pending().music_volume - 0.5).abs() < 0.01);
    }

    #[test]
    fn switching_tabs_hides_the_other_tabs_elements() {
        let mut world = World::new();
        let video = world.spawn((TabOnly(SettingsTabKind::Video),));
        let audio = world.spawn((TabOnly(SettingsTabKind::Audio),));
        let mut settings = Settings::in_memory();
        let mut tab = SettingsTabKind::Audio;
        let input = Input::default();

        update_settings_ui(&mut world, &input, &mut settings, &mut tab);

        assert!(world.get::<&Hidden>(video).is_ok());
        assert!(world.get::<&Hidden>(audio).is_err());
    }

    fn key_slot_world(action: Action) -> World {
        let mut world = World::new();
        world.spawn((
            Position(Vec2::new(100.0, 100.0)),
            Size(Vec2::new(100.0, 30.0)),
            KeyBindSlot {
                action,
                listening: false,
            },
            Text::centered("", glam::Vec4::ONE, crate::render::FontSize::Size22),
        ));
        world
    }

    #[test]
    fn clicking_a_key_slot_then_pressing_a_key_rebinds_the_action() {
        let mut world = key_slot_world(Action::Reload);
        let mut settings = Settings::in_memory();
        let mut tab = SettingsTabKind::Controls;
        let mut input = Input::default();

        press_at(&mut input, Vec2::new(100.0, 100.0));
        update_settings_ui(&mut world, &input, &mut settings, &mut tab);
        input.end_frame();
        assert_eq!(
            settings.pending().controls.key(Action::Reload),
            KeyCode::KeyR
        );

        input.press(KeyCode::KeyX);
        update_settings_ui(&mut world, &input, &mut settings, &mut tab);

        assert_eq!(
            settings.pending().controls.key(Action::Reload),
            KeyCode::KeyX
        );
        let label = world
            .query_mut::<&Text>()
            .into_iter()
            .next()
            .map(|text| text.content.clone());
        assert_eq!(label.as_deref(), Some("X"));
    }

    #[test]
    fn escape_cancels_listening_and_keeps_the_old_key() {
        let mut world = key_slot_world(Action::Reload);
        let mut settings = Settings::in_memory();
        let mut tab = SettingsTabKind::Controls;
        let mut input = Input::default();

        press_at(&mut input, Vec2::new(100.0, 100.0));
        update_settings_ui(&mut world, &input, &mut settings, &mut tab);
        input.end_frame();
        input.press(KeyCode::Escape);
        update_settings_ui(&mut world, &input, &mut settings, &mut tab);
        input.end_frame();
        input.press(KeyCode::KeyX);
        update_settings_ui(&mut world, &input, &mut settings, &mut tab);

        assert_eq!(
            settings.pending().controls.key(Action::Reload),
            KeyCode::KeyR
        );
    }

    #[test]
    fn clicking_elsewhere_stops_listening() {
        let mut world = key_slot_world(Action::Reload);
        let mut settings = Settings::in_memory();
        let mut tab = SettingsTabKind::Controls;
        let mut input = Input::default();

        press_at(&mut input, Vec2::new(100.0, 100.0));
        update_settings_ui(&mut world, &input, &mut settings, &mut tab);
        input.end_frame();
        press_at(&mut input, Vec2::new(700.0, 400.0));
        update_settings_ui(&mut world, &input, &mut settings, &mut tab);
        input.end_frame();
        input.press(KeyCode::KeyX);
        update_settings_ui(&mut world, &input, &mut settings, &mut tab);

        assert_eq!(
            settings.pending().controls.key(Action::Reload),
            KeyCode::KeyR
        );
    }

    #[test]
    fn apply_and_revert_grey_out_when_there_is_nothing_to_change() {
        let mut world = World::new();
        let apply = world.spawn((
            Position(glam::Vec2::new(100.0, 100.0)),
            Size(glam::Vec2::new(140.0, 50.0)),
            Button {
                idle_uv: Sprite::cell_uv(0, 0, 1, 3),
                hover_uv: Sprite::cell_uv(0, 1, 1, 3),
            },
            NeedsChanges,
            Sprite::whole(crate::render::TextureId::WHITE),
        ));
        let exit = world.spawn((
            Position(glam::Vec2::new(100.0, 200.0)),
            Size(glam::Vec2::new(140.0, 50.0)),
            Button {
                idle_uv: Sprite::cell_uv(0, 0, 1, 3),
                hover_uv: Sprite::cell_uv(0, 1, 1, 3),
            },
            Sprite::whole(crate::render::TextureId::WHITE),
        ));
        let label = |world: &mut World, button| {
            world.spawn((
                LabelOf(button),
                Text::centered("x", Vec4::ONE, crate::render::FontSize::Size22),
            ))
        };
        let apply_label = label(&mut world, apply);
        let exit_label = label(&mut world, exit);
        let input = Input::default();
        let color =
            |world: &World, entity| world.get::<&Text>(entity).map(|t| t.color).expect("label");

        update_side_buttons(&mut world, &input, false);
        assert_eq!(color(&world, apply_label), LABEL_DISABLED);
        assert_eq!(color(&world, exit_label), LABEL_IDLE);
        let uv = world.get::<&Sprite>(apply).map(|s| s.uv).expect("sprite");
        assert_eq!(uv, Sprite::cell_uv(0, DISABLED_ROW, 1, SIDE_ROWS));

        update_side_buttons(&mut world, &input, true);
        assert_eq!(color(&world, apply_label), LABEL_IDLE);

        let mut hovering = Input::default();
        hovering.set_mouse(glam::Vec2::new(100.0, 100.0));
        update_side_buttons(&mut world, &hovering, true);
        assert_eq!(color(&world, apply_label), KEY_ACTIVE_COLOR);
        update_side_buttons(&mut world, &hovering, false);
        assert_eq!(color(&world, apply_label), LABEL_DISABLED);
    }
}
