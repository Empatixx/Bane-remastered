use hecs::World;

use crate::components::{Button, Color, OnClick, Position, Size, UiAction};
use crate::geometry;
use crate::input::Input;

pub(crate) fn update_buttons(world: &mut World, input: &Input) -> Option<UiAction> {
    let mouse = input.mouse();
    let mut triggered = None;
    for (position, size, button, action, color) in
        world.query_mut::<(&Position, &Size, &Button, &OnClick, &mut Color)>()
    {
        let hovered = geometry::contains(position.0, size.0, mouse);
        color.0 = if hovered { button.hover } else { button.idle };
        if hovered && input.clicked() {
            triggered = Some(action.0);
        }
    }
    triggered
}
