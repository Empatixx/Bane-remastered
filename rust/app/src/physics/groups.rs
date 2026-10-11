use rapier2d::prelude::{Group, InteractionGroups, InteractionTestMode};

const WALL: Group = Group::GROUP_1;
const PLAYER: Group = Group::GROUP_2;
const BULLET: Group = Group::GROUP_3;
const ENEMY: Group = Group::GROUP_4;
const ENEMY_BULLET: Group = Group::GROUP_5;
const OBJECT: Group = Group::GROUP_6;

pub(super) const WALLS: InteractionGroups = InteractionGroups::new(
    WALL,
    PLAYER.union(BULLET).union(ENEMY).union(ENEMY_BULLET),
    InteractionTestMode::And,
);

pub(super) const PLAYERS: InteractionGroups = InteractionGroups::new(
    PLAYER,
    WALL.union(ENEMY_BULLET).union(OBJECT),
    InteractionTestMode::And,
);

pub(super) const ENEMY_BULLETS: InteractionGroups = InteractionGroups::new(
    ENEMY_BULLET,
    WALL.union(PLAYER).union(OBJECT),
    InteractionTestMode::And,
);

pub(super) const BULLETS: InteractionGroups = InteractionGroups::new(
    BULLET,
    WALL.union(ENEMY).union(OBJECT),
    InteractionTestMode::And,
);

pub(super) const ENEMIES: InteractionGroups = InteractionGroups::new(
    ENEMY,
    WALL.union(BULLET).union(ENEMY).union(OBJECT),
    InteractionTestMode::And,
);

pub(super) const OBJECTS: InteractionGroups = InteractionGroups::new(
    OBJECT,
    PLAYER.union(ENEMY).union(BULLET).union(ENEMY_BULLET),
    InteractionTestMode::And,
);
