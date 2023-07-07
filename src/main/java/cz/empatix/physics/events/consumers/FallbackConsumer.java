package cz.empatix.physics.events.consumers;

import cz.empatix.physics.events.domain.Event;
import cz.empatix.physics.events.domain.EventType;

public class FallbackConsumer implements EventConsumer{
    @Override
    public void consume(Event e) {

    }

    @Override
    public EventType getType() {
        return null;
    }
}
