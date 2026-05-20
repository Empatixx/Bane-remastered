package cz.empatix.gamestate.service;

import cz.empatix.newphysics.service.PhysicsDebugRenderer;
import cz.empatix.render.service.Camera;
import org.joml.Vector2f;
import org.lwjgl.opengl.GL30;

/**
 * OpenGL implementation of {@link PhysicsDebugRenderer}. Draws each physics body as a
 * camera-relative line loop. Lives in the rendering layer so the physics module stays
 * free of any OpenGL / camera dependency.
 */
public class GlPhysicsDebugRenderer implements PhysicsDebugRenderer {

    @Override
    public void beginBody() {
        GL30.glBegin(GL30.GL_LINE_LOOP);
    }

    @Override
    public void vertex(final double worldX, final double worldY) {
        final Vector2f cameraPos = Camera.getInstance().getPosition();
        GL30.glVertex2d(worldX - cameraPos.x, worldY - cameraPos.y);
    }

    @Override
    public void endBody() {
        GL30.glEnd();
    }
}
