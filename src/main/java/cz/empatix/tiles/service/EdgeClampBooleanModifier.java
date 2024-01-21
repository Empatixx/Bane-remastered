package cz.empatix.tiles.service;

public class EdgeClampBooleanModifier implements BooleanMapModifier{
    @Override
    public boolean[][] modify(boolean[][] original) {
        final int originalHeight = original.length;
        final int originalWidth = original[0].length;

        final int newHeight = originalHeight + 2;
        final int newWidth = originalWidth + 2;

        final boolean[][] expanded = new boolean[newHeight][newWidth];

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

    @Override
    public float chance() {
        return 1f;
    }
}
