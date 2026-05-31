package cz.empatix.tiles.service;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

/** Pure-logic coverage for {@link CenterRectBooleanModifier}. */
class CenterRectBooleanModifierTest {

    private final CenterRectBooleanModifier modifier = new CenterRectBooleanModifier();

    @Test
    void chanceIsTwentyPercent() {
        assertEquals(0.2f, modifier.chance());
    }

    @Test
    void modifyMutatesAndReturnsTheSameArrayInstance() {
        final boolean[][] map = new boolean[9][9];

        final boolean[][] result = modifier.modify(map);

        assertSame(map, result);
    }

    @Test
    void modifyMarksTheCentralRectangle() {
        // RECT_SIZE == 3, so the loop covers centerY-1..centerY and centerX-1..centerX.
        final boolean[][] map = new boolean[9][9];

        modifier.modify(map);

        final int centerX = 9 / 2;
        final int centerY = 9 / 2;
        for (int i = centerY - 1; i < centerY + 1; i++) {
            for (int j = centerX - 1; j < centerX + 1; j++) {
                assertTrue(map[i][j], "centre cell (" + i + "," + j + ") should be set");
            }
        }
        // A far corner is untouched.
        assertFalse(map[0][0]);
    }
}
