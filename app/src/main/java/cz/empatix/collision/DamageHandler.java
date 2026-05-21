package cz.empatix.collision;

import cz.empatix.core.CollisionHandler;

/**
 * Applies game damage when a {@link DamageSource} collides with a {@link Damageable}.
 * Registered into the physics engine via
 * {@code PhysicsService.addCollisionHandler(...)} so the engine itself stays
 * agnostic of combat rules.
 */
public class DamageHandler implements CollisionHandler {

    @Override
    public void onCollision(final Object a, final Object b) {
        applyDamage(a, b);
        applyDamage(b, a);
    }

    private void applyDamage(final Object target, final Object source) {
        if (target instanceof Damageable damageable && source instanceof DamageSource damageSource) {
            if (damageSource.damage() <= 0) {
                return;
            }
            final int received = damageable.hit(damageSource.type(), damageSource.damage());
            damageSource.postDamage(received);
        }
    }
}
