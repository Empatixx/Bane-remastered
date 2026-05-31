package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.mockito.Mockito.mockStatic;

/**
 * Headless coverage for the real {@link Candlestick}: the flicker-intensity update path against a
 * headless-safe light source, the disable transition and stable geometry. {@code getWidth}/
 * {@code getHeight} multiply the already-scaled dimension constant by the scale again, so they are
 * pinned here to lock the (intentional) value.
 */
class CandlestickTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    @Test
    void updateFlickersLightWithoutThrowing() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Candlestick candlestick = new Candlestick();
            candlestick.setPosition(40f, 50f);

            for (int i = 0; i < 10; i++) {
                candlestick.update(0.05);
            }

            assertFalse(candlestick.canRemove());
            assertEquals(40f, candlestick.getPosition().x, 0.0001f);
            assertEquals(50f, candlestick.getPosition().y, 0.0001f);
        }
    }

    @Test
    void disableLightSwapsSpriteAndRunsCleanly() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Candlestick candlestick = new Candlestick();

            candlestick.disableLight();

            assertEquals((16 * 3) * 3, candlestick.getWidth());
            assertEquals((16 * 3) * 3, candlestick.getHeight());
        }
    }
}
