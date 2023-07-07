package cz.empatix.physics.events.domain;

import java.util.Set;

public record KeyPressEvent(
        Set<Integer> keys
) implements Event {
    @Override
    public EventType getType() {
        return EventType.KEY_PRESS;
    }
}
