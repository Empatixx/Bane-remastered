package cz.empatix.physics.events.consumers;

import cz.empatix.physics.events.domain.Event;
import cz.empatix.physics.events.domain.EventType;
import cz.empatix.physics.events.domain.KeyPressEvent;

import java.util.Set;
import java.util.function.Consumer;

public class KeyPressConsumer implements EventConsumer {
    private final Consumer<Set<Integer>> consumer;
    public KeyPressConsumer(final Consumer<Set<Integer>> consumer) {
        this.consumer = consumer;
    }
    @Override
    public void consume(final Event e) {
        final KeyPressEvent keyPressEvent = (KeyPressEvent) e;
        consumer.accept(keyPressEvent.keys());
    }

    @Override
    public EventType getType() {
        return EventType.KEY_PRESS;
    }
}
