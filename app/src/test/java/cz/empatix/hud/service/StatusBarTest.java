package cz.empatix.hud.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.hud.domain.StatusBarType;
import cz.empatix.render.domain.RenderData;
import cz.empatix.shaderbindings.StatusBarBinding;
import cz.empatix.testsupport.MockedAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mockStatic;

/**
 * Headless coverage for the asset-coupled {@link StatusBar}: the value-supplier-driven fill mapping
 * exposed through {@link StatusBarBinding}, the delayed (smoothed) fill, and the layout getters that
 * derive from {@link StatusBarType} scale. The bar fetches its texture through the GL-free fakes from
 * {@link MockedAssets} so the quad math in {@code renderData()} produces finite, in-range values.
 */
class StatusBarTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    private static StatusBarBinding bindingOf(final StatusBar bar) {
        final RenderData data = bar.renderData();
        return (StatusBarBinding) data.shaderProgramBinding();
    }

    @Test
    void fillAmountIsCurrentOverMaxFromTheSuppliers() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final StatusBar bar = new StatusBar(() -> 5, () -> 10, StatusBarType.HEALTH, new Vector2f(0, 0));

            assertEquals(0.5f, bindingOf(bar).fillAmount(), 1e-6f);
        }
    }

    @Test
    void fillAmountTracksSupplierChangesAfterUpdate() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final int[] current = {10};
            final StatusBar bar = new StatusBar(() -> current[0], () -> 10, StatusBarType.HEALTH, new Vector2f(0, 0));
            assertEquals(1f, bindingOf(bar).fillAmount(), 1e-6f);

            current[0] = 3;
            bar.update(0.016);

            assertEquals(0.3f, bindingOf(bar).fillAmount(), 1e-6f);
        }
    }

    @Test
    void fullBarReportsAFullDelayedFill() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final StatusBar bar = new StatusBar(() -> 10, () -> 10, StatusBarType.ARMOR, new Vector2f(0, 0));

            // a full bar that never drops keeps both fills pinned at 1
            bar.update(0.016);

            assertEquals(1f, bindingOf(bar).fillAmount(), 1e-6f);
            assertEquals(1f, bindingOf(bar).delayedFillAmount(), 1e-6f);
        }
    }

    // NOTE: the delayed-fill smoothing is driven by wall-clock elapsed time (not the dt arg), so it
    // can't be asserted deterministically headlessly. The tween itself is covered by StatusControllerTest.

    @Test
    void barColorComesFromTheStatusBarType() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final StatusBar bar = new StatusBar(() -> 1, () -> 1, StatusBarType.HEALTH, new Vector2f(0, 0));

            final StatusBarBinding binding = bindingOf(bar);
            assertEquals(StatusBarType.HEALTH.getBarColor(), binding.barColor());
        }
    }

    @Test
    void widthAndXOffsetAreScaledByTheType() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final StatusBar bar = new StatusBar(() -> 1, () -> 1, StatusBarType.HEALTH, new Vector2f(0, 0));

            final int scale = StatusBarType.HEALTH.getScale();
            assertEquals(StatusBarType.HEALTH.getTexture().width() * scale, bar.getWidth());
            assertEquals(StatusBarType.HEALTH.getXOffset() * scale, bar.getXOffset());
        }
    }

    @Test
    void positionAndZIndexAndRenderabilityAreStable() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Vector2f pos = new Vector2f(25, 50);
            final StatusBar bar = new StatusBar(() -> 1, () -> 1, StatusBarType.ARMOR, pos);

            assertEquals(pos, bar.getPosition());
            assertEquals(1, bar.getZIndex());
            assertTrue(bar.canRender());
        }
    }

    @Test
    void zeroMaxValueYieldsAFullFillGuard() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            // StatusController.delta() returns 1 when max == 0
            final StatusBar bar = new StatusBar(() -> 0, () -> 0, StatusBarType.BOSS_HEALTH, new Vector2f(0, 0));

            assertEquals(1f, bindingOf(bar).fillAmount(), 1e-6f);
        }
    }
}
