package cz.empatix.common.utils;
import cz.empatix.core.Viewport;


public class PositionUtils {
    public static int getCenterX() {
        return Viewport.WIDTH / 2;
    }

    public static int getCenterY() {
        return Viewport.HEIGHT / 2;
    }

    public static int getCenterXInRange(int xMin, int xMax) {
        return (xMin + xMax) / 2;
    }

    public static int getCenterYInRange(int yMin, int yMax) {
        return (yMin + yMax) / 2;
    }

    public static int getCenterXInRange(int xMin, int xMax, int width) {
        return (xMin + xMax) / 2 - width / 2;
    }
    public static int getCenterYInRange(int yMin, int yMax, int height) {
        return(yMin + yMax) / 2 - height / 2;
    }
}
