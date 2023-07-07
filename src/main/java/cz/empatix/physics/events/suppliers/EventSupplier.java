package cz.empatix.physics.events.suppliers;

import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.events.domain.Event;
import cz.empatix.physics.events.domain.EventType;

public interface EventSupplier {
    Event supply(final PhysicsBody bodyA, final PhysicsBody bodyB);
    EventType getType();
}
