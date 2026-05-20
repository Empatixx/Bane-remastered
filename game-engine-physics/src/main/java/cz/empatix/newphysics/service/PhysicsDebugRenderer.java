package cz.empatix.newphysics.service;

/**
 * Render-agnostic callback used to draw physics body outlines for debugging.
 * <p>
 * The physics engine walks its bodies and emits world-space vertices through this
 * interface; the concrete implementation (in the rendering layer) is responsible for
 * the actual drawing, camera transform and any visibility gating. This keeps the
 * physics module free of any rendering / OpenGL dependency.
 */
public interface PhysicsDebugRenderer {

    /** Starts a new body outline (a closed polygon of {@link #vertex} calls). */
    void beginBody();

    /** Emits a single world-space vertex of the current body outline. */
    void vertex(double worldX, double worldY);

    /** Finishes the current body outline. */
    void endBody();
}
