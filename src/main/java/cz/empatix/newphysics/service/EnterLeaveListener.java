package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.PhysicsObject;

import java.util.HashSet;
import java.util.Set;

public class EnterLeaveListener {
    private record ActiveCollisionPair<E>(PhysicsObject a, PhysicsObject b, E data) {
    }
    private final Set<ActiveCollisionPair<PhysicsObject>> activeCollisions = new HashSet<>();
    public void begin(PhysicsObject a, PhysicsObject b) {
        activeCollisions.add(new ActiveCollisionPair<>(a, b, null));
    }
    public void end(PhysicsObject a, PhysicsObject b) {
        activeCollisions.remove(new ActiveCollisionPair<>(a, b, null));
    }
}
