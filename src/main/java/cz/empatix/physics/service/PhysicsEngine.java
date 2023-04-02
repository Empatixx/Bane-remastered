package cz.empatix.physics.service;

import cz.empatix.physics.domain.CollisionObject;
import org.dyn4j.world.PhysicsWorld;
import org.dyn4j.world.World;

public class PhysicsEngine {
    private static World<CollisionObject> world;

    public PhysicsEngine() {
        world = new World<>();
        world.setGravity(PhysicsWorld.ZERO_GRAVITY);
        world.getSettings().setStepFrequency(1/60.0);
        world.getSettings().setMaximumTranslation(20);
    }
    public void update(final double dt){
        world.step(1);
        world.getBodies().forEach(CollisionObject::postPhysicsUpdate);
    }
    public void reset(){
        world.removeAllBodies();
    }
    public static void addBody(final CollisionObject body){
        world.addBody(body);
    }
}
