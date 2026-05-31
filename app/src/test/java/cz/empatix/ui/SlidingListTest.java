package cz.empatix.ui;

import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Pure-logic tests for {@link SlidingList}: cyclic next/previous navigation and value-seeking with a
 * graceful fallback. No GL or audio involved, so no stubbing is needed.
 */
class SlidingListTest {

    @Test
    void nextWrapsAroundToTheFirstElement() {
        final SlidingList<String> list = new SlidingList<>(List.of("low", "medium", "high"));

        list.next(); // medium
        list.next(); // high
        list.next(); // wraps to low

        assertEquals("low", list.getSelected(), "next() is cyclic over the list");
    }

    @Test
    void previousWrapsAroundToTheLastElement() {
        final SlidingList<String> list = new SlidingList<>(List.of("low", "medium", "high"));

        list.previous(); // wraps to high

        assertEquals("high", list.getSelected(), "previous() from index 0 wraps to the last element");
    }

    @Test
    void getSelectedReturnsFirstElementInitially() {
        final SlidingList<Integer> list = new SlidingList<>(List.of(10, 20, 30));

        assertEquals(10, list.getSelected(), "the first element is selected by default");
    }

    @Test
    void slideToValueSelectsTheMatchingElement() {
        final SlidingList<String> list = new SlidingList<>(List.of("low", "medium", "high"));

        list.slideToValue("high");

        assertEquals("high", list.getSelected(), "slideToValue jumps to the matching entry");
    }

    @Test
    void slideToValueFallsBackToFirstWhenValueIsAbsent() {
        final SlidingList<String> list = new SlidingList<>(List.of("low", "medium", "high"));
        list.next(); // move off index 0 first

        list.slideToValue("missing");

        assertEquals("low", list.getSelected(), "an unknown value falls back to the first element");
    }
}
