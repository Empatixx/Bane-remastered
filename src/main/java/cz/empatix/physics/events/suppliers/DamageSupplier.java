package cz.empatix.physics.events.suppliers;

import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.events.domain.DamageEvent;
import cz.empatix.physics.events.domain.DamageSource;
import cz.empatix.physics.events.domain.EventType;

import java.util.function.Supplier;

public class DamageSupplier implements EventSupplier {
    private final static Runnable NOOP = () -> {};
    private final DamageSource damageSource;
    private final Supplier<Integer> damageSupplier;
    private final Runnable onDamageCallback;

    public DamageSupplier(final DamageSource damageSource,
                          final Supplier<Integer> damageSupplier,
                          final Runnable onDamageCallback) {
        this.onDamageCallback = onDamageCallback;
        this.damageSupplier = damageSupplier;
        this.damageSource = damageSource;
    }
    public DamageSupplier(final DamageSource damageSource,
                          final Supplier<Integer> damageSupplier) {
        this(damageSource, damageSupplier, NOOP);
    }

    @Override
    public DamageEvent supply(final PhysicsBody bodyA, final PhysicsBody bodyB) {
        return new DamageEvent(bodyA, bodyB, damageSource, damageSupplier.get(), onDamageCallback);
    }

    @Override
    public EventType getType() {
        return EventType.DAMAGE;
    }
}
