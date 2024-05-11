package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.geometry.Shape;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class PhysicsService {
    private static PhysicsService instance;

    public static PhysicsService getInstance() {
        if (instance == null) {
            instance = new PhysicsService();
        }
        return instance;
    }

    private final PhysicsEngine physicsEngine;

    public PhysicsService() {
        physicsEngine = new PhysicsEngine();
    }

    public List<PhysicsObject> query(final Vector2f point, final Vector2i dimensions,
                                     final Set<CollisionType> ignored) {
        return physicsEngine.query(point, dimensions, ignored);
    }

    public boolean queryExists(final Vector2f point, final Vector2i dimensions,
                               final Set<CollisionType> ignored) {
        return physicsEngine.queryExists(point, dimensions, ignored);
    }

    public List<PhysicsObject> queryType(Vector2f targetPosition, Vector2i dimensions, Set<CollisionType> types) {
        final Set<CollisionType> ignored = new HashSet<>(Set.of(CollisionType.values()));
        ignored.removeAll(types);
        return physicsEngine.query(targetPosition, dimensions, ignored);
    }

    public void updateBroadphase() {
        physicsEngine.updateBroadphase();
    }

    public void add(final PhysicsObject physicsObject) {
        physicsEngine.add(physicsObject);
    }

    public void fixedUpdate(double dt) {
        physicsEngine.fixedUpdate(dt);
    }

    public void debugRender() {
        physicsEngine.debugRender();
    }

    public void clear() {
        physicsEngine.clear();
    }

    public boolean isColliding(final PhysicsObject object) {
        return physicsEngine.isColliding(object);
    }

    public void remove(final PhysicsObject physicsObject) {
        physicsEngine.remove(physicsObject);
    }

    public void update(double dt) {
        physicsEngine.update(dt);
    }

}
