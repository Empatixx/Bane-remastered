package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.world.BroadphaseCollisionData;
import org.dyn4j.world.ManifoldCollisionData;
import org.dyn4j.world.NarrowphaseCollisionData;

public class CollisionListener implements org.dyn4j.world.listener.CollisionListener<PhysicsObject, BodyFixture> {
    @Override
    public boolean collision(BroadphaseCollisionData<PhysicsObject, BodyFixture> collision) {
        collision.getBody1().onCollision();
        collision.getBody2().onCollision();
        return true;
    }

    @Override
    public boolean collision(NarrowphaseCollisionData<PhysicsObject, BodyFixture> collision) {
        collision.getBody1().onCollision();
        collision.getBody2().onCollision();
        return true;
    }

    @Override
    public boolean collision(ManifoldCollisionData<PhysicsObject, BodyFixture> collision) {
        collision.getBody1().onCollision();
        collision.getBody2().onCollision();
        return true;
    }
}
