package cz.empatix.physics.events.domain;

import cz.empatix.physics.domain.PhysicsBody;

public record DamageEvent(
        PhysicsBody bodyA,
        PhysicsBody bodyB,
        int damage
) implements Event{
    @Override
    public EventType getType() {
        return EventType.DAMAGE;
    }
}
