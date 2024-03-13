package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.dynamics.TimeStep;
import org.dyn4j.world.PhysicsWorld;
import org.dyn4j.world.listener.StepListener;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class PostUpdateStepListener implements StepListener<PhysicsObject> {
    private final static Logger LOGGER = LoggerFactory.getLogger(PostUpdateStepListener.class);
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
        for (final PhysicsObject physicsObject : world.getBodies()) {
            if (physicsObject.canRemove()){
                continue;
            }
            physicsObject.postPhysicsUpdate();
        }
    }
}
