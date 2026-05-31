package cz.empatix.ui;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import java.util.List;
import java.util.concurrent.atomic.AtomicReference;
import java.util.function.Function;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.mockito.Mockito.mockStatic;

/**
 * Exercises the REAL {@link SlidingListText}: it wraps a {@link SlidingList} and an asset-backed
 * {@code CommonText}. The text's GL render path is avoided; we assert the navigation drives the
 * backing list and fires the onChange callback. The centring math uses the GL-free fake font (zero
 * metrics) and does not throw.
 */
class SlidingListTextTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    private static final Function<String, String> IDENTITY = s -> s;

    @Test
    void nextAdvancesBackingListAndFiresOnChange() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final SlidingList<String> list = new SlidingList<>(List.of("a", "b", "c"));
            final SlidingListText<String> view = new SlidingListText<>(list, IDENTITY);
            view.center(0, 100, 50); // sets min/max bounds used by next()/previous()
            final AtomicReference<String> changed = new AtomicReference<>();
            view.onChange(changed::set);

            view.next();

            assertEquals("b", list.getSelected(), "next() advances the underlying list");
            assertEquals("b", changed.get(), "onChange receives the newly selected value");
        }
    }

    @Test
    void previousMovesBackwardAndFiresOnChange() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final SlidingList<String> list = new SlidingList<>(List.of("a", "b", "c"));
            final SlidingListText<String> view = new SlidingListText<>(list, IDENTITY);
            view.center(0, 100, 50);
            final AtomicReference<String> changed = new AtomicReference<>();
            view.onChange(changed::set);

            view.previous(); // wraps to the last element

            assertEquals("c", list.getSelected(), "previous() wraps in the underlying list");
            assertEquals("c", changed.get(), "onChange receives the wrapped value");
        }
    }

    @Test
    void slideToValueSeeksAndFiresOnChange() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final SlidingList<String> list = new SlidingList<>(List.of("a", "b", "c"));
            final SlidingListText<String> view = new SlidingListText<>(list, IDENTITY);
            view.center(0, 100, 50);
            final AtomicReference<String> changed = new AtomicReference<>();
            view.onChange(changed::set);

            view.slideToValue("c");

            assertEquals("c", list.getSelected(), "slideToValue seeks the underlying list");
            assertEquals("c", changed.get(), "onChange receives the sought value");
        }
    }

    @Test
    void navigationWithNoListenerIsSafe() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final SlidingList<String> list = new SlidingList<>(List.of("a", "b"));
            final SlidingListText<String> view = new SlidingListText<>(list, IDENTITY);
            view.center(0, 100, 50);

            view.next();    // default empty consumer — must not throw
            view.update(0); // refreshes text from current selection — must not throw

            assertEquals("b", list.getSelected(), "navigation still advances without a listener");
        }
    }

    @Test
    void renderContractMethodsThrowByDesign() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final SlidingList<String> list = new SlidingList<>(List.of("a"));
            final SlidingListText<String> view = new SlidingListText<>(list, IDENTITY);

            assertThrows(IllegalStateException.class, view::renderData, "renderData is not implemented");
            assertThrows(IllegalStateException.class, view::getZIndex, "getZIndex is not implemented");
            assertThrows(IllegalStateException.class, view::canRender, "canRender is not implemented");
        }
    }
}
