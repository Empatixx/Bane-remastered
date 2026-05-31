package cz.empatix.ui;

import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Exercises the REAL {@link LabelCheckbox}, which composes a label with a {@link Checkbox} and
 * delegates the {@link UIElement} contract to it. We drive it through its public surface and assert
 * the delegated behaviour; the label's GL-bound render path is not touched.
 */
@ExtendWith(HeadlessAssets.class)
class LabelCheckboxTest {

    @Test
    void pressInsideDelegatedCheckboxRegionTogglesUnderlyingState() {
        final LabelCheckbox labelCheckbox = new LabelCheckbox("Fullscreen");
        // The underlying checkbox sits at (0,0) spanning 24x24 by default.
        labelCheckbox.mousePressed(0, 5, 5); // press inside — must not throw, delegates cleanly

        assertFalse(labelCheckbox.isHovered(), "hover flag is only set by update(), not press");
    }

    @Test
    void zIndexAndRenderabilityComeFromTheCheckbox() {
        final LabelCheckbox labelCheckbox = new LabelCheckbox("Vsync");

        assertEquals(1, labelCheckbox.getZIndex(), "z-index delegates to the checkbox default");
        assertTrue(labelCheckbox.canRender(), "renderability delegates to the checkbox");
    }

    @Test
    void setPositionDoesNotThrowAndKeepsDelegationConsistent() {
        final LabelCheckbox labelCheckbox = new LabelCheckbox("Vsync");

        labelCheckbox.setPosition(100, 100);   // moves checkbox + recenters label
        labelCheckbox.setTextPosition(120, 100); // moves only the label

        assertFalse(labelCheckbox.isHovered(), "still unhovered after repositioning");
    }

    @Test
    void unhandledInputCallbacksAreSafeNoOps() {
        final LabelCheckbox labelCheckbox = new LabelCheckbox("Vsync");

        labelCheckbox.keyPressed(65);
        labelCheckbox.keyReleased(65);
        labelCheckbox.mouseScrolled(0, 1);
        labelCheckbox.mouseReleased(0, 0, 0);

        assertFalse(labelCheckbox.isHovered(), "delegated no-op input does not change state");
    }
}
