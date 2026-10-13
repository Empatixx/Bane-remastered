use hecs::{Entity, World};

use crate::assets::Assets;
use crate::components::{PauseUi, UiAction};
use crate::input::Input;
use crate::prefabs;
use crate::systems;

#[derive(Debug, Default)]
pub(crate) struct Pause {
    active: bool,
}

impl Pause {
    pub(crate) fn is_active(&self) -> bool {
        self.active
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
            let ui: Vec<Entity> = world.query::<Entity>().with::<&PauseUi>().iter().collect();
            for entity in ui {
                let _ = world.despawn(entity);
            }
        }
    }

    pub(crate) fn update(&self, world: &mut World, input: &Input) -> Option<UiAction> {
        if self.active {
            systems::update_text_buttons(world, input)
        } else {
            None
        }
    }
}
