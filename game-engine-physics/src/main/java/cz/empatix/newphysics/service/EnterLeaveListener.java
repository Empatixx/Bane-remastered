package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.collision.BasicCollisionPair;
import org.dyn4j.collision.CollisionPair;
import org.dyn4j.dynamics.TimeStep;
import org.dyn4j.world.PhysicsWorld;
import org.dyn4j.world.World;
import org.dyn4j.world.listener.CollisionListener;
import org.dyn4j.world.listener.StepListener;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class EnterLeaveListener implements StepListener<PhysicsObject> {
    private final Set<CollisionPair<PhysicsObject>> collisionPairs = new HashSet<>();

    @Override
    public void begin(TimeStep step, PhysicsWorld<PhysicsObject, ?> world) {

    }

    @Override
    public void updatePerformed(TimeStep step, PhysicsWorld<PhysicsObject, ?> world) {
    }

    @Override
    public void postSolve(TimeStep step, PhysicsWorld<PhysicsObject, ?> world) {
        collisionPairs.removeIf(pair -> {
            final PhysicsObject first = pair.getFirst();
            final PhysicsObject second = pair.getSecond();
            final boolean leave = !world.isInContact(first, second);
            if (leave){
                first.onLeave(second);
                second.onLeave(first);
            }
            return leave;
        });
    }

    @Override
    public void end(TimeStep step, PhysicsWorld<PhysicsObject, ?> world) {

    }

    public void enter(final PhysicsObject a, final PhysicsObject b) {
        final boolean added = collisionPairs.add(new BasicCollisionPair<>(a, b));
        if (added){
            a.onEnter(b);
            b.onEnter(a);
        }
    }
}
