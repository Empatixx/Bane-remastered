package cz.empatix.items.service;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Deterministic coverage of {@link ItemDespawnService} time gates. The spawn time is "now", so the
 * gates are driven purely through the public despawn-time setter:
 *   canDespawn = now - spawn  > despawnTime
 *   canBlink   = now - spawn  > despawnTime - 2000
 * With now - spawn ~= 0, a negative/zero despawnTime forces the gates without any sleeping.
 */
class ItemDespawnServiceTest {

    @Test
    void freshItemWithDefaultTimeDoesNotDespawnOrBlink() {
        final ItemDespawnService service = new ItemDespawnService();
        assertFalse(service.canDespawn());
        assertFalse(service.canBlink());
    }

    @Test
    void negativeDespawnTimeForcesImmediateDespawnAndBlink() {
        final ItemDespawnService service = new ItemDespawnService();
        service.setDespawnTime(-1L); // 0 > -1 -> despawn; 0 > -2001 -> blink
        assertTrue(service.canDespawn());
        assertTrue(service.canBlink());
    }

    @Test
    void smallPositiveDespawnTimeBlinksButDoesNotDespawnYet() {
        final ItemDespawnService service = new ItemDespawnService();
        service.setDespawnTime(1000L); // 0 > 1000 false (no despawn); 0 > -1000 true (blink)
        assertTrue(service.canBlink());
        assertFalse(service.canDespawn());
    }

    @Test
    void raisingDespawnTimeToMaxPreventsDespawn() {
        final ItemDespawnService service = new ItemDespawnService();
        service.setDespawnTime(-1L);
        assertTrue(service.canDespawn());

        service.setDespawnTime(Long.MAX_VALUE);
        assertFalse(service.canDespawn());
        assertFalse(service.canBlink());
    }
}
