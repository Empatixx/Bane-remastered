package cz.empatix.ui;

import cz.empatix.render.domain.RenderScene;
import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.concurrent.atomic.AtomicInteger;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Exercises the REAL {@link TransparentButton}. Its hit-box is derived from text metrics; with the
 * GL-free fake font those metrics are zero, leaving the constant 10px bonus offset, so a button at
 * (100,100) accepts hits strictly inside (90,110) on both axes.
 */
@ExtendWith(HeadlessAssets.class)
class TransparentButtonTest {

    @Test
    void mousePressedInsideRunsTheClickHandler() {
        final TransparentButton button = new TransparentButton("Back", new Vector2f(100, 100), 3);
        final AtomicInteger clicks = new AtomicInteger();
        button.onClick(clicks::incrementAndGet);

        button.mousePressed(0, 100, 100);

        assertEquals(1, clicks.get(), "a press inside the bonus-offset box fires onClick");
    }

    @Test
    void mousePressedOutsideDoesNotRunTheClickHandler() {
        final TransparentButton button = new TransparentButton("Back", new Vector2f(100, 100), 3);
        final AtomicInteger clicks = new AtomicInteger();
        button.onClick(clicks::incrementAndGet);

        button.mousePressed(0, 100, 200);

        assertEquals(0, clicks.get(), "a press outside the box does not fire onClick");
    }

    @Test
    void mousePressedWithNoHandlerIsANoOp() {
        final TransparentButton button = new TransparentButton("Back", new Vector2f(100, 100), 3);

        button.mousePressed(0, 100, 100); // no onClick — must not throw

        assertFalse(button.isHovered(), "no hover is computed until update() runs");
    }

    @Test
    void updateTogglesHoverFromMousePosition() {
        final TransparentButton button = new TransparentButton("Back", new Vector2f(100, 100), 3);

        button.update(0.016, 100, 100);
        assertTrue(button.isHovered(), "mouse on the button hovers it");

        button.update(0.016, 100, 200);
        assertFalse(button.isHovered(), "mouse off the button clears hover");
    }

    @Test
    void rendersOnOverlaysSceneAndProducesNoRenderData() {
        final TransparentButton button = new TransparentButton("Back", new Vector2f(100, 100), 3);

        assertEquals(RenderScene.OVERLAYS, button.renderScene(), "transparent button lives on OVERLAYS");
        assertFalse(button.canRender(), "it draws only its text sub-renderable, not itself");
        assertNull(button.renderData(), "renderData is null for a transparent button");
        assertEquals(3, button.getZIndex(), "z-depth comes from the constructor");
    }
}
