package cz.empatix.newphysics.service;

import cz.empatix.common.GameLoop;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.render.service.Camera;
import org.dyn4j.dynamics.Settings;
import org.dyn4j.geometry.Rectangle;
import org.dyn4j.geometry.Shape;
import org.dyn4j.geometry.Vector2;
import org.dyn4j.world.World;
import org.joml.Vector2f;
import org.lwjgl.opengl.GL30;

public class PhysicsEngine {
    private final World<PhysicsObject> world;

    public PhysicsEngine() {
        final Settings settings = new Settings();
        settings.setStepFrequency(GameLoop.FIXED_DELTA_TIME);
        settings.setAtRestDetectionEnabled(false);
        settings.setMaximumTranslation(100);

        world = new World<>();
        world.setGravity(0,0);
        world.setSettings(settings);


        world.addCollisionListener(new DamageCollisionListener());
        world.addCollisionListener(new CollisionListener());
    }

    public void add(final PhysicsObject object) {
        world.addBody(object);
    }

    public void fixedUpdate(double dt) {
        world.update(dt);
    }

    public void clear() {
        world.removeAllBodies();
    }


    public void debugRender() {
        world.getBodies().forEach(body -> {
            GL30.glBegin(GL30.GL_LINE_LOOP);
            final Rectangle rectangle = (Rectangle) body.getFixture(0).getShape();
            final Vector2[] vertices = rectangle.getVertices();
            final Vector2 position = body.getTransform().getTranslation().copy().subtract(new Vector2(Camera.getPosition().x, Camera.getPosition().y));
            final double angle =  body.getTransform().getRotationAngle();
            for (Vector2 vertex : vertices) {
                final Vector2 rotated = vertex.rotate(angle);
                GL30.glVertex2d(rotated.x + position.x, rotated.y + position.y);
            }
            GL30.glEnd();
        });
    }

    public boolean isColliding(final Shape shape, final Vector2f position) {
        return false;
    }

    public void remove(final PhysicsObject physicsObject) {
        world.removeBody(physicsObject);
    }
}
