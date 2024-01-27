package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.geometry.Shape;
import org.joml.Vector2f;

public class PhysicsService {
    private static PhysicsService instance;
    public static PhysicsService getInstance() {
        if (instance == null) {
            instance = new PhysicsService();
        }
        return instance;
    }
    private final PhysicsEngine physicsEngine;
    public PhysicsService(){
        physicsEngine = new PhysicsEngine();
    }

    public void add(final PhysicsObject physicsObject){
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
