package cz.empatix.ui;

import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.concurrent.atomic.AtomicInteger;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Exercises the REAL {@link StickyHoverButton}: its press-to-stick selection model, the
 * {@link SelectableUI} select/unselect transitions, and position/size getters. Unlike
 * {@link CommonButton} this button anchors at its position (no centre offset), so a 84x42 button at
 * (0,0) spans (0,0)..(84,42).
 */
@ExtendWith(HeadlessAssets.class)
class StickyHoverButtonTest {

    @Test
    void mousePressedInsideSticksHoverAndStaysHoveredAfterUpdate() {
        final StickyHoverButton button = new StickyHoverButton("Opt", new Vector2f(0, 0));

        button.mousePressed(0, 10, 10);

        assertTrue(button.isHovered(), "a press inside sticks the hover state");
        assertTrue(button.isSelected(), "sticky hover reports as selected");

        button.update(0.016, 1000, 1000); // moving the mouse away must not clear sticky hover
        assertTrue(button.isHovered(), "hover stays stuck after the mouse leaves");
    }

    @Test
    void mousePressedOutsideLeavesButtonUnhovered() {
        final StickyHoverButton button = new StickyHoverButton("Opt", new Vector2f(0, 0));

        button.mousePressed(0, 1000, 1000);

        assertFalse(button.isHovered(), "a press outside does not stick hover");
        assertFalse(button.isSelected(), "and the button is not selected");
    }

    @Test
    void mousePressedInsideRunsTheClickHandler() {
        final StickyHoverButton button = new StickyHoverButton("Opt", new Vector2f(0, 0));
        final AtomicInteger clicks = new AtomicInteger();
        button.onClick(clicks::incrementAndGet);

        button.mousePressed(0, 10, 10);

        assertEquals(1, clicks.get(), "a press inside fires onClick once");
    }

    @Test
    void selectAndUnselectToggleHoverState() {
        final StickyHoverButton button = new StickyHoverButton("Opt", new Vector2f(0, 0));

        button.select();
        assertTrue(button.isSelected(), "select() marks the button selected");

        button.unselect();
        assertFalse(button.isSelected(), "unselect() clears the selection");
    }

    @Test
    void positionAndSizeGettersReflectConstructorArguments() {
        final StickyHoverButton button = new StickyHoverButton("Opt", new Vector2f(5, 7));

        assertEquals(84, button.getWidth(), "default width survives scale 1");
        assertEquals(42, button.getHeight(), "default height survives scale 1");
        assertEquals(5f, button.getPosition().x(), 0f, "anchored x equals position");
        assertEquals(7f, button.getPosition().y(), 0f, "anchored y equals position");
    }

    @Test
    void setPositionMovesTheButton() {
        final StickyHoverButton button = new StickyHoverButton("Opt", new Vector2f(0, 0));

        button.setPosition(50, 60);

        assertEquals(50f, button.getPosition().x(), 0f, "x moved");
        assertEquals(60f, button.getPosition().y(), 0f, "y moved");
        assertTrue(button.mouseIntersectsUI(60, 70), "hit-test follows the new position");
        assertFalse(button.mouseIntersectsUI(10, 10), "old location no longer intersects");
    }
}
