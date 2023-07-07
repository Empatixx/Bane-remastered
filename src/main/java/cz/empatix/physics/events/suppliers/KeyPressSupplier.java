package cz.empatix.physics.events.suppliers;

import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.events.domain.Event;
import cz.empatix.physics.events.domain.EventType;
import cz.empatix.physics.events.domain.KeyPressEvent;

import java.util.HashSet;
import java.util.Set;

public class KeyPressSupplier implements EventSupplier{
    private final Set<Integer> keys = new HashSet<>(20);
    public void addKey(final int key) {
        keys.add(key);
    }
    @Override
    public Event supply(final PhysicsBody bodyA, final PhysicsBody bodyB) {
        try{
            return new KeyPressEvent(Set.copyOf(keys));
        } finally {
            keys.clear();
        }
    }

    @Override
    public EventType getType() {
        return EventType.KEY_PRESS;
    }
}
