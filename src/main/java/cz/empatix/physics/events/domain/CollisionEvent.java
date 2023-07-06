package cz.empatix.physics.events.domain;

import cz.empatix.physics.domain.PhysicsBody;
import org.joml.Vector2f;

public record CollisionEvent (
        PhysicsBody bodyA,
        PhysicsBody bodyB,
        Vector2f normal,
        float depth
) implements Event{

    @Override
    public EventType getType() {
        return EventType.COLLISION;
    }
}