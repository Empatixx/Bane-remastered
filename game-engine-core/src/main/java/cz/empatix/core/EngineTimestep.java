package cz.empatix.core;

/**
 * Shared fixed-timestep configuration for the engine. Both the game loop and the
 * physics engine step at this rate, and interpolation is computed against it.
 */
public final class EngineTimestep {

    /** Fixed update delta in seconds (60 updates per second). */
    public static final double FIXED_DELTA_TIME = 1.0 / 60.0;

    private EngineTimestep() {
    }
}
