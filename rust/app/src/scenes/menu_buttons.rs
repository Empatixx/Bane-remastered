use glam::{Vec2, Vec4};
use hecs::{Entity, World};
use winit::keyboard::KeyCode;

use crate::assets::Textures;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{Color, Position, ScreenSpace, Size, Sprite, Text, UiAction};
use crate::geometry;
use crate::input::Input;
use crate::prefabs;
use crate::render::{Align, Anchor, FontSize};

pub(crate) const BUTTON_SIZE: Vec2 = Vec2::new(126.0, 63.0);
const BASE_TEXT: Vec4 = Vec4::new(200.0 / 255.0, 92.0 / 255.0, 18.0 / 255.0, 1.0);
const LIT_TEXT: Vec4 = Vec4::new(1.0, 0.78, 0.35, 1.0);
const ENTER_DELAY: f32 = 0.5;
const ENTER_STAGGER: f32 = 0.12;
const ENTER_SECONDS: f32 = 0.5;
const ENTER_SLIDE: f32 = 140.0;
const LIFT_SPEED: f32 = 12.0;
const GROW: f32 = 0.08;
const NUDGE: f32 = 6.0;
const MARKER_GAP: f32 = 18.0;
const MARKER_BOUNCE: f32 = 3.0;
const MARKER_RATE: f32 = 8.0;
const PRESS_SQUASH: f32 = 0.1;
pub(crate) const PRESS_SECONDS: f32 = 0.14;

struct Row {
    button: Entity,
    text: Entity,
    left: Entity,
    right: Entity,
    rest: Vec2,
    action: UiAction,
    lift: f32,
}

pub(crate) struct MenuButtons {
    rows: Vec<Row>,
    selected: usize,
    clock: f32,
    pressed: Option<(usize, f32)>,
    last_mouse: Vec2,
}

fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t.clamp(0.0, 1.0)).powi(3)
}

fn marker(world: &mut World, glyph: &str, at: Vec2) -> Entity {
    world.spawn((
        Position(at),
        Text {
            content: glyph.to_owned(),
            color: Vec4::new(LIT_TEXT.x, LIT_TEXT.y, LIT_TEXT.z, 0.0),
            size: FontSize::Size27,
            align: Align::Center,
            anchor: Anchor::Middle,
        },
        ScreenSpace,
        crate::components::Layer(3),
    ))
}

impl MenuButtons {
    pub(crate) fn build(
        world: &mut World,
        textures: &Textures,
        texture_name: &str,
        x: f32,
        rows: &[(f32, &str, UiAction)],
    ) -> Self {
        let mut built = Vec::new();
        for &(y, label, action) in rows {
            let rest = Vec2::new(x, y);
            let (button, text) =
                prefabs::button(world, rest, label, action, textures.get(texture_name));
            let _ = world.insert_one(button, ScreenSpace);
            let _ = world.insert_one(text, ScreenSpace);
            let left = marker(world, ">", rest);
            let right = marker(world, "<", rest);
            built.push(Row {
                button,
                text,
                left,
                right,
                rest,
                action,
                lift: 0.0,
            });
        }
        Self {
            rows: built,
            selected: 0,
            clock: 0.0,
            pressed: None,
            last_mouse: Vec2::NEG_INFINITY,
        }
    }

    pub(crate) fn entities(&self) -> Vec<Entity> {
        self.rows
            .iter()
            .flat_map(|row| [row.button, row.text, row.left, row.right])
            .collect()
    }

    fn select(&mut self, index: usize, sounds: &mut Vec<AudioCommand>) {
        if index != self.selected {
            self.selected = index;
            sounds.push(AudioCommand::Effect(SoundEffect::SwitchGun));
        }
    }

    fn activate(&mut self, index: usize, sounds: &mut Vec<AudioCommand>) {
        self.selected = index;
        self.pressed = Some((index, PRESS_SECONDS));
        sounds.push(AudioCommand::Effect(SoundEffect::Click));
    }

    pub(crate) fn update(
        &mut self,
        world: &mut World,
        input: &Input,
        dt: f32,
        sounds: &mut Vec<AudioCommand>,
    ) -> Option<UiAction> {
        self.clock += dt;
        let mut fired = None;
        if let Some((index, left)) = self.pressed {
            let left = left - dt;
            if left <= 0.0 {
                self.pressed = None;
                fired = Some(self.rows[index].action);
            } else {
                self.pressed = Some((index, left));
            }
        } else if self.clock >= ENTER_DELAY {
            self.read_input(input, sounds);
        }
        self.animate(world, dt);
        fired
    }

    fn read_input(&mut self, input: &Input, sounds: &mut Vec<AudioCommand>) {
        let count = self.rows.len();
        if count == 0 {
            return;
        }
        let next = input.just_pressed(KeyCode::ArrowDown) || input.just_pressed(KeyCode::KeyS);
        let previous = input.just_pressed(KeyCode::ArrowUp) || input.just_pressed(KeyCode::KeyW);
        if next {
            self.select((self.selected + 1) % count, sounds);
        } else if previous {
            self.select((self.selected + count - 1) % count, sounds);
        }
        let mouse = input.mouse();
        let hovered = self
            .rows
            .iter()
            .position(|row| geometry::contains(row.rest, BUTTON_SIZE, mouse));
        if mouse != self.last_mouse {
            self.last_mouse = mouse;
            if let Some(index) = hovered {
                self.select(index, sounds);
            }
        }
        if input.clicked() {
            if let Some(index) = hovered {
                self.activate(index, sounds);
                return;
            }
        }
        if input.just_pressed(KeyCode::Enter) || input.just_pressed(KeyCode::Space) {
            self.activate(self.selected, sounds);
        }
    }

    fn animate(&mut self, world: &mut World, dt: f32) {
        let clock = self.clock;
        let selected = self.selected;
        let pressed = self.pressed;
        for (index, row) in self.rows.iter_mut().enumerate() {
            let target = if index == selected { 1.0 } else { 0.0 };
            row.lift += (target - row.lift) * (LIFT_SPEED * dt).min(1.0);
            let arrival =
                ease_out((clock - ENTER_DELAY - index as f32 * ENTER_STAGGER) / ENTER_SECONDS);
            let squash = match pressed {
                Some((pressed_index, left)) if pressed_index == index => {
                    PRESS_SQUASH * (left / PRESS_SECONDS).clamp(0.0, 1.0).max(0.4)
                }
                _ => 0.0,
            };
            let scale = 1.0 + GROW * row.lift - squash;
            let at = row.rest + Vec2::new(NUDGE * row.lift - (1.0 - arrival) * ENTER_SLIDE, 0.0);
            let bounce = (clock * MARKER_RATE).sin() * MARKER_BOUNCE * row.lift;
            let half = BUTTON_SIZE.x * scale / 2.0;
            let tint = BASE_TEXT.lerp(LIT_TEXT, row.lift);
            if let Ok((position, size, color, sprite)) =
                world.query_one_mut::<(&mut Position, &mut Size, &mut Color, &mut Sprite)>(
                    row.button,
                )
            {
                position.0 = at;
                size.0 = BUTTON_SIZE * scale;
                color.0.w = arrival;
                let _ = sprite;
            }
            if let Ok((position, text)) =
                world.query_one_mut::<(&mut Position, &mut Text)>(row.text)
            {
                position.0 = at;
                text.color = Vec4::new(tint.x, tint.y, tint.z, arrival);
            }
            for (entity, side) in [(row.left, -1.0), (row.right, 1.0)] {
                if let Ok((position, text)) =
                    world.query_one_mut::<(&mut Position, &mut Text)>(entity)
                {
                    position.0 = at + Vec2::new(side * (half + MARKER_GAP - side * bounce), 0.0);
                    text.color.w = row.lift * arrival;
                }
            }
        }
        for row in &self.rows {
            if let Ok((button, sprite)) =
                world.query_one_mut::<(&crate::components::Button, &mut Sprite)>(row.button)
            {
                sprite.uv = if row.lift > 0.5 {
                    button.hover_uv
                } else {
                    button.idle_uv
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use winit::event::{ElementState, MouseButton};

    use super::*;

    const ROWS: [(f32, &str, UiAction); 3] = [
        (225.0, "Play", UiAction::Play),
        (325.0, "Settings", UiAction::Settings),
        (425.0, "Exit", UiAction::Exit),
    ];

    fn menu() -> (World, MenuButtons) {
        let mut world = World::new();
        let buttons = MenuButtons::build(
            &mut world,
            &Textures::default(),
            "menu/Menu_bar2.tga",
            480.0,
            &ROWS,
        );
        (world, buttons)
    }

    fn settle(world: &mut World, buttons: &mut MenuButtons, seconds: f32) {
        let steps = (seconds * 60.0) as usize;
        let input = Input::default();
        for _ in 0..steps {
            buttons.update(world, &input, 1.0 / 60.0, &mut Vec::new());
        }
    }

    fn text_alpha(world: &World, entity: Entity) -> f32 {
        world
            .get::<&Text>(entity)
            .map(|t| t.color.w)
            .unwrap_or(-1.0)
    }

    #[test]
    fn the_buttons_slide_in_one_after_another() {
        let (mut world, mut buttons) = menu();
        settle(&mut world, &mut buttons, ENTER_DELAY + ENTER_STAGGER + 0.1);

        let first = world
            .get::<&Position>(buttons.rows[0].button)
            .map(|p| p.0.x)
            .unwrap_or(0.0);
        let last = world
            .get::<&Position>(buttons.rows[2].button)
            .map(|p| p.0.x)
            .unwrap_or(0.0);
        assert!(first > last, "{first} {last}");

        settle(&mut world, &mut buttons, 2.0);
        for row in &buttons.rows {
            let at = world
                .get::<&Position>(row.button)
                .map(|p| p.0)
                .unwrap_or_default();
            assert!((at.x - row.rest.x).abs() <= NUDGE + 0.01);
            assert_eq!(text_alpha(&world, row.text), 1.0);
        }
    }

    #[test]
    fn the_keys_move_the_selection_around_and_the_markers_follow() {
        let (mut world, mut buttons) = menu();
        settle(&mut world, &mut buttons, 2.0);
        let mut sounds = Vec::new();
        let mut down = Input::default();
        down.press(KeyCode::ArrowDown);

        buttons.update(&mut world, &down, 0.016, &mut sounds);
        assert_eq!(buttons.selected, 1);
        assert_eq!(sounds.len(), 1);

        let mut up = Input::default();
        up.press(KeyCode::KeyW);
        buttons.update(&mut world, &up, 0.016, &mut sounds);
        buttons.update(&mut world, &up, 0.016, &mut sounds);
        assert_eq!(buttons.selected, 2);

        settle(&mut world, &mut buttons, 1.0);
        assert!(text_alpha(&world, buttons.rows[2].left) > 0.9);
        assert!(text_alpha(&world, buttons.rows[0].left) < 0.05);
    }

    #[test]
    fn enter_presses_the_selected_button_and_fires_after_the_press_animation() {
        let (mut world, mut buttons) = menu();
        settle(&mut world, &mut buttons, 2.0);
        let mut enter = Input::default();
        enter.press(KeyCode::Enter);
        let mut sounds = Vec::new();

        let early = buttons.update(&mut world, &enter, 0.016, &mut sounds);
        assert_eq!(early, None);
        assert!(buttons.pressed.is_some());

        let idle = Input::default();
        let mut fired = None;
        for _ in 0..20 {
            fired = fired.or(buttons.update(&mut world, &idle, 0.016, &mut sounds));
        }
        assert_eq!(fired, Some(UiAction::Play));
    }

    #[test]
    fn the_mouse_selects_what_it_hovers_and_a_click_presses_it() {
        let (mut world, mut buttons) = menu();
        settle(&mut world, &mut buttons, 2.0);
        let mut mouse = Input::default();
        mouse.set_mouse(Vec2::new(480.0, 330.0));
        let mut sounds = Vec::new();

        buttons.update(&mut world, &mouse, 0.016, &mut sounds);
        assert_eq!(buttons.selected, 1);

        mouse.handle_click(MouseButton::Left, ElementState::Pressed);
        mouse.handle_click(MouseButton::Left, ElementState::Released);
        buttons.update(&mut world, &mouse, 0.016, &mut sounds);
        let idle = Input::default();
        let mut fired = None;
        for _ in 0..20 {
            fired = fired.or(buttons.update(&mut world, &idle, 0.016, &mut sounds));
        }
        assert_eq!(fired, Some(UiAction::Settings));
    }

    #[test]
    fn nothing_can_be_pressed_while_the_buttons_are_still_arriving() {
        let (mut world, mut buttons) = menu();
        let mut enter = Input::default();
        enter.press(KeyCode::Enter);

        buttons.update(&mut world, &enter, 0.016, &mut Vec::new());

        assert!(buttons.pressed.is_none());
    }

    #[test]
    fn all_the_entities_can_be_hidden_together() {
        let (_world, buttons) = menu();
        assert_eq!(buttons.entities().len(), ROWS.len() * 4);
    }
}
