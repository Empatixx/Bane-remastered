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
    private final static boolean DEBUG_RENDER = true;
    public PhysicsEngine() {
        final Settings settings = new Settings();
        settings.setStepFrequency(GameLoop.FIXED_DELTA_TIME);
        settings.setAtRestDetectionEnabled(false);
        settings.setMaximumTranslation(100);
        settings.setMaximumLinearCorrection(100);
        settings.setMaximumRotation(0);
        settings.setMaximumAngularCorrection(0);
        settings.setLinearTolerance(0.1);


        world = new World<>();
        world.setGravity(0,0);
        world.setSettings(settings);


        world.addStepListener(new InterpolationStepListener());
        world.addCollisionListener(new DamageCollisionListener());
        world.addCollisionListener(new CollisionListener());

    }

    public void add(final PhysicsObject object) {
        world.addBody(object);
    }

    public void fixedUpdate(double dt) {
        world.step(1);
    }
    public void update(final double dt){
        final float delta = (float) dt;
        for (PhysicsObject body : world.getBodies()) {
            body.update(delta);
        }
    }

    public void clear() {
        world.removeAllBodies();
    }


    public void debugRender() {
        if (!DEBUG_RENDER) {
            return;
        }
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
