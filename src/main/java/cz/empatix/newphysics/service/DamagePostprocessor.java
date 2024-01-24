package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.DamageSource;
import cz.empatix.newphysics.attributes.Damageable;
import cz.empatix.newphysics.attributes.PhysicsObject;

public class DamagePostprocessor implements CollisionPostprocessor {

    public void postprocessDamageable(PhysicsObject a, PhysicsObject b) {
        if (a.getOwner() instanceof Damageable d) {
            if (b.getOwner() instanceof DamageSource damageSource) {
                if (damageSource.damage() <= 0) {
                    return;
                }
                d.hit(damageSource.type(), damageSource.damage());
            }
        }
    }

    @Override
    public void postprocess(PhysicsObject o1, PhysicsObject o2) {
        postprocessDamageable(o1, o2);
        postprocessDamageable(o2, o1);
    }
}
