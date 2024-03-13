package cz.empatix.newphysics.service;

import cz.empatix.common.GameLoop;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.render.service.Camera;
import org.dyn4j.collision.Filter;
import org.dyn4j.collision.Fixture;
import org.dyn4j.dynamics.Body;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.dynamics.Settings;
import org.dyn4j.geometry.*;
import org.dyn4j.world.DetectFilter;
import org.dyn4j.world.World;
import org.dyn4j.world.result.DetectResult;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.lwjgl.opengl.GL30;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;

public class PhysicsEngine {
    private final static Logger LOGGER = LoggerFactory.getLogger(PhysicsEngine.class);
    private final World<PhysicsObject> world;
    private final static boolean DEBUG_RENDER = true;

    public PhysicsEngine() {
        final Settings settings = new Settings();
        settings.setStepFrequency(GameLoop.FIXED_DELTA_TIME);
        settings.setAtRestDetectionEnabled(true);
        settings.setMaximumTranslation(100);
        settings.setMaximumLinearCorrection(100);
        settings.setMaximumAngularCorrection(100);
        settings.setLinearTolerance(0.1);


        world = new World<>();
        world.setGravity(0, 0);
        world.setSettings(settings);
        world.setBounds(null);

        world.addStepListener(new PostUpdateStepListener());
        world.addStepListener(new InterpolationStepListener());
        world.addCollisionListener(new DamageCollisionListener());
        world.addCollisionListener(new CollisionListener());
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


    public void debugRender() {
        if (!DEBUG_RENDER) {
            return;
        }
        world.getBodies().forEach(body -> {
            GL30.glBegin(GL30.GL_LINE_LOOP);
            final Rectangle rectangle = (Rectangle) body.getFixture(0).getShape();
            final Vector2[] originVertices = rectangle.getVertices();
            final List<Vector2> vertices = new ArrayList<>(originVertices.length);
            for (final Vector2 originVertex : originVertices){
                vertices.add(new Vector2(originVertex.x, originVertex.y));
            }
            final Vector2f cameraPos = Camera.getPosition();
            for (Vector2 vertex : vertices) {
                final Vector2 transformed = body.getWorldPoint(vertex);
                GL30.glVertex2d(transformed.x - cameraPos.x, transformed.y - cameraPos.y);
            }
            GL30.glEnd();
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

    public List<PhysicsObject> query(final Vector2f point, final Vector2i dimensions, final Set<CollisionType> ignored) {
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
            results.add(object);
        }
        return List.copyOf(results);
    }

    public boolean queryExists(final Vector2f point, final Vector2i dimensions, final Set<CollisionType> ignored) {
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
