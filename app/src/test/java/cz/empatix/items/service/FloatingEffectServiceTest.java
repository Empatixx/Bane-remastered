package cz.empatix.items.service;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertTrue;

/** Sanity coverage of {@link FloatingEffectService} offset bounds and update loop. */
class FloatingEffectServiceTest {

    @Test
    void offsetStaysWithinAmplitudeAndAdvancesWithoutError() {
        final FloatingEffectService service = new FloatingEffectService(2.5f, 0.25f);

        // Drive several updates; the bobbing offset must stay within +/- amplitude.
        for (int i = 0; i < 50; i++) {
            service.update(0.1f);
            final float y = service.getOffsetY();
            assertTrue(y >= -2.5f - 1e-3f && y <= 2.5f + 1e-3f,
                    "offset out of bounds: " + y);
        }
    }

    @Test
    void defaultConstructorIsUsable() {
        final FloatingEffectService service = new FloatingEffectService();
        service.update(0.05f);
        final float y = service.getOffsetY();
        assertTrue(y >= -2.5f - 1e-3f && y <= 2.5f + 1e-3f);
    }
}
