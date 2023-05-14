package cz.empatix.tiles.service;

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
}
