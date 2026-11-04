use rapier2d::prelude::{Group, InteractionGroups, InteractionTestMode};

const WALL: Group = Group::GROUP_1;
const PLAYER: Group = Group::GROUP_2;
const BULLET: Group = Group::GROUP_3;
const ENEMY: Group = Group::GROUP_4;
const ENEMY_BULLET: Group = Group::GROUP_5;
const OBJECT: Group = Group::GROUP_6;
const PET: Group = Group::GROUP_7;
/// Tables: everyone bumps into them and bullets fly over, but a flipped one stops enemy bullets.
const FURNITURE: Group = Group::GROUP_8;

pub(super) const WALLS: InteractionGroups = InteractionGroups::new(
    WALL,
    PLAYER
        .union(BULLET)
        .union(ENEMY)
        .union(ENEMY_BULLET)
        .union(PET)
        .union(OBJECT),
    InteractionTestMode::And,
);

/// The hero's feet: what bumps into walls, objects and furniture.
pub(super) const PLAYER_FEET: InteractionGroups = InteractionGroups::new(
    PLAYER,
    WALL.union(OBJECT).union(FURNITURE),
    InteractionTestMode::And,
);

/// The ground the hero covers for shots: what enemy bullets hit.
pub(super) const PLAYER_FIGURE: InteractionGroups =
    InteractionGroups::new(PLAYER, ENEMY_BULLET, InteractionTestMode::And);

pub(super) const ENEMY_BULLETS: InteractionGroups = InteractionGroups::new(
    ENEMY_BULLET,
    WALL.union(PLAYER).union(OBJECT).union(FURNITURE),
    InteractionTestMode::And,
);

pub(super) const BULLETS: InteractionGroups = InteractionGroups::new(
    BULLET,
    WALL.union(ENEMY).union(OBJECT),
    InteractionTestMode::And,
);

/// An enemy's feet: what bumps into walls, objects and furniture. Enemies pass through one
/// another: their steering keeps them apart, and bodies shoving at each other only had them
/// fighting for room.
pub(super) const ENEMY_FEET: InteractionGroups = InteractionGroups::new(
    ENEMY,
    WALL.union(OBJECT).union(FURNITURE),
    InteractionTestMode::And,
);

/// The ground an enemy covers for shots (see `ground_hurt`): only the hero's bullets hit it.
pub(super) const ENEMY_FIGURE: InteractionGroups =
    InteractionGroups::new(ENEMY, BULLET, InteractionTestMode::And);

pub(super) const OBJECTS: InteractionGroups = InteractionGroups::new(
    OBJECT,
    PLAYER
        .union(WALL)
        .union(OBJECT)
        .union(ENEMY)
        .union(BULLET)
        .union(ENEMY_BULLET)
        .union(PET)
        .union(FURNITURE),
    InteractionTestMode::And,
);

/// The tall part of an object above its base, like a barrel's or a training dummy's body: only
/// bullets hit it, so walkers pass behind it and are stopped by the base alone.
pub(super) const TARGETS: InteractionGroups =
    InteractionGroups::new(OBJECT, BULLET.union(ENEMY_BULLET), InteractionTestMode::And);

pub(super) const PETS: InteractionGroups = InteractionGroups::new(
    PET,
    WALL.union(OBJECT).union(FURNITURE),
    InteractionTestMode::And,
);

/// A standing table: in everyone's way, but bullets of both sides fly over it.
pub(super) const TABLES: InteractionGroups = InteractionGroups::new(
    FURNITURE,
    PLAYER.union(ENEMY).union(PET).union(OBJECT),
    InteractionTestMode::And,
);

/// A flipped table: the player's bullets still fly over it, enemy bullets stop at it.
pub(super) const COVER: InteractionGroups = InteractionGroups::new(
    FURNITURE,
    PLAYER.union(ENEMY).union(PET).union(OBJECT).union(ENEMY_BULLET),
    InteractionTestMode::And,
);
