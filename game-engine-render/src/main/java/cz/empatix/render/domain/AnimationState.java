package cz.empatix.render.domain;

/**
 * Opaque key identifying one state (frame sequence) of an {@link Animation}.
 * The engine never enumerates or interprets the concrete states; the game
 * defines its own catalogue (e.g. an enum) implementing this marker interface
 * and uses those constants as map keys. This keeps the render engine free of
 * any game-specific animation vocabulary.
 */
public interface AnimationState {
}
