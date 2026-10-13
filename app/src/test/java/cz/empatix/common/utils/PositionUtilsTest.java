package cz.empatix.common.utils;

import cz.empatix.core.Viewport;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class PositionUtilsTest {

    @Test
    void centerOfViewportIsHalfOfDimensions() {
        Viewport.WIDTH = 800;
        Viewport.HEIGHT = 600;
        assertEquals(400, PositionUtils.getCenterX());
        assertEquals(300, PositionUtils.getCenterY());
    }

    @Test
    void centerInRangeIsMidpoint() {
        assertEquals(15, PositionUtils.getCenterXInRange(10, 20));
        assertEquals(50, PositionUtils.getCenterYInRange(40, 60));
    }

    @Test
    void centerInRangeWithSizeShiftsByHalfSize() {
        // midpoint 15, minus half of width 4 -> 13
        assertEquals(13, PositionUtils.getCenterXInRange(10, 20, 4));
        // midpoint 50, minus half of height 10 -> 45
        assertEquals(45, PositionUtils.getCenterYInRange(40, 60, 10));
    }

    @Test
    void centerInRangeUsesIntegerDivision() {
        // (0 + 5) / 2 == 2 (truncated)
        assertEquals(2, PositionUtils.getCenterXInRange(0, 5));
        assertEquals(2, PositionUtils.getCenterYInRange(0, 5));
    }
}
