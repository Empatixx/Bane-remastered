package cz.empatix.tiles.service;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

class PillarsBooleanModifierTest {

    private static final int PILLAR_SIZE = 2;
    private static final int OFFSET = 2;

    private final PillarsBooleanModifier modifier = new PillarsBooleanModifier();

    @Test
    void chanceIsTwentyPercent() {
        assertEquals(0.2f, modifier.chance());
    }

    @Test
    void modifyReturnsSameArrayInstance() {
        final boolean[][] map = BooleanMapUtil.createEmptyMap(12, 12);
        final boolean[][] result = modifier.modify(map);
        assertSame(map, result, "PillarsBooleanModifier mutates the grid in place");
    }

    @Test
    void modifyPlacesFourPillarBlocksInTheCorners() {
        final boolean[][] map = new boolean[12][12];
        modifier.modify(map);

        final int rows = 12;
        final int cols = 12;

        for (int i = 0; i < PILLAR_SIZE; i++) {
            for (int j = 0; j < PILLAR_SIZE; j++) {
                // top-left pillar
                assertTrue(map[i + OFFSET][j + OFFSET]);
                // top-right pillar
                assertTrue(map[i + OFFSET][cols - j - OFFSET - 1]);
                // bottom-left pillar
                assertTrue(map[rows - i - OFFSET - 1][j + OFFSET]);
                // bottom-right pillar
                assertTrue(map[rows - i - OFFSET - 1][cols - j - OFFSET - 1]);
            }
        }
    }

    @Test
    void modifyLeavesCenterCellsUntouched() {
        final boolean[][] map = new boolean[12][12];
        modifier.modify(map);

        // A cell well away from all four pillar blocks stays false.
        assertEquals(false, map[6][6]);
    }
}
