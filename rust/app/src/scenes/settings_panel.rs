use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::Assets;
use crate::components::{
    Button, Checkbox, Color, Hidden, KeyBindSlot, LabelOf, Layer, NeedsChanges, OnClick, Position,
    ResolutionText, ScreenSpace, SettingsTabKind, SettingsUi, Size, SliderHandle, SliderValue,
    Sprite, TabButton, TabOnly, Text, Toggle, UiAction, Volume,
};
use crate::controls::Action;
use crate::input::Input;
use crate::prefabs;
use crate::render::{Align, Anchor, FontSize};
use crate::settings::Settings;
use crate::systems;

const BACKGROUND_CENTER: Vec2 = Vec2::new(480.0, 270.0);
const BACKGROUND_SIZE: Vec2 = Vec2::new(516.0, 354.0);
const TAB_SIZE: Vec2 = Vec2::new(150.0, 48.0);
const TABS: [(SettingsTabKind, &str, f32); 3] = [
    (SettingsTabKind::Video, "Video", 250.0),
    (SettingsTabKind::Audio, "Audio", 405.0),
    (SettingsTabKind::Controls, "Controls", 560.0),
];
const TAB_TOP: f32 = 128.0;
const SIDE_ROWS: usize = 3;
const SIDE_BUTTON_SIZE: Vec2 = Vec2::new(144.0, 57.0);
const SIDE_BUTTONS: [(f32, &str, UiAction); 3] = [
    (190.0, "Apply", UiAction::SettingsApply),
    (256.0, "Revert", UiAction::SettingsRevert),
    (322.0, "Exit", UiAction::SettingsExit),
];
const LABEL_X: f32 = 275.0;
const CHECKBOX_X: f32 = 470.0;
const CHECKBOX_SIZE: f32 = 24.0;
const CHECKBOXES: [(Toggle, &str, f32); 2] = [
    (Toggle::Vsync, "V-sync:", 225.0),
    (Toggle::Fullscreen, "Fullscreen:", 275.0),
];
const ARROW_SIZE: f32 = 32.0;
const SLIDER_X: f32 = 425.0;
const SLIDER_TRACK: Vec2 = Vec2::new(256.0, 16.0);
const SLIDER_HANDLE: f32 = 32.0;
const SLIDER_VALUE_X: f32 = 365.0;
const SLIDERS: [(Volume, &str, f32); 3] = [
    (Volume::Overall, "Overall:", 225.0),
    (Volume::Music, "Music:", 275.0),
    (Volume::Effects, "Effects:", 325.0),
];
const KEY_COLUMNS: [(f32, f32); 2] = [(260.0, 420.0), (490.0, 660.0)];
const KEY_ROW_TOP: f32 = 225.0;
const KEY_ROW_STEP: f32 = 42.0;
const KEY_ROWS: usize = 5;
const KEY_SLOT: Vec2 = Vec2::new(110.0, 32.0);
const LABEL_COLOR: Vec4 = Vec4::new(222.0 / 255.0, 112.0 / 255.0, 38.0 / 255.0, 1.0);
const VALUE_COLOR: Vec4 = Vec4::new(225.0 / 255.0, 152.0 / 255.0, 116.0 / 255.0, 1.0);

#[derive(Debug)]
pub(crate) struct SettingsPanel {
    open: bool,
    tab: SettingsTabKind,
}

impl Default for SettingsPanel {
    fn default() -> Self {
        Self {
            open: false,
            tab: SettingsTabKind::Video,
        }
    }
}

impl SettingsPanel {
    pub(crate) fn is_open(&self) -> bool {
        self.open
    }

    pub(crate) fn open(&mut self, world: &mut World, assets: &Assets, hide: &[Entity]) {
        self.open = true;
        self.tab = SettingsTabKind::Video;
        for entity in hide.iter().copied() {
            let _ = world.insert_one(entity, Hidden);
        }
        spawn_panel(world, assets);
    }

    pub(crate) fn close(&mut self, world: &mut World) {
        self.open = false;
        let panel: Vec<Entity> = world
            .query::<Entity>()
            .with::<&SettingsUi>()
            .iter()
            .collect();
        for entity in panel {
            let _ = world.despawn(entity);
        }
        let hidden: Vec<Entity> = world.query::<Entity>().with::<&Hidden>().iter().collect();
        for entity in hidden {
            let _ = world.remove_one::<Hidden>(entity);
        }
    }

    pub(crate) fn update(
        &mut self,
        world: &mut World,
        input: &Input,
        settings: &mut Settings,
    ) -> bool {
        let dirty = settings.has_changes();
        let mut action = systems::update_buttons(world, input);
        if !dirty
            && matches!(
                action,
                Some(UiAction::SettingsApply | UiAction::SettingsRevert)
            )
        {
            action = None;
        }
        systems::update_settings_ui(world, input, settings, &mut self.tab);
        systems::update_side_buttons(world, input, settings.has_changes());
        match action {
            Some(UiAction::SettingsApply) => settings.apply(),
            Some(UiAction::SettingsRevert) => settings.revert(),
            Some(UiAction::ResolutionNext) => settings.next_resolution(),
            Some(UiAction::ResolutionPrevious) => settings.previous_resolution(),
            Some(UiAction::SettingsExit) => self.close(world),
            _ => {}
        }
        action.is_some()
    }
}

fn spawn_panel(world: &mut World, assets: &Assets) {
    let textures = &assets.textures;
    let ui = |world: &mut World, entity: Entity, layer: i32, tab: Option<SettingsTabKind>| {
        let _ = world.insert(entity, (SettingsUi, ScreenSpace, Layer(layer)));
        if let Some(tab) = tab {
            let _ = world.insert_one(entity, TabOnly(tab));
        }
    };

    let background = world.spawn((
        Position(BACKGROUND_CENTER),
        Size(BACKGROUND_SIZE),
        Color(Vec4::ONE),
        Sprite::whole(textures.get("menu/settings_background.tga")),
    ));
    ui(world, background, 5, None);

    for (kind, label, left) in TABS {
        let center = Vec2::new(left, TAB_TOP) + TAB_SIZE / 2.0;
        let tab = world.spawn((
            TabButton(kind),
            Position(center),
            Size(TAB_SIZE),
            Color(Vec4::ONE),
            Sprite::cell(textures.get("menu/settings_menubar.tga"), 0, 0, 1, 2),
        ));
        ui(world, tab, 6, None);
        let text = world.spawn((
            Position(center),
            Text::centered(label, LABEL_COLOR, FontSize::Size32),
        ));
        ui(world, text, 7, None);
    }

    for (y, label, action) in SIDE_BUTTONS {
        let (button, text) = prefabs::button(
            world,
            Vec2::new(800.0, y),
            label,
            action,
            textures.get("menu/Menu_plate_small.tga"),
        );
        if let Ok(mut size) = world.get::<&mut Size>(button) {
            size.0 = SIDE_BUTTON_SIZE;
        }
        if let Ok((sprite, state)) = world.query_one_mut::<(&mut Sprite, &mut Button)>(button) {
            sprite.uv = Sprite::cell_uv(0, 0, 1, SIDE_ROWS);
            state.idle_uv = Sprite::cell_uv(0, 0, 1, SIDE_ROWS);
            state.hover_uv = Sprite::cell_uv(0, 1, 1, SIDE_ROWS);
        }
        if matches!(action, UiAction::SettingsApply | UiAction::SettingsRevert) {
            let _ = world.insert_one(button, NeedsChanges);
        }
        let _ = world.insert_one(text, LabelOf(button));
        ui(world, button, 8, None);
        ui(world, text, 9, None);
    }

    let video = Some(SettingsTabKind::Video);
    for (toggle, label, y) in CHECKBOXES {
        let center = Vec2::new(CHECKBOX_X + CHECKBOX_SIZE / 2.0, y + CHECKBOX_SIZE / 2.0);
        let checkbox = world.spawn((
            Checkbox { toggle },
            Position(center),
            Size(Vec2::splat(CHECKBOX_SIZE)),
            Color(Vec4::ONE),
            Sprite::cell(textures.get("menu/checkbox.tga"), 0, 0, 1, 4),
        ));
        ui(world, checkbox, 6, video);
        spawn_label(world, label, Vec2::new(LABEL_X, center.y), video);
    }

    let resolution_label = world.spawn((
        Position(Vec2::new(LABEL_X, 341.0)),
        Text {
            content: "Resolution:".to_owned(),
            color: LABEL_COLOR,
            size: FontSize::Size22,
            align: Align::Left,
            anchor: Anchor::Baseline,
        },
    ));
    ui(world, resolution_label, 7, video);
    let resolution = world.spawn((
        ResolutionText,
        Position(Vec2::new(485.0, 340.0)),
        Text {
            content: String::new(),
            color: LABEL_COLOR,
            size: FontSize::Size22,
            align: Align::Center,
            anchor: Anchor::Baseline,
        },
    ));
    ui(world, resolution, 7, video);
    for (center_x, action, texture) in [
        (
            560.0,
            UiAction::ResolutionNext,
            "menu/button_arrow_rotated.tga",
        ),
        (410.0, UiAction::ResolutionPrevious, "menu/button_arrow.tga"),
    ] {
        let arrow = world.spawn((
            Button {
                idle_uv: Sprite::cell_uv(0, 0, 1, 2),
                hover_uv: Sprite::cell_uv(0, 1, 1, 2),
            },
            OnClick(action),
            Position(Vec2::new(center_x, 335.0)),
            Size(Vec2::splat(ARROW_SIZE)),
            Color(Vec4::ONE),
            Sprite::cell(textures.get(texture), 0, 0, 1, 2),
        ));
        ui(world, arrow, 8, video);
    }

    let audio = Some(SettingsTabKind::Audio);
    for (volume, label, y) in SLIDERS {
        let center_y = y + SLIDER_TRACK.y / 2.0;
        let track = world.spawn((
            Position(Vec2::new(SLIDER_X + SLIDER_TRACK.x / 2.0, center_y)),
            Size(SLIDER_TRACK),
            Color(Vec4::ONE),
            Sprite::whole(textures.get("menu/slider.tga")),
        ));
        ui(world, track, 6, audio);
        let handle = world.spawn((
            SliderHandle {
                volume,
                left: SLIDER_X,
                track_width: SLIDER_TRACK.x,
                dragging: false,
            },
            Position(Vec2::new(SLIDER_X + SLIDER_HANDLE / 2.0, center_y)),
            Size(Vec2::splat(SLIDER_HANDLE)),
            Color(Vec4::ONE),
            Sprite::cell(textures.get("menu/slider-button.tga"), 0, 0, 1, 2),
        ));
        ui(world, handle, 7, audio);
        spawn_label(world, label, Vec2::new(LABEL_X, center_y), audio);
        let value = world.spawn((
            SliderValue(volume),
            Position(Vec2::new(SLIDER_VALUE_X, center_y)),
            Text {
                content: String::new(),
                color: VALUE_COLOR,
                size: FontSize::Size22,
                align: Align::Left,
                anchor: Anchor::Middle,
            },
        ));
        ui(world, value, 7, audio);
    }

    let controls = Some(SettingsTabKind::Controls);
    for (index, action) in Action::ALL.into_iter().enumerate() {
        let (label_x, key_x) = KEY_COLUMNS[index / KEY_ROWS];
        let y = KEY_ROW_TOP + KEY_ROW_STEP * (index % KEY_ROWS) as f32;
        spawn_label(world, action.label(), Vec2::new(label_x, y), controls);
        let slot = world.spawn((
            KeyBindSlot {
                action,
                listening: false,
            },
            Position(Vec2::new(key_x, y)),
            Size(KEY_SLOT),
            Text {
                content: String::new(),
                color: VALUE_COLOR,
                size: FontSize::Size22,
                align: Align::Center,
                anchor: Anchor::Middle,
            },
        ));
        ui(world, slot, 7, controls);
    }
}

fn spawn_label(world: &mut World, label: &str, left_middle: Vec2, tab: Option<SettingsTabKind>) {
    let entity = world.spawn((
        Position(left_middle),
        Text {
            content: label.to_owned(),
            color: LABEL_COLOR,
            size: FontSize::Size22,
            align: Align::Left,
            anchor: Anchor::Middle,
        },
        SettingsUi,
        ScreenSpace,
        Layer(7),
    ));
    if let Some(tab) = tab {
        let _ = world.insert_one(entity, TabOnly(tab));
    }
}
