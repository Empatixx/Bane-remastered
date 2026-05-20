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

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        PathCell pathCell = (PathCell) obj;
        return x == pathCell.x && y == pathCell.y;
    }

    @Override
    public int hashCode() {
        return 31 * x + y;
    }
}
