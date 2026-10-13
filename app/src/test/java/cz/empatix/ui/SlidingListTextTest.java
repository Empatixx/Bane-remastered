package cz.empatix.ui;

import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.List;
import java.util.concurrent.atomic.AtomicReference;
import java.util.function.Function;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

/**
 * Exercises the REAL {@link SlidingListText}: it wraps a {@link SlidingList} and an asset-backed
 * {@code CommonText}. The text's GL render path is avoided; we assert the navigation drives the
 * backing list and fires the onChange callback. The centring math uses the GL-free fake font (zero
 * metrics) and does not throw.
 */
@ExtendWith(HeadlessAssets.class)
class SlidingListTextTest {

    private static final Function<String, String> IDENTITY = s -> s;

    @Test
    void nextAdvancesBackingListAndFiresOnChange() {
        final SlidingList<String> list = new SlidingList<>(List.of("a", "b", "c"));
        final SlidingListText<String> view = new SlidingListText<>(list, IDENTITY);
        view.center(0, 100, 50); // sets min/max bounds used by next()/previous()
        final AtomicReference<String> changed = new AtomicReference<>();
        view.onChange(changed::set);

        view.next();

        assertEquals("b", list.getSelected(), "next() advances the underlying list");
        assertEquals("b", changed.get(), "onChange receives the newly selected value");
    }

    @Test
    void previousMovesBackwardAndFiresOnChange() {
        final SlidingList<String> list = new SlidingList<>(List.of("a", "b", "c"));
        final SlidingListText<String> view = new SlidingListText<>(list, IDENTITY);
        view.center(0, 100, 50);
        final AtomicReference<String> changed = new AtomicReference<>();
        view.onChange(changed::set);

        view.previous(); // wraps to the last element

        assertEquals("c", list.getSelected(), "previous() wraps in the underlying list");
        assertEquals("c", changed.get(), "onChange receives the wrapped value");
    }

    @Test
    void slideToValueSeeksAndFiresOnChange() {
        final SlidingList<String> list = new SlidingList<>(List.of("a", "b", "c"));
        final SlidingListText<String> view = new SlidingListText<>(list, IDENTITY);
        view.center(0, 100, 50);
        final AtomicReference<String> changed = new AtomicReference<>();
        view.onChange(changed::set);

        view.slideToValue("c");

        assertEquals("c", list.getSelected(), "slideToValue seeks the underlying list");
        assertEquals("c", changed.get(), "onChange receives the sought value");
    }

    @Test
    void navigationWithNoListenerIsSafe() {
        final SlidingList<String> list = new SlidingList<>(List.of("a", "b"));
        final SlidingListText<String> view = new SlidingListText<>(list, IDENTITY);
        view.center(0, 100, 50);

        view.next();    // default empty consumer — must not throw
        view.update(0); // refreshes text from current selection — must not throw

        assertEquals("b", list.getSelected(), "navigation still advances without a listener");
    }

    @Test
    void renderContractMethodsThrowByDesign() {
        final SlidingList<String> list = new SlidingList<>(List.of("a"));
        final SlidingListText<String> view = new SlidingListText<>(list, IDENTITY);

        assertThrows(IllegalStateException.class, view::renderData, "renderData is not implemented");
        assertThrows(IllegalStateException.class, view::getZIndex, "getZIndex is not implemented");
        assertThrows(IllegalStateException.class, view::canRender, "canRender is not implemented");
    }
}
