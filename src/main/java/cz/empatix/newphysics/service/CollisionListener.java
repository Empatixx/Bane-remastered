package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.collision.Fixture;
import org.dyn4j.dynamics.Body;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.world.BroadphaseCollisionData;
import org.dyn4j.world.ManifoldCollisionData;
import org.dyn4j.world.NarrowphaseCollisionData;

public class CollisionListener implements org.dyn4j.world.listener.CollisionListener<PhysicsObject, BodyFixture> {

    @Override
    public boolean collision(BroadphaseCollisionData<PhysicsObject, BodyFixture> collision) {
        return checkCollision(collision);
    }

    private boolean checkCollision(BroadphaseCollisionData<PhysicsObject, BodyFixture> collision) {
        final PhysicsObject o1 = collision.getBody1();
        final PhysicsObject o2 = collision.getBody2();
        final boolean canCollide = canCollide(o1, o2);
        if (!canCollide) {
            return false;
        }
        o1.onCollision();
        o2.onCollision();
        return true;
    }

    @Override
    public boolean collision(NarrowphaseCollisionData<PhysicsObject, BodyFixture> collision) {
        return checkCollision(collision);
    }

    @Override
    public boolean collision(ManifoldCollisionData<PhysicsObject, BodyFixture> collision) {
        return checkCollision(collision);
    }
    private boolean canCollide(final PhysicsObject o1, final PhysicsObject o2){
        final CollisionType t1 = o1.getType();
        final CollisionType t2 = o2.getType();

        if (t1 == null || t2 == null) { // default
            return true;
        }
        return t1.collidesWith(t2);
    }
}
