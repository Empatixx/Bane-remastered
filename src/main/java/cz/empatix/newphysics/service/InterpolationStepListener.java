package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.InterpolatedPhysicsObject;
import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.dynamics.TimeStep;
import org.dyn4j.world.PhysicsWorld;
import org.dyn4j.world.listener.StepListener;

import java.util.List;

public class InterpolationStepListener implements StepListener<PhysicsObject> {

    @Override
    public void begin(TimeStep step, PhysicsWorld<PhysicsObject, ?> world) {

    }

    @Override
    public void updatePerformed(TimeStep step, PhysicsWorld<PhysicsObject, ?> world) {
    }

    @Override
    public void postSolve(TimeStep step, PhysicsWorld<PhysicsObject, ?> world) {
    }

    @Override
    public void end(TimeStep step, PhysicsWorld<PhysicsObject, ?> world) {
        final List<PhysicsObject> bodies = world.getBodies();
        for (PhysicsObject body : bodies) {
            if (body instanceof InterpolatedPhysicsObject o) {
                o.updateInterpolatedPosition();
            }
        }
    }
}
