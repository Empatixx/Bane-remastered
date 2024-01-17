package cz.empatix.physics.events.consumers;

import cz.empatix.physics.events.domain.DamageEvent;
import cz.empatix.physics.events.domain.DamageSource;
import cz.empatix.physics.events.domain.Event;
import cz.empatix.physics.events.domain.EventType;

import java.util.function.BiPredicate;
import java.util.function.Consumer;
import java.util.function.Predicate;

public class DamageConsumer implements EventConsumer {
    private final static BiPredicate<DamageSource, Integer> DEFAULT_CANDAMAGE = (s,d) -> true;
    private final Consumer<Integer> damageConsumer;
    private final BiPredicate<DamageSource, Integer> canDamage;
    public DamageConsumer(final Consumer<Integer> damageConsumer) {
        this.damageConsumer = damageConsumer;
        canDamage = DEFAULT_CANDAMAGE;
    }
    public DamageConsumer(final Consumer<Integer> damageConsumer,
                          final BiPredicate<DamageSource, Integer> canDamage){
        this.damageConsumer = damageConsumer;
        this.canDamage = canDamage;
    }
    @Override
    public void consume(final Event event) {
        final DamageEvent damageEvent = (DamageEvent) event;
        if (!canDamage.test(damageEvent.source(), damageEvent.damage())){
            return;
        }
        if (damageEvent.damage() == 0){
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
