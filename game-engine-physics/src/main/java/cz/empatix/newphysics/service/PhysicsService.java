package cz.empatix.newphysics.service;

import cz.empatix.core.CollisionCategory;
import cz.empatix.core.CollisionHandler;
import cz.empatix.newphysics.attributes.PhysicsObject;
import org.joml.Vector2f;
import org.joml.Vector2i;

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
                                     final Set<? extends CollisionCategory> ignored) {
        return physicsEngine.query(point, dimensions, ignored);
    }

    public boolean queryExists(final Vector2f point, final Vector2i dimensions,
                               final Set<? extends CollisionCategory> ignored) {
        return physicsEngine.queryExists(point, dimensions, ignored);
    }

    public void addCollisionHandler(final CollisionHandler handler) {
        physicsEngine.addCollisionHandler(handler);
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

    public void debugRender(final PhysicsDebugRenderer renderer) {
        physicsEngine.debugRender(renderer);
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
