package cz.empatix.enemies.targetting;

public record PathCell(
        int x,
        int y,
        int g,
        int h,
        int f,
        PathCell parent
) {
}
