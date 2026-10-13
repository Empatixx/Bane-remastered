package cz.empatix.tiles.service;

public class CenterRectBooleanModifier implements BooleanMapModifier{
    private final static int RECT_SIZE = 3;
    @Override
    public boolean[][] modify(boolean[][] original) {
        final int rows = original.length;
        final int cols = original[0].length;

        final int centerX = cols / 2;
        final int centerY = rows / 2;
        for (int i = centerY - RECT_SIZE / 2; i < centerY + RECT_SIZE / 2; i++) {
            for (int j = centerX - RECT_SIZE / 2; j < centerX + RECT_SIZE / 2; j++) {
                original[i][j] = true;
            }
        }
        return original;
    }

    @Override
    public float chance() {
        return 0.2f;
    }
}
