package cz.empatix.enemies.targetting;

public record PathCell(
        int x,
        int y,
        int g,
        int h,
        int f,
        PathCell parent
) {
    public PathCell(final int x, final int y, final int g, final int h, final PathCell parent) {
        this(x, y, g, h, g + h, parent);
    }
}
