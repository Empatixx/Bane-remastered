package cz.empatix.tiles.service;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

class EdgeClampBooleanModifierTest {

    private final EdgeClampBooleanModifier modifier = new EdgeClampBooleanModifier();

    @Test
    void chanceIsAlwaysOne() {
        assertEquals(1f, modifier.chance());
    }

    @Test
    void modifyGrowsGridByTwoInEachDimension() {
        final boolean[][] original = new boolean[3][4];
        final boolean[][] result = modifier.modify(original);

        assertEquals(5, result.length);
        assertEquals(6, result[0].length);
    }

    @Test
    void modifyClampsEdgesAndPreservesInterior() {
        final boolean[][] original = {
                {true, false},
                {false, true}
        };

        final boolean[][] result = modifier.modify(original);

        // Interior mirrors the original.
        assertTrue(result[1][1]);
        assertFalse(result[1][2]);
        assertFalse(result[2][1]);
        assertTrue(result[2][2]);

        // Clamped corners equal nearest original corner.
        assertEquals(original[0][0], result[0][0]);
        assertEquals(original[1][1], result[3][3]);
    }

    @Test
    void modifyMatchesUtilEdgeClamp() {
        final boolean[][] original = {
                {true, false, true},
                {false, true, false}
        };

        final boolean[][] viaModifier = modifier.modify(original);
        final boolean[][] viaUtil = BooleanMapUtil.edgeClamp(original);

        assertEquals(viaUtil.length, viaModifier.length);
        for (int i = 0; i < viaUtil.length; i++) {
            for (int j = 0; j < viaUtil[0].length; j++) {
                assertEquals(viaUtil[i][j], viaModifier[i][j], "cell (" + i + "," + j + ")");
            }
        }
    }
}
