package cz.empatix.hud.service;

import cz.empatix.hud.domain.StatusBarType;
import cz.empatix.render.domain.RenderData;
import cz.empatix.shaderbindings.StatusBarBinding;
import cz.empatix.testsupport.HeadlessAssets;
import cz.empatix.testsupport.MockedAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Headless coverage for the asset-coupled {@link StatusBar}: the value-supplier-driven fill mapping
 * exposed through {@link StatusBarBinding}, the delayed (smoothed) fill, and the layout getters that
 * derive from {@link StatusBarType} scale. The bar fetches its texture through the GL-free fakes from
 * {@link MockedAssets} so the quad math in {@code renderData()} produces finite, in-range values.
 */
@ExtendWith(HeadlessAssets.class)
class StatusBarTest {

    private static StatusBarBinding bindingOf(final StatusBar bar) {
        final RenderData data = bar.renderData();
        return (StatusBarBinding) data.shaderProgramBinding();
    }

    @Test
    void fillAmountIsCurrentOverMaxFromTheSuppliers() {
        final StatusBar bar = new StatusBar(() -> 5, () -> 10, StatusBarType.HEALTH, new Vector2f(0, 0));

        assertEquals(0.5f, bindingOf(bar).fillAmount(), 1e-6f);
    }

    @Test
    void fillAmountTracksSupplierChangesAfterUpdate() {
        final int[] current = {10};
        final StatusBar bar = new StatusBar(() -> current[0], () -> 10, StatusBarType.HEALTH, new Vector2f(0, 0));
        assertEquals(1f, bindingOf(bar).fillAmount(), 1e-6f);

        current[0] = 3;
        bar.update(0.016);

        assertEquals(0.3f, bindingOf(bar).fillAmount(), 1e-6f);
    }

    @Test
    void fullBarReportsAFullDelayedFill() {
        final StatusBar bar = new StatusBar(() -> 10, () -> 10, StatusBarType.ARMOR, new Vector2f(0, 0));

        // a full bar that never drops keeps both fills pinned at 1
        bar.update(0.016);

        assertEquals(1f, bindingOf(bar).fillAmount(), 1e-6f);
        assertEquals(1f, bindingOf(bar).delayedFillAmount(), 1e-6f);
    }

    // NOTE: the delayed-fill smoothing is driven by wall-clock elapsed time (not the dt arg), so it
    // can't be asserted deterministically headlessly. The tween itself is covered by StatusControllerTest.

    @Test
    void barColorComesFromTheStatusBarType() {
        final StatusBar bar = new StatusBar(() -> 1, () -> 1, StatusBarType.HEALTH, new Vector2f(0, 0));

        final StatusBarBinding binding = bindingOf(bar);
        assertEquals(StatusBarType.HEALTH.getBarColor(), binding.barColor());
    }

    @Test
    void widthAndXOffsetAreScaledByTheType() {
        final StatusBar bar = new StatusBar(() -> 1, () -> 1, StatusBarType.HEALTH, new Vector2f(0, 0));

        final int scale = StatusBarType.HEALTH.getScale();
        assertEquals(StatusBarType.HEALTH.getTexture().width() * scale, bar.getWidth());
        assertEquals(StatusBarType.HEALTH.getXOffset() * scale, bar.getXOffset());
    }

    @Test
    void positionAndZIndexAndRenderabilityAreStable() {
        final Vector2f pos = new Vector2f(25, 50);
        final StatusBar bar = new StatusBar(() -> 1, () -> 1, StatusBarType.ARMOR, pos);

        assertEquals(pos, bar.getPosition());
        assertEquals(1, bar.getZIndex());
        assertTrue(bar.canRender());
    }

    @Test
    void zeroMaxValueYieldsAFullFillGuard() {
        // StatusController.delta() returns 1 when max == 0
        final StatusBar bar = new StatusBar(() -> 0, () -> 0, StatusBarType.BOSS_HEALTH, new Vector2f(0, 0));

        assertEquals(1f, bindingOf(bar).fillAmount(), 1e-6f);
    }
}
