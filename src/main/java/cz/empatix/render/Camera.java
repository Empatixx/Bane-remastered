package cz.empatix.render;

import cz.empatix.handlers.WindowHandler;
import org.joml.Matrix4f;

public class Camera {
    public static Matrix4f getProjection() {
        return new Matrix4f().ortho(
                -WindowHandler.TARGET_WIDTH/2,
                WindowHandler.TARGET_WIDTH/2,
                WindowHandler.TARGET_HEIGHT/2,
                -WindowHandler.TARGET_HEIGHT/2,
                0, 1);
    }
}
