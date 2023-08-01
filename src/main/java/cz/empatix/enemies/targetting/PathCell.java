package cz.empatix.enemies.targetting;

public record PathCell(
        int x,
        int y,
        float g,
        float h,
        float f,
        PathCell parent
) {
    public PathCell(final int x, final int y, final float g, final float h, final PathCell parent) {
        this(x, y, g, h, g + h, parent);
    }
}
