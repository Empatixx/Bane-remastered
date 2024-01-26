package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.DamageSource;
import cz.empatix.newphysics.attributes.Damageable;
import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.world.BroadphaseCollisionData;
import org.dyn4j.world.ManifoldCollisionData;
import org.dyn4j.world.NarrowphaseCollisionData;
import org.dyn4j.world.listener.CollisionListener;

public class DamageCollisionListener implements CollisionListener<PhysicsObject, BodyFixture> {

    public void postprocessDamageable(Object a, Object b) {
        if (a instanceof Damageable d) {
            if (b instanceof DamageSource damageSource) {
                if (damageSource.damage() <= 0) {
                    return;
                }
                d.hit(damageSource.type(), damageSource.damage());
            }
        }
    }

    @Override
    public boolean collision(BroadphaseCollisionData<PhysicsObject, BodyFixture> collision) {
        final Object a = collision.getBody1().getUserData();
        final Object b = collision.getBody2().getUserData();

        postprocessDamageable(a, b);
        postprocessDamageable(b, a);
        final boolean notActive = !collision.getBody1().isActive() || !collision.getBody2().isActive();
        if (notActive) {
            return false;
        }
        return true;
    }

    @Override
    public boolean collision(NarrowphaseCollisionData<PhysicsObject, BodyFixture> collision) {
        final Object a = collision.getBody1().getUserData();
        final Object b = collision.getBody2().getUserData();
        final boolean notActive = !collision.getBody1().isActive() || !collision.getBody2().isActive();
        postprocessDamageable(a, b);
        postprocessDamageable(b, a);
        if (notActive) {
            return false;
        }
        return true;
    }

    @Override
    public boolean collision(ManifoldCollisionData<PhysicsObject, BodyFixture> collision) {
        final Object a = collision.getBody1().getUserData();
        final Object b = collision.getBody2().getUserData();

        final boolean notActive = !collision.getBody1().isActive() || !collision.getBody2().isActive();
        postprocessDamageable(a, b);
        postprocessDamageable(b, a);
        if (notActive) {
            return false;
        }
        return true;
    }
}
