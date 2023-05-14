package cz.empatix.tiles.service;

import cz.empatix.map.domain.RoomDirection;

import java.util.List;

public class BooleanMapUtil {
    public static boolean[][] edgeClamp(final boolean[][] original) {
        int originalHeight = original.length;
        int originalWidth = original[0].length;
        int newHeight = originalHeight + 2;
        int newWidth = originalWidth + 2;

        boolean[][] expanded = new boolean[newHeight][newWidth];

        for (int i = 0; i < newHeight; i++) {
            for (int j = 0; j < newWidth; j++) {
                // Edge clamping
                int originalI = Math.min(Math.max(i - 1, 0), originalHeight - 1);
                int originalJ = Math.min(Math.max(j - 1, 0), originalWidth - 1);

                expanded[i][j] = original[originalI][originalJ];
            }
        }

        return expanded;
    }

    public static boolean[][] createEmptyMap(final int rows, final int cols) {
        final boolean[][] map = new boolean[rows][cols];
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                if (i == 0 || j == 0 || i == rows - 1 || j == cols - 1) {
                    map[i][j] = true;
                }
            }
        }
        return map;
    }
    public static void setPaths(final boolean[][] map, List<RoomDirection> directions){
        final int rows = map.length;
        final int cols = map[0].length;

        for (final RoomDirection dir : directions) {
            switch (dir) {
                case BOTTOM -> {
                    final int centerCols = (cols - 4) / 2;
                    map[rows - 1][centerCols + 1] = false;
                    map[rows - 1][centerCols + 2] = false;
                }
                case TOP -> {
                    final int centerCols = (cols - 4) / 2;
                    map[0][centerCols + 1] = false;
                    map[0][centerCols + 2] = false;
                }
                case LEFT -> {
                    final int centerRows = (rows - 4) / 2;
                    map[centerRows + 1][0] = false;
                    map[centerRows + 2][0] = false;
                }
                case RIGHT -> {
                    final int centerRows = (rows - 4) / 2;
                    map[centerRows + 1][cols - 1] = false;
                    map[centerRows + 2][cols - 1] = false;
                }
            }
        }
    }
}
