package cz.empatix.ui;

import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * Exercises the REAL {@link UISelector}: single-selection bookkeeping, value mapping, and the
 * press-to-select state machine. Collaborator {@link SelectableUI} elements are mocked so hover
 * outcomes are deterministic; one test uses real {@link StickyHoverButton}s end-to-end.
 */
@ExtendWith(HeadlessAssets.class)
class UISelectorTest {

    @Test
    void firstAddedElementBecomesSelectedAndIsSelectCalled() {
        final UISelector<String> selector = new UISelector<>();
        final SelectableUI first = mock(SelectableUI.class);

        selector.addElement(first, "easy");

        verify(first).select();
        assertEquals("easy", selector.getSelected(), "the first element is auto-selected");
    }

    @Test
    void laterAddedElementsAreNotAutoSelected() {
        final UISelector<String> selector = new UISelector<>();
        final SelectableUI first = mock(SelectableUI.class);
        final SelectableUI second = mock(SelectableUI.class);

        selector.addElement(first, "easy");
        selector.addElement(second, "hard");

        verify(second, never()).select();
        assertEquals("easy", selector.getSelected(), "selection stays on the first element");
    }

    @Test
    void pressingAHoveredElementMovesSelectionToIt() {
        final UISelector<String> selector = new UISelector<>();
        final SelectableUI first = mock(SelectableUI.class);
        final SelectableUI second = mock(SelectableUI.class);
        when(second.isHovered()).thenReturn(true); // the press lands on the second element
        selector.addElement(first, "easy");
        selector.addElement(second, "hard");

        selector.mousePressed(0, 5, 5);

        assertEquals("hard", selector.getSelected(), "selection follows the hovered element");
        verify(first).unselect(); // the previously selected element gets unselected
    }

    @Test
    void pressingWithNoHoverKeepsCurrentSelection() {
        final UISelector<String> selector = new UISelector<>();
        final SelectableUI first = mock(SelectableUI.class);
        final SelectableUI second = mock(SelectableUI.class);
        when(second.isHovered()).thenReturn(false);
        selector.addElement(first, "easy");
        selector.addElement(second, "hard");

        selector.mousePressed(0, 5, 5);

        assertEquals("easy", selector.getSelected(), "no hovered element means no selection change");
    }

    @Test
    void inputEventsAreFannedOutToEveryElement() {
        final UISelector<String> selector = new UISelector<>();
        final SelectableUI first = mock(SelectableUI.class);
        final SelectableUI second = mock(SelectableUI.class);
        selector.addElement(first, "easy");
        selector.addElement(second, "hard");

        selector.update(0.016, 1, 2);
        selector.keyPressed(65);
        selector.keyReleased(65);
        selector.mouseScrolled(0, 1);

        verify(first).update(0.016, 1, 2);
        verify(second).update(0.016, 1, 2);
        verify(first).keyPressed(65);
        verify(second).keyReleased(65);
        verify(first).mouseScrolled(0, 1);
    }

    @Test
    void containerNeverReportsItselfHoveredAndForbidsDirectRendering() {
        final UISelector<String> selector = new UISelector<>();

        assertFalse(selector.isHovered(), "the selector container is never hovered");
        assertThrows(IllegalStateException.class, selector::renderData, "direct renderData is forbidden");
        assertThrows(IllegalStateException.class, selector::getZIndex, "direct getZIndex is forbidden");
        assertThrows(IllegalStateException.class, selector::canRender, "direct canRender is forbidden");
    }

    @Test
    void worksEndToEndWithRealStickyHoverButtons() {
        final UISelector<String> selector = new UISelector<>();
        final StickyHoverButton easy = new StickyHoverButton("Easy", new Vector2f(0, 0));
        final StickyHoverButton hard = new StickyHoverButton("Hard", new Vector2f(200, 0));
        selector.addElement(easy, "easy");
        selector.addElement(hard, "hard");

        // The first button is auto-selected (and thus stuck-hovered).
        assertTrue(easy.isSelected(), "first real element starts selected");
        assertSame("easy", selector.getSelected(), "value maps to the first element");

        // Press inside the second button (200..284 x 0..42); it sticks and wins the selection.
        selector.mousePressed(0, 210, 10);

        assertEquals("hard", selector.getSelected(), "pressing the second button selects it");
        assertFalse(easy.isSelected(), "the first button is unselected once the second wins");
    }
}
