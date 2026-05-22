package cz.empatix.tiles.service;

import cz.empatix.map.domain.RoomDirection;
import org.junit.jupiter.api.Test;

import java.util.EnumSet;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

class BooleanMapUtilTest {

    @Test
    void createEmptyMapMarksOnlyTheBorder() {
        final boolean[][] map = BooleanMapUtil.createEmptyMap(4, 5);

        assertEquals(4, map.length);
        assertEquals(5, map[0].length);

        for (int i = 0; i < 4; i++) {
            for (int j = 0; j < 5; j++) {
                final boolean isBorder = i == 0 || j == 0 || i == 3 || j == 4;
                assertEquals(isBorder, map[i][j], "cell (" + i + "," + j + ")");
            }
        }
    }

    @Test
    void edgeClampGrowsMapByOnePerSideAndDuplicatesEdges() {
        final boolean[][] original = {
                {true, false},
                {false, true}
        };

        final boolean[][] expanded = BooleanMapUtil.edgeClamp(original);

        assertEquals(4, expanded.length);
        assertEquals(4, expanded[0].length);

        // Inner 2x2 mirrors the original (shifted by one).
        assertTrue(expanded[1][1]);
        assertFalse(expanded[1][2]);
        assertFalse(expanded[2][1]);
        assertTrue(expanded[2][2]);

        // Corners clamp to nearest original corner.
        assertEquals(original[0][0], expanded[0][0]);
        assertEquals(original[0][1], expanded[0][3]);
        assertEquals(original[1][0], expanded[3][0]);
        assertEquals(original[1][1], expanded[3][3]);
    }

    @Test
    void edgeClampClampsTopAndLeftBorders() {
        final boolean[][] original = {
                {true, true, true},
                {false, false, false},
                {true, true, true}
        };

        final boolean[][] expanded = BooleanMapUtil.edgeClamp(original);

        // Top border row duplicates first original row.
        for (int j = 0; j < expanded[0].length; j++) {
            final int oj = Math.min(Math.max(j - 1, 0), 2);
            assertEquals(original[0][oj], expanded[0][j]);
        }
        // Left border column duplicates first original column.
        for (int i = 0; i < expanded.length; i++) {
            final int oi = Math.min(Math.max(i - 1, 0), 2);
            assertEquals(original[oi][0], expanded[i][0]);
        }
    }

    @Test
    void createMapWithNoModifiersReturnsEmptyMap() {
        final boolean[][] map = BooleanMapUtil.createMap(6, 6, List.of());

        // Same shape and border as createEmptyMap.
        final boolean[][] expected = BooleanMapUtil.createEmptyMap(6, 6);
        for (int i = 0; i < 6; i++) {
            for (int j = 0; j < 6; j++) {
                assertEquals(expected[i][j], map[i][j]);
            }
        }
    }

    @Test
    void createMapAppliesAlwaysOnModifier() {
        // EdgeClampBooleanModifier has chance()==1f so it always applies, growing the map.
        final boolean[][] map = BooleanMapUtil.createMap(6, 6,
                List.of(new EdgeClampBooleanModifier()));

        assertEquals(8, map.length);
        assertEquals(8, map[0].length);
    }

    @Test
    void setPathsBottomOpensTwoCellsOnBottomRow() {
        final boolean[][] map = BooleanMapUtil.createEmptyMap(8, 8);
        // cols=8, MIN_COLS=4 -> centerCols = (8-4)/2 = 2; opens columns 3 and 4 on last row.
        BooleanMapUtil.setPaths(map, EnumSet.of(RoomDirection.BOTTOM));

        assertFalse(map[7][3]);
        assertFalse(map[7][4]);
        // Neighboring border cells remain walls.
        assertTrue(map[7][2]);
        assertTrue(map[7][5]);
    }

    @Test
    void setPathsTopOpensTwoCellsOnTopRow() {
        final boolean[][] map = BooleanMapUtil.createEmptyMap(8, 8);
        BooleanMapUtil.setPaths(map, EnumSet.of(RoomDirection.TOP));

        assertFalse(map[0][3]);
        assertFalse(map[0][4]);
    }

    @Test
    void setPathsLeftAndRightOpenSideColumns() {
        final boolean[][] map = BooleanMapUtil.createEmptyMap(8, 8);
        // rows=8, MIN_ROWS=4 -> centerRows = (8-4)/2 = 2; opens rows 3 and 4.
        BooleanMapUtil.setPaths(map, EnumSet.of(RoomDirection.LEFT, RoomDirection.RIGHT));

        assertFalse(map[3][0]);
        assertFalse(map[4][0]);
        assertFalse(map[3][7]);
        assertFalse(map[4][7]);
    }

    @Test
    void setPathsWithEmptyDirectionsLeavesMapUnchanged() {
        final boolean[][] map = BooleanMapUtil.createEmptyMap(8, 8);
        final boolean[][] expected = BooleanMapUtil.createEmptyMap(8, 8);

        BooleanMapUtil.setPaths(map, EnumSet.noneOf(RoomDirection.class));

        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                assertEquals(expected[i][j], map[i][j]);
            }
        }
    }
}
