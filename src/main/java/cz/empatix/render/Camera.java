package cz.empatix.render;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.render.domain.InterpolatedPosition;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector3f;

public class Camera {
    private static float zoom = 1;
    private static final InterpolatedPosition interpolatedPosition = new InterpolatedPosition();

    public static Matrix4f getProjection() {
        return new Matrix4f().ortho2D(
                        0,
                        WindowHandler.TARGET_WIDTH * zoom,
                        WindowHandler.TARGET_HEIGHT * zoom,
                        0)
                .translate(new Vector3f(interpolatedPosition.getPosition(), 0));
    }

    public static void update(final float dt) {
        interpolatedPosition.update(dt);
    }

    public static void setPosition(final float x, final float y) {
        interpolatedPosition.setPosition(x, y);
    }
    public static float getX() {
        return interpolatedPosition.getX();
    }
    public static float getY() {
        return interpolatedPosition.getY();
    }
    public static Vector2f getPosition() {
        return new Vector2f(-interpolatedPosition.getX(), -interpolatedPosition.getY());
    }
    public static Vector2f centerX() {
        return new Vector2f(WindowHandler.TARGET_WIDTH / 2, 0);
    }

    public static float getTargetWidth() {
        return WindowHandler.TARGET_WIDTH * zoom;
    }

    public static float getTargetHeight() {
        return WindowHandler.TARGET_HEIGHT * zoom;
    }

    public static void zoomIn(final float zoom){
        Camera.zoom += zoom;
    }
    public static void zoomOut(final float zoom){
        Camera.zoom -= zoom;
    }

}
