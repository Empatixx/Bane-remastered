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

pub(super) const PLAYERS: InteractionGroups = InteractionGroups::new(
    PLAYER,
    WALL.union(ENEMY_BULLET).union(OBJECT).union(FURNITURE),
    InteractionTestMode::And,
);

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

/// Enemies pass through one another: their steering keeps them apart, and bodies shoving at
/// each other only had them fighting for room.
pub(super) const ENEMIES: InteractionGroups = InteractionGroups::new(
    ENEMY,
    WALL.union(BULLET)
        .union(OBJECT)
        .union(FURNITURE),
    InteractionTestMode::And,
);

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
