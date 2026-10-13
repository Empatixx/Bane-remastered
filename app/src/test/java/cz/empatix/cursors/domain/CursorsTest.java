package cz.empatix.cursors.domain;

import cz.empatix.render.domain.RenderScene;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Headless coverage for the concrete {@link Cursor} implementations. Each cursor wraps a GL-free
 * image (see {@link HeadlessAssets}); we assert the type tag, the shared overlay scene/z-index
 * defaults from the {@link Cursor} interface, the renderability flag, and that {@code updateMouse}
 * runs without touching GL. The GL-bound {@code renderData} draw path is intentionally not driven.
 */
@ExtendWith(HeadlessAssets.class)
class CursorsTest {

    @Test
    void defaultCursorReportsItsTypeAndRenders() {
        final DefaultCursor cursor = new DefaultCursor();

        assertEquals(CursorType.DEFAULT, cursor.getType());
        assertTrue(cursor.canRender());
        assertEquals(RenderScene.OVERLAYS, cursor.renderScene());
        assertEquals(-200, cursor.getZIndex());

        cursor.updateMouse(120f, 90f, 0.016f); // no throw
    }

    @Test
    void aimCursorIsACrosshairThatRenders() {
        final AimCursor cursor = new AimCursor();

        assertEquals(CursorType.CROSSHAIR, cursor.getType());
        assertTrue(cursor.canRender());
        assertEquals(RenderScene.OVERLAYS, cursor.renderScene());

        cursor.updateMouse(10f, 20f, 0.016f);
    }

    @Test
    void reloadCursorAnimatesOnMouseUpdate() {
        final ReloadCursor cursor = new ReloadCursor();

        assertEquals(CursorType.RELOAD, cursor.getType());
        assertTrue(cursor.canRender());

        cursor.updateMouse(50f, 60f, 0.5f); // advances the animation; must not touch GL
        cursor.updateMouse(50f, 60f, 0.5f);
    }

    @Test
    void disabledCursorIsADefaultThatNeitherRendersNorMoves() {
        final DisabledCursor cursor = new DisabledCursor();

        assertEquals(CursorType.DISABLED, cursor.getType());
        assertFalse(cursor.canRender(), "the disabled cursor is hidden");
        // overrides updateMouse to a no-op -> still safe to call
        cursor.updateMouse(999f, 999f, 0.016f);
    }

    @Test
    void everyCursorTypeIsDistinct() {
        assertEquals(CursorType.DEFAULT, new DefaultCursor().getType());
        assertEquals(CursorType.CROSSHAIR, new AimCursor().getType());
        assertEquals(CursorType.RELOAD, new ReloadCursor().getType());
        assertEquals(CursorType.DISABLED, new DisabledCursor().getType());
    }
}
