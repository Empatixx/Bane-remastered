package cz.empatix.physics.domain.geometry;

public record Rectangle(
        int width,
        int height
) implements Geometry{
    public static Rectangle from(final int width, final int height){
        return new Rectangle(width, height);
    }
    @Override
    public int getArea() {
        return width * height;
    }

    @Override
    public GeometryType getType() {
        return GeometryType.RECT;
    }
}
