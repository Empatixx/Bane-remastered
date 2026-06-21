package cz.empatix.common;

import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Deterministic coverage of the seeded {@link RngService}: the same seed must reproduce the same
 * sequence (the whole point of a seedable RNG — daily runs, replays, future multiplayer determinism),
 * and the convenience bounds must hold. Pure; no GL, no harness.
 */
class RngServiceTest {

    @Test
    void sameSeedReproducesTheSameSequence() {
        final RngService a = new RngService(42L);
        final RngService b = new RngService(42L);

        for (int i = 0; i < 100; i++) {
            assertEquals(a.nextInt(1000), b.nextInt(1000), "same seed -> identical draw " + i);
        }
    }

    @Test
    void nextIntBoundStaysInRange() {
        final RngService rng = new RngService(1L);

        for (int i = 0; i < 1000; i++) {
            final int v = rng.nextInt(5);
            assertTrue(v >= 0 && v < 5, "value " + v + " out of [0,5)");
        }
    }

    @Test
    void nextIntOriginBoundStaysInHalfOpenRange() {
        final RngService rng = new RngService(1L);

        for (int i = 0; i < 1000; i++) {
            final int v = rng.nextInt(10, 20);
            assertTrue(v >= 10 && v < 20, "value " + v + " out of [10,20)");
        }
    }

    @Test
    void seedIsExposed() {
        assertEquals(123L, new RngService(123L).seed());
    }

    @Test
    void pickReturnsAListElementDeterministically() {
        final List<String> list = List.of("a", "b", "c", "d");
        final RngService a = new RngService(7L);
        final RngService b = new RngService(7L);

        for (int i = 0; i < 50; i++) {
            final String fromA = a.pick(list);
            assertTrue(list.contains(fromA));
            assertEquals(fromA, b.pick(list), "same seed -> same pick");
        }
    }
}
