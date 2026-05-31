package cz.empatix.tiles.service;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

/** Pure-logic coverage for {@link WallsBooleanModifier}, which rings the grid border with walls. */
class WallsBooleanModifierTest {

    private final WallsBooleanModifier modifier = new WallsBooleanModifier();

    @Test
    void chanceIsAlwaysOne() {
        assertEquals(1f, modifier.chance());
    }

    @Test
    void modifyMutatesAndReturnsTheSameArrayInstance() {
        final boolean[][] map = new boolean[5][6];

        final boolean[][] result = modifier.modify(map);

        assertSame(map, result);
    }

    @Test
    void modifySetsExactlyTheBorderCells() {
        final int rows = 5;
        final int cols = 6;
        final boolean[][] map = new boolean[rows][cols];

        modifier.modify(map);

        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                final boolean isBorder = i == 0 || j == 0 || i == rows - 1 || j == cols - 1;
                assertEquals(isBorder, map[i][j], "cell (" + i + "," + j + ")");
            }
        }
        assertTrue(map[0][0]);
        assertFalse(map[2][2]);
    }
}
