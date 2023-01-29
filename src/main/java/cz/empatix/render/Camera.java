package cz.empatix.render;

import cz.empatix.handlers.WindowHandler;
import org.joml.Matrix4f;
import org.joml.Vector2f;

public class Camera {
    public static Matrix4f getProjection() {
        return new Matrix4f().ortho2D(
                0,
                WindowHandler.TARGET_WIDTH,
                WindowHandler.TARGET_HEIGHT,
                0);
    }
    public static Vector2f centerX(){
        return new Vector2f(WindowHandler.TARGET_WIDTH / 2, 0);
    }
}
