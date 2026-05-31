package cz.empatix.ui;

import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicReference;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Exercises the REAL {@link Checkbox}: toggle-on-press hit-testing, the {@link java.util.function.Consumer}
 * callback, and position/dimension getters. The checkbox anchors at its position; with a 16x16 sprite
 * at scale 1.5 the box is 24x24, so at (0,0) it spans (0,0)..(24,24).
 */
@ExtendWith(HeadlessAssets.class)
class CheckboxTest {

    @Test
    void pressInsideTogglesCheckedAndNotifiesConsumer() {
        final Checkbox checkbox = new Checkbox();
        final AtomicReference<Boolean> last = new AtomicReference<>();
        final AtomicInteger calls = new AtomicInteger();
        checkbox.onCheck(value -> {
            last.set(value);
            calls.incrementAndGet();
        });

        checkbox.mousePressed(0, 5, 5);

        assertEquals(1, calls.get(), "consumer fires once per toggle");
        assertTrue(last.get(), "first toggle turns the checkbox on");
    }

    @Test
    void pressInsideTwiceTogglesBackOff() {
        final Checkbox checkbox = new Checkbox();
        final AtomicReference<Boolean> last = new AtomicReference<>();
        checkbox.onCheck(last::set);

        checkbox.mousePressed(0, 5, 5);
        checkbox.mousePressed(0, 5, 5);

        assertFalse(last.get(), "second toggle turns it back off");
    }

    @Test
    void pressOutsideDoesNotToggle() {
        final Checkbox checkbox = new Checkbox();
        final AtomicInteger calls = new AtomicInteger();
        checkbox.onCheck(value -> calls.incrementAndGet());

        checkbox.mousePressed(0, 1000, 1000);

        assertEquals(0, calls.get(), "a press outside the box does not toggle");
    }

    @Test
    void pressWithoutConsumerDoesNotThrow() {
        final Checkbox checkbox = new Checkbox();

        checkbox.mousePressed(0, 5, 5); // no consumer registered — must not throw

        assertFalse(checkbox.isHovered(), "isHovered tracks the (here unset) hover field");
    }

    @Test
    void setPositionMovesHitTestRegion() {
        final Checkbox checkbox = new Checkbox();
        final AtomicInteger calls = new AtomicInteger();
        checkbox.onCheck(value -> calls.incrementAndGet());

        checkbox.setPosition(100, 100);
        checkbox.mousePressed(0, 5, 5);     // old region — should miss
        checkbox.mousePressed(0, 110, 110); // new region — should hit

        assertEquals(1, calls.get(), "only the press inside the moved region toggles");
        assertEquals(100f, checkbox.getPosition().x(), 0f, "position x updated");
        assertEquals(100f, checkbox.getPosition().y(), 0f, "position y updated");
    }

    @Test
    void dimensionsAreSpriteSizeTimesScale() {
        final Checkbox checkbox = new Checkbox();

        assertEquals(24, checkbox.getDimensions().x(), "16 * 1.5 = 24 width");
        assertEquals(24, checkbox.getDimensions().y(), "16 * 1.5 = 24 height");
        assertEquals(24, checkbox.getHeight(), "getHeight mirrors dimension y");
        assertEquals(1, checkbox.getZIndex(), "default z-index is 1");
    }

    @Test
    void checkSetsStateProgrammatically() {
        final Checkbox checkbox = new Checkbox();
        final AtomicReference<Boolean> last = new AtomicReference<>();
        checkbox.onCheck(last::set);

        checkbox.check(true);          // programmatic set must NOT fire the consumer
        checkbox.mousePressed(0, 5, 5); // now a press toggles from true -> false

        assertFalse(last.get(), "after check(true), pressing toggles to false");
    }
}
