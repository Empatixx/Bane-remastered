package cz.empatix.physics.events.domain;

import cz.empatix.physics.domain.PhysicsBody;

public record DamageEvent(
        PhysicsBody bodyA,
        PhysicsBody bodyB,
        DamageSource source,
        int damage,
        Runnable onDamageCallback
) implements Event{
    @Override
    public EventType getType() {
        return EventType.DAMAGE;
    }
}
