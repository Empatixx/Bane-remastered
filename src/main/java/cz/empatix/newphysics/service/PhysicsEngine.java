package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.AABB;
import cz.empatix.newphysics.attributes.Collisionable;
import cz.empatix.newphysics.attributes.PhysicsObject;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;

public class PhysicsEngine {
    private final List<PhysicsObject> physicsObjects;
    private final List<CollisionPostprocessor> collisionPostprocessors;

    public PhysicsEngine() {
        physicsObjects = new ArrayList<>(50);
        collisionPostprocessors = List.of(new DamagePostprocessor());
    }

    public void add(final PhysicsObject object) {
        physicsObjects.add(object);
    }

    public void update(final double dt) {
        for (PhysicsObject physicsObject : physicsObjects) {
            physicsObject.update(dt);
        }
        physicsObjects.removeIf(PhysicsObject::isRemove);
        for (int i = 0; i < physicsObjects.size(); i++) {
            for (int j = i + 1; j < physicsObjects.size(); j++) {
                final PhysicsObject object1 = physicsObjects.get(i);
                final PhysicsObject object2 = physicsObjects.get(j);

                final Collisionable shape1 = object1.getShape();
                final Collisionable shape2 = object2.getShape();

                final boolean supportsAABB = shape1.supportsAABB() && shape2.supportsAABB();
                if (supportsAABB) {
                    final AABB aabb1 = (AABB) shape1;
                    final AABB aabb2 = (AABB) shape2;
                    if (aabb1.xMin() < aabb2.xMax() &&
                            aabb1.xMax() > aabb2.xMin() &&
                            aabb1.yMin() < aabb2.yMax() &&
                            aabb1.yMax() > aabb2.yMin()) {
                        object1.onCollision();
                        object2.onCollision();
                        for (CollisionPostprocessor collisionPostprocessor : collisionPostprocessors) {
                            collisionPostprocessor.postprocess(object1, object2);
                        }
                        resolveCollision(aabb1, aabb2, object1, object2);
                    }
                }
            }
        }
    }
    private void resolveCollision(AABB aabb1, AABB aabb2, PhysicsObject object1, PhysicsObject object2) {
        final float xOverlap = Math.min(aabb1.xMax(), aabb2.xMax()) - Math.max(aabb1.xMin(), aabb2.xMin());
        final float yOverlap = Math.min(aabb1.yMax(), aabb2.yMax()) - Math.max(aabb1.yMin(), aabb2.yMin());

        if (xOverlap < yOverlap) {
            adjustPositionX(object1, object2, xOverlap);
        } else {
            adjustPositionY(object1, object2, yOverlap);
        }
    }
    private void adjustPositionX(PhysicsObject object1, PhysicsObject object2, final float overlap) {
        final float halfOverlap = overlap / 2;

        // Assuming getPosition() and setPosition() methods exist
        final Vector2f pos1 = object1.getShape().getPosition();
        final Vector2f pos2 = object1.getShape().getPosition();

        object1.getShape().setPosition(pos1.x() - halfOverlap, pos1.y());
        object2.getShape().setPosition(pos2.x() + halfOverlap, pos2.y());
    }

    private void adjustPositionY(PhysicsObject object1, PhysicsObject object2, float overlap) {
        float halfOverlap = overlap / 2;

        // Assuming getPosition() and setPosition() methods exist
        Vector2f pos1 = object1.getShape().getPosition();
        Vector2f pos2 = object1.getShape().getPosition();

        object1.getShape().setPosition(pos1.x(), pos1.y() - halfOverlap);
        object2.getShape().setPosition(pos2.x(), pos2.y() + halfOverlap);
    }
}
