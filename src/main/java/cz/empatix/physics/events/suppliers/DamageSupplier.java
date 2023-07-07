package cz.empatix.physics.events.suppliers;

import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.events.domain.DamageEvent;
import cz.empatix.physics.events.domain.EventType;

import java.util.function.Supplier;

public class DamageSupplier implements EventSupplier{
    private final Supplier<Integer> damageSupplier;
    public DamageSupplier(final Supplier<Integer> damageSupplier) {
        this.damageSupplier = damageSupplier;
    }

    @Override
    public DamageEvent supply(final PhysicsBody bodyA,final PhysicsBody bodyB) {
        return new DamageEvent(bodyA, bodyB, damageSupplier.get());
    }

    @Override
    public EventType getType() {
        return EventType.DAMAGE;
    }
}
