use rapier2d::prelude::{Group, InteractionGroups, InteractionTestMode};

const WALL: Group = Group::GROUP_1;
const PLAYER: Group = Group::GROUP_2;
const BULLET: Group = Group::GROUP_3;
const ENEMY: Group = Group::GROUP_4;

pub(super) const WALLS: InteractionGroups = InteractionGroups::new(
    WALL,
    PLAYER.union(BULLET).union(ENEMY),
    InteractionTestMode::And,
);

pub(super) const PLAYERS: InteractionGroups =
    InteractionGroups::new(PLAYER, WALL, InteractionTestMode::And);

pub(super) const BULLETS: InteractionGroups =
    InteractionGroups::new(BULLET, WALL.union(ENEMY), InteractionTestMode::And);

pub(super) const ENEMIES: InteractionGroups = InteractionGroups::new(
    ENEMY,
    WALL.union(BULLET).union(ENEMY),
    InteractionTestMode::And,
);
