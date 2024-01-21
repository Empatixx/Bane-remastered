package cz.empatix.tiles.service;

public class PillarsBooleanModifier implements BooleanMapModifier {
    private final static int PILLAR_SIZE = 2;
    private final static int MAX_PILLARS = 4;
    private final static int OFFSET = 2;
    @Override
    public boolean[][] modify(boolean[][] original) {
        final int rows = original.length;
        final int cols = original[0].length;

        for(int i = 0; i < PILLAR_SIZE;i++){
            for(int j = 0; j < PILLAR_SIZE;j++){
                original[i + OFFSET][j + OFFSET] = true;
                original[i + OFFSET][cols - j - OFFSET - 1] = true;
                original[rows - i - OFFSET - 1][j + OFFSET] = true;
                original[rows - i - OFFSET - 1][cols - j - OFFSET - 1] = true;
            }
        }

        return original;
    }

    @Override
    public float chance() {
        return 0.2f;
    }
}
