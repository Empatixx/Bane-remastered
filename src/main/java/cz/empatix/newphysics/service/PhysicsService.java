package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.Collisionable;
import cz.empatix.newphysics.attributes.PhysicsObject;

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
    public void update(final double dt){
        physicsEngine.update(dt);
    }

}
