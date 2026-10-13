package cz.empatix.newphysics.service;

import cz.empatix.core.CollisionCategory;
import cz.empatix.core.CollisionHandler;
import cz.empatix.core.EngineTimestep;
import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.collision.Filter;
import org.dyn4j.collision.Fixture;
import org.dyn4j.dynamics.Body;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.dynamics.ContinuousDetectionMode;
import org.dyn4j.dynamics.Settings;
import org.dyn4j.geometry.*;
import org.dyn4j.world.DetectFilter;
import org.dyn4j.world.World;
import org.dyn4j.world.result.DetectResult;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;

public class PhysicsEngine {
    private final static Logger LOGGER = LoggerFactory.getLogger(PhysicsEngine.class);
    private final World<PhysicsObject> world;
    private final CollisionHandlerDispatcher collisionHandlerDispatcher = new CollisionHandlerDispatcher();

    public PhysicsEngine() {
        final Settings settings = new Settings();
        settings.setStepFrequency(EngineTimestep.FIXED_DELTA_TIME);
        settings.setAtRestDetectionEnabled(true);
        settings.setMaximumTranslation(1000);
        settings.setMaximumLinearCorrection(1);
        settings.setMaximumAngularCorrection(100);
        settings.setLinearTolerance(0.1);
        settings.setContinuousDetectionMode(ContinuousDetectionMode.NONE);


        world = new World<>();
        world.setGravity(0, 0);
        world.setSettings(settings);
        world.setBounds(null);

        final EnterLeaveListener enterLeaveListener = new EnterLeaveListener();

        world.addStepListener(new InterpolationStepListener());
        world.addStepListener(enterLeaveListener);
        world.addCollisionListener(collisionHandlerDispatcher);
        world.addCollisionListener(new CollisionListener(enterLeaveListener));
    }

    public void addCollisionHandler(final CollisionHandler handler) {
        collisionHandlerDispatcher.add(handler);
    }

    public void add(final PhysicsObject object) {
        world.addBody(object);
    }

    public void fixedUpdate(double dt) {
        world.step(1);

        updateForRemoval();
    }

    private void updateForRemoval() {
        final List<PhysicsObject> toRemove = world.getBodies()
                .stream()
                .filter(PhysicsObject::canRemove)
                .toList();
        toRemove.forEach(world::removeBody);
    }

    public void update(final double dt) {
        final float delta = (float) dt;
        for (PhysicsObject body : world.getBodies()) {
            body.update(delta);
        }
    }

    public void clear() {
        world.removeAllBodies();
    }


    public void debugRender(final PhysicsDebugRenderer renderer) {
        world.getBodies().forEach(body -> {
            final Rectangle rectangle = (Rectangle) body.getFixture(0).getShape();
            renderer.beginBody();
            for (final Vector2 vertex : rectangle.getVertices()) {
                final Vector2 transformed = body.getWorldPoint(vertex);
                renderer.vertex(transformed.x, transformed.y);
            }
            renderer.endBody();
        });
    }

    public boolean isColliding(final PhysicsObject object) {
        for (PhysicsObject body : world.getBodies()) {
            if (body == object) {
                continue;
            }
            final AABB aabb = body.createAABB();
            final AABB aabb1 = object.createAABB();
            if (aabb.overlaps(aabb1)) {
                return true;
            }
        }
        return false;
    }

    public void remove(final PhysicsObject physicsObject) {
        world.removeBody(physicsObject);
    }

    public List<PhysicsObject> query(final Vector2f point, final Vector2i dimensions, final Set<? extends CollisionCategory> ignored) {
        final Transform transform = new Transform();
        transform.setTranslation(new Vector2(point.x, point.y));
        final Rectangle rectangle = Geometry.createRectangle(dimensions.x, dimensions.y);
        rectangle.translate(new Vector2(dimensions.x / 2f, dimensions.y / 2f));

        final Set<PhysicsObject> results = new HashSet<>();
        final Iterator<DetectResult<PhysicsObject, BodyFixture>> resultIterator = world.detectIterator(rectangle, transform, new DetectFilter(true, true, null));
        while (resultIterator.hasNext()) {
            final DetectResult<PhysicsObject, BodyFixture> result = resultIterator.next();
            final PhysicsObject object = result.getBody();
            final BodyFixture fixture = result.getFixture();
            if (object == null) {
                continue;
            }
            if (object.canRemove()) {
                continue;
            }
            if (object.getType() != null && ignored.contains(object.getType())) {
                continue;
            }
            results.add(object);
        }
        return List.copyOf(results);
    }

    public boolean queryExists(final Vector2f point, final Vector2i dimensions, final Set<? extends CollisionCategory> ignored) {
        final Transform transform = new Transform();
        transform.setTranslation(new Vector2(point.x, point.y));
        final Rectangle rectangle = Geometry.createRectangle(dimensions.x, dimensions.y);
        rectangle.getVertices()[0].set(0, 0);
        rectangle.getVertices()[1].set(dimensions.x(), 0);
        rectangle.getVertices()[2].set(dimensions.x(), dimensions.y());
        rectangle.getVertices()[3].set(0, dimensions.y());

        final Set<PhysicsObject> results = new HashSet<>();
        final Iterator<DetectResult<PhysicsObject, BodyFixture>> resultIterator = world.detectIterator(rectangle, transform, new DetectFilter(true, true, null));
        while (resultIterator.hasNext()) {
            final DetectResult<PhysicsObject, BodyFixture> result = resultIterator.next();
            final PhysicsObject object = result.getBody();
            final BodyFixture fixture = result.getFixture();
            if (object == null) {
                continue;
            }
            if (object.canRemove()) {
                continue;
            }
            if (object.getType() != null && ignored.contains(object.getType())) {
                continue;
            }
            return true;
        }
        return false;
    }

    public void updateBroadphase() {
        world.getBroadphaseDetector().update();
    }
}
