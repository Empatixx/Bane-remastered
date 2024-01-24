package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.Collisionable;

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

    public void add(final Collisionable physicsObject){
        physicsEngine.add(physicsObject);
    }
    public void update(final double dt){
        physicsEngine.update(dt);
    }

}
