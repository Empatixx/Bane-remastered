package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.Collisionable;
import cz.empatix.newphysics.attributes.PhysicsObject;

public interface CollisionPostprocessor {
    void postprocess(final PhysicsObject o1, final PhysicsObject o2);
}
