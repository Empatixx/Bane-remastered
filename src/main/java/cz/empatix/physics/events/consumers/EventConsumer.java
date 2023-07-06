package cz.empatix.physics.events.consumers;

import cz.empatix.physics.events.domain.Event;
import cz.empatix.physics.events.domain.EventType;

public interface EventConsumer {
    void consume(final Event e);
    boolean accepts(final EventType eventType);
}
