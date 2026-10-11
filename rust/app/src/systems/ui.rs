use hecs::World;

use crate::components::{
    Button, Hitbox, OnClick, Position, Size, Sprite, Text, TextButton, UiAction,
};
use crate::geometry;
use crate::input::Input;

pub(crate) fn update_buttons(world: &mut World, input: &Input) -> Option<UiAction> {
    let mouse = input.mouse();
    let mut triggered = None;
    for (position, size, button, action, sprite) in
        world.query_mut::<(&Position, &Size, &Button, &OnClick, &mut Sprite)>()
    {
        let hovered = geometry::contains(position.0, size.0, mouse);
        sprite.uv = if hovered {
            button.hover_uv
        } else {
            button.idle_uv
        };
        if hovered && input.clicked() {
            triggered = Some(action.0);
        }
    }
    triggered
}

pub(crate) fn update_text_buttons(world: &mut World, input: &Input) -> Option<UiAction> {
    let mouse = input.mouse();
    let mut triggered = None;
    for (position, hitbox, button, action, text) in
        world.query_mut::<(&Position, &Hitbox, &TextButton, &OnClick, &mut Text)>()
    {
        let hovered = geometry::contains(position.0 + hitbox.offset, hitbox.size, mouse);
        text.color = if hovered { button.hover } else { button.idle };
        if hovered && input.clicked() {
            triggered = Some(action.0);
        }
    }
    triggered
}
