package cz.empatix.physics;

import org.dyn4j.world.World;

public class PhysicsEngine {
    private static World<PhysicsBody> world;

    public PhysicsEngine() {
        world = new World<>();
        world.setGravity(0f,0f);
    }
    public void update(final double dt){
        world.update(dt);
        world.getBodies().forEach(PhysicsBody::updatePosition);
    }
    public void reset(){
        world.removeAllBodies();
    }
    public static void addBody(final PhysicsBody body){
        world.addBody(body);
    }
}
