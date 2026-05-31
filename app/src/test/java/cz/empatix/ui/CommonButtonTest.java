package cz.empatix.ui;

import cz.empatix.testsupport.HeadlessAssets;
import cz.empatix.testsupport.MockedAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.concurrent.atomic.AtomicInteger;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Exercises the REAL {@link CommonButton} headlessly: hover hit-testing, the click {@link Runnable}
 * gate, and the text-position setter. Asset accessors are stubbed to GL-free fakes via
 * {@link MockedAssets}; rendering-output paths (which need a real {@code Font}) are not touched.
 */
@ExtendWith(HeadlessAssets.class)
class CommonButtonTest {

    /** The button is centred on its position: a 84x42 button at (100,100) spans (58,79)..(142,121). */
    @Test
    void updateMarksHoveredWhenMouseIsInsideBounds() {
        final CommonButton button = new CommonButton("Play", new Vector2f(100, 100));

        button.update(0.016, 100, 100);

        assertTrue(button.isHovered(), "mouse over the centre of the button hovers it");
    }

    @Test
    void updateClearsHoverWhenMouseLeavesBounds() {
        final CommonButton button = new CommonButton("Play", new Vector2f(100, 100));

        button.update(0.016, 100, 100);
        button.update(0.016, 1000, 1000);

        assertFalse(button.isHovered(), "mouse far outside the button clears hover");
    }

    @Test
    void mousePressedInsideRunsTheClickHandler() {
        final CommonButton button = new CommonButton("Play", new Vector2f(100, 100));
        final AtomicInteger clicks = new AtomicInteger();
        button.onClick(clicks::incrementAndGet);

        button.mousePressed(0, 100, 100);

        assertEquals(1, clicks.get(), "a press inside the bounds fires onClick exactly once");
    }

    @Test
    void mousePressedOutsideDoesNotRunTheClickHandler() {
        final CommonButton button = new CommonButton("Play", new Vector2f(100, 100));
        final AtomicInteger clicks = new AtomicInteger();
        button.onClick(clicks::incrementAndGet);

        button.mousePressed(0, 1000, 1000);

        assertEquals(0, clicks.get(), "a press outside the bounds does not fire onClick");
    }

    @Test
    void mousePressedWithNoHandlerIsANoOp() {
        final CommonButton button = new CommonButton("Play", new Vector2f(100, 100));

        button.mousePressed(0, 100, 100); // no onClick registered — must not throw

        assertFalse(button.isHovered(), "press alone does not set hover on a plain button");
    }

    @Test
    void getZIndexReflectsDefaultDepth() {
        final CommonButton button = new CommonButton("Play", new Vector2f(100, 100));

        assertEquals(1, button.getZIndex(), "default z-depth is 1");
        assertTrue(button.canRender(), "common button is renderable");
    }

    @Test
    void setTextPositionUpdatesTheReportedTextPosition() {
        final CommonButton button = new CommonButton("Play", new Vector2f(100, 100));

        button.setTextPosition(33, 44);

        assertEquals(33f, button.getTextPosition().x(), 0f, "text x is set");
        assertEquals(44f, button.getTextPosition().y(), 0f, "text y is set");
    }
}
