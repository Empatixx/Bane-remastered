package cz.empatix.newphysics.attributes;

public class RotatedRect {
    private final Rect rect;
    private double angle;
    public RotatedRect(Rect rect, double angle) {
        this.rect = rect;
        this.angle = angle;
    }

}
