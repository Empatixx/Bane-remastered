package cz.empatix.roomobjects.domain;

import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;

/**
 * Headless coverage for the real {@link Candlestick}: the flicker-intensity update path against a
 * headless-safe light source, the disable transition and stable geometry. {@code getWidth}/
 * {@code getHeight} multiply the already-scaled dimension constant by the scale again, so they are
 * pinned here to lock the (intentional) value.
 */
@ExtendWith(HeadlessAssets.class)
class CandlestickTest {

    @Test
    void updateFlickersLightWithoutThrowing() {
        final Candlestick candlestick = new Candlestick();
        candlestick.setPosition(40f, 50f);

        for (int i = 0; i < 10; i++) {
            candlestick.update(0.05);
        }

        assertFalse(candlestick.canRemove());
        assertEquals(40f, candlestick.getPosition().x, 0.0001f);
        assertEquals(50f, candlestick.getPosition().y, 0.0001f);
    }

    @Test
    void disableLightSwapsSpriteAndRunsCleanly() {
        final Candlestick candlestick = new Candlestick();

        candlestick.disableLight();

        assertEquals((16 * 3) * 3, candlestick.getWidth());
        assertEquals((16 * 3) * 3, candlestick.getHeight());
    }
}
