package cz.empatix.tiles.service;

public class WallsBooleanModifier implements BooleanMapModifier{
    @Override
    public boolean[][] modify(boolean[][] original) {
        final int rows = original.length;
        final int cols = original[0].length;
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                if (i == 0 || j == 0 || i == rows - 1 || j == cols - 1) {
                    original[i][j] = true;
                }
            }
        }
        return original;
    }

    @Override
    public float chance() {
        return 1f;
    }
}
