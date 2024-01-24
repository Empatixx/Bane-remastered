package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.AABB;
import cz.empatix.newphysics.attributes.Collisionable;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.physics.service.PhysicsService;

import java.util.ArrayList;
import java.util.List;

public class PhysicsEngine {
    private final List<PhysicsObject> physicsObjects;
    public PhysicsEngine(){
        physicsObjects = new ArrayList<>(50);
    }
    public void add(final PhysicsObject object){
        physicsObjects.add(object);
    }
    public void update(final double dt){
        for (PhysicsObject physicsObject : physicsObjects) {
            physicsObject.update(dt);
        }
        for (int i = 0; i < physicsObjects.size(); i++) {
            for (int j = i + 1; j < physicsObjects.size(); j++) {
                final PhysicsObject object1 = physicsObjects.get(i);
                final PhysicsObject object2 = physicsObjects.get(j);

                final Collisionable shape1 = object1.getShape();
                final Collisionable shape2 = object2.getShape();

                final boolean supportsAABB = shape1.supportsAABB() && shape2.supportsAABB();
                if(supportsAABB){
                    final AABB aabb1 = (AABB) shape1;
                    final AABB aabb2 = (AABB) shape2;
                    if(aabb1.xMin() < aabb2.xMax() &&
                            aabb1.xMax() > aabb2.xMin() &&
                            aabb1.yMin() < aabb2.yMax() &&
                            aabb1.yMax() > aabb2.yMin()){
                        aabb1
                    }
                }
            }
        }
    }
}
