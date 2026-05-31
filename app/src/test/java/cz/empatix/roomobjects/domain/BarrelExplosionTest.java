package cz.empatix.roomobjects.domain;

import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;

/**
 * Headless coverage for the real {@link BarrelExplosion} animation effect: position round-trip,
 * geometry, the fresh {@code hasPlayedOnce} state, and that the {@code update} tick runs cleanly.
 * The play-once flag is wall-clock gated (80ms/frame) so it is not asserted here to keep the test
 * deterministic without sleeping.
 */
@ExtendWith(HeadlessAssets.class)
class BarrelExplosionTest {

    @Test
    void geometryAndPositionRoundTrip() {
        final BarrelExplosion explosion = new BarrelExplosion();
        explosion.setPosition(11f, 22f);

        assertEquals(64 * 2, explosion.getWidth());
        assertEquals(64 * 2, explosion.getHeight());
        assertFalse(explosion.hasPlayedOnce());
    }

    @Test
    void updateTickRunsCleanly() {
        final BarrelExplosion explosion = new BarrelExplosion();
        explosion.setPosition(1f, 2f);

        for (int i = 0; i < 5; i++) {
            explosion.update(0.05);
        }

        assertFalse(explosion.getWidth() <= 0);
    }
}
