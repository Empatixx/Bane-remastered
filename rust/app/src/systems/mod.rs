mod animation;
mod camera;
mod damage;
mod draw;
mod effects;
mod enemies;
mod hud;
mod interaction;
mod lighting;
mod minimap;
mod player_animation;
mod rooms;
mod settings_ui;
mod shadow;
mod shooting;
mod simulation;
mod steering;
mod ui;

pub(crate) use animation::{animate, chain_animations, remove_expired};
pub(crate) use camera::camera_target;
pub(crate) use damage::{
    contact_damage, player_is_dead, player_is_low_on_health, tick_invulnerability,
};
pub(crate) use draw::collect_draws;
pub(crate) use effects::{fade_lights, spawn_gaze_wind};
#[cfg(test)]
pub(crate) use enemies::finish_spawning;
pub(crate) use enemies::{chase_player, dash, flash_hits, reap_dead, tick_spawning, SPAWN_SECONDS};
pub(crate) use hud::{update_status_bars, update_weapon_hud};
pub(crate) use interaction::interact;
pub(crate) use lighting::collect_lights;
pub(crate) use minimap::{rooms_for, update_minimaps};
pub(crate) use player_animation::{animate_player, IDLE, PLAYER_COLUMNS, PLAYER_ROWS};
pub(crate) use rooms::{update_rooms, RoomContext, RoomRuntime};
pub(crate) use settings_ui::update_settings_ui;
pub(crate) use shadow::update_shadow;
pub(crate) use shooting::{bullet_hits, shoot, ShotContext, ShotRoll};
pub(crate) use simulation::simulate;
pub(crate) use steering::steer_player;
pub(crate) use ui::{update_buttons, update_text_buttons};
