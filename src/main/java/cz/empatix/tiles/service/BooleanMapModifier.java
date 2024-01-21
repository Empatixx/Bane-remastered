package cz.empatix.tiles.service;

public interface BooleanMapModifier {
    boolean[][] modify(final boolean[][] original);
    float chance();
}
