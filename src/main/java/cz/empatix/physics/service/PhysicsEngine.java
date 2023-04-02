package cz.empatix.physics.service;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.physics.domain.CollisionObject;
import cz.empatix.render.Camera;
import org.dyn4j.geometry.Rectangle;
import org.dyn4j.geometry.Vector2;
import org.dyn4j.world.PhysicsWorld;
import org.dyn4j.world.World;
import org.lwjgl.opengl.GL30;

public class PhysicsEngine {
    private static World<CollisionObject> world;

    public PhysicsEngine() {
        world = new World<>();
        world.setGravity(PhysicsWorld.ZERO_GRAVITY);
        world.getSettings().setStepFrequency(1/60.0);
        world.getSettings().setMaximumTranslation(5);
        world.getSettings().setLinearTolerance(0.01);
        world.getSettings().setMaximumRotation(0);
        world.getSettings().setVelocityConstraintSolverIterations(30);
        world.getSettings().setPositionConstraintSolverIterations(30);
        GL30.glOrtho(0, WindowHandler.TARGET_WIDTH, WindowHandler.TARGET_HEIGHT, 0, -1, 1);

    }
    public void update(final double dt){
        world.step(1);
        world.getBodies().forEach(CollisionObject::postPhysicsUpdate);
    }
    public void reset(){
        world.removeAllBodies();
    }
    public void debugRender(){
        world.getBodies().forEach(body -> {
            body.getFixtures().forEach(fixture -> {
                Vector2[] vertices = ((Rectangle) fixture.getShape()).getVertices();
                GL30.glBegin(GL30.GL_LINE_LOOP);
                for (Vector2 vertex : vertices) {
                            GL30.glVertex2f((float) vertex.x + (float)body.getTransform().getTranslationX() - Camera.getX(), (float) vertex.y + (float)body.getTransform().getTranslationY() - Camera.getY());
                }
                GL30.glEnd();
            });
        });
    }
    public static void addBody(final CollisionObject body){
        world.addBody(body);
    }
}
