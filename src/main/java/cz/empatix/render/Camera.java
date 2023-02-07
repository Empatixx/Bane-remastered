package cz.empatix.render;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.render.domain.InterpolatedPosition;
import org.joml.Matrix4f;
import org.joml.Vector2f;

public class Camera {
    private static final InterpolatedPosition interpolatedPosition = new InterpolatedPosition();

    public static Matrix4f getProjection() {
        return new Matrix4f().ortho2D(
                        0,
                        WindowHandler.TARGET_WIDTH,
                        WindowHandler.TARGET_HEIGHT,
                        0)
                .translate(interpolatedPosition.getPosition());
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

    public static Vector2f centerX() {
        return new Vector2f(WindowHandler.TARGET_WIDTH / 2, 0);
    }

    public static int getTargetWidth() {
        return WindowHandler.TARGET_WIDTH;
    }

    public static int getTargetHeight() {
        return WindowHandler.TARGET_HEIGHT;
    }
}
