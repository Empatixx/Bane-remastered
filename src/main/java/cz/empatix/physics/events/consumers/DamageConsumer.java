package cz.empatix.physics.events.consumers;

import cz.empatix.physics.events.domain.DamageEvent;
import cz.empatix.physics.events.domain.DamageSource;
import cz.empatix.physics.events.domain.Event;
import cz.empatix.physics.events.domain.EventType;

import java.util.function.Consumer;

public class DamageConsumer implements EventConsumer {
    private final DamageSource preventDamageSource;
    private final Consumer<Integer> damageConsumer;
    public DamageConsumer(final DamageSource source,
                          final Consumer<Integer> damageConsumer) {
        this.preventDamageSource = source;
        this.damageConsumer = damageConsumer;
    }
    @Override
    public void consume(final Event event) {
        final DamageEvent damageEvent = (DamageEvent) event;
        if (damageEvent.source() == preventDamageSource) {
            return;
        }
        damageConsumer.accept(damageEvent.damage());
        damageEvent.onDamageCallback().run();
    }

    @Override
    public EventType getType() {
        return EventType.DAMAGE;
    }
}
