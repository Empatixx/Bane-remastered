package cz.empatix.animation;

/**
 * The game's catalogue of animation states. The render engine treats animation
 * states as opaque keys (see {@link cz.empatix.render.domain.AnimationState});
 * this enum is the concrete, game-specific set of them.
 */
public enum AnimationState implements cz.empatix.render.domain.AnimationState {
    IDLE,
    ATTACK,
    RIGHT_MOVE, LEFT_MOVE, TOP_MOVE, BOTTOM_MOVE,
    OPEN,
    CLOSE,
    WALK,
    DEATH,
    REVERSE,
    STATIC,
    UNSELECTED, SELECTED,
    UNSELECTED_HOVER, SELECTED_HOVER,
    COLLISION,
    HOVER,
    LEFT_SIDE, RIGHT_SIDE,
    TOP,

    TYPE0, TYPE1, TYPE2, TYPE3, TYPE4,
    TYPE5, TYPE6, TYPE7, TYPE8, TYPE9,
    SPELL0, SPELL1, SPELL2, SPELL3, SPELL4, HIT, DEACTIVATED, EMPTY, IDLE_2, SIDE_OPEN, SIDE_CLOSE, SIDE_IDLE,
    RIGHT_SIDE_OPEN, RIGHT_SIDE_CLOSE, RIGHT_SIDE_IDLE,
    LEFT_SIDE_OPEN, LEFT_SIDE_CLOSE, LEFT_SIDE_IDLE,
    DISABLED_LEFT_SIDE, DISABLED_RIGHT_SIDE, DISABLED_TOP,
    SPIN, WIN, LOSE,
}
