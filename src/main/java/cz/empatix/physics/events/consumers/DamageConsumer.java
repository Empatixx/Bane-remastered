package cz.empatix.physics.events.consumers;

import cz.empatix.physics.events.domain.DamageEvent;
import cz.empatix.physics.events.domain.Event;
import cz.empatix.physics.events.domain.EventType;

import java.util.function.Consumer;

public class DamageConsumer implements EventConsumer {
    private final Consumer<Integer> damageConsumer;
    public DamageConsumer(final Consumer<Integer> damageConsumer) {
        this.damageConsumer = damageConsumer;
    }
    @Override
    public void consume(final Event event) {
        final DamageEvent damageEvent = (DamageEvent) event;
        damageConsumer.accept(damageEvent.damage());
    }

    @Override
    public EventType getType() {
        return EventType.DAMAGE;
    }
}
