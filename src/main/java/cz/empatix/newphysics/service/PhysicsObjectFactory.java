package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.Collisionable;
import cz.empatix.newphysics.attributes.PhysicsObject;

public class PhysicsObjectFactory {
    public static PhysicsObject create(final Collisionable shape) {
        final PhysicsObject physicsObject = new PhysicsObject(shape);
        PhysicsService.getInstance().add(physicsObject);
        return physicsObject;
    }
}
