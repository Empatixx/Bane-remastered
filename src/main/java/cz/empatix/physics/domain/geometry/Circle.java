package cz.empatix.physics.domain.geometry;

public record Circle(
       int radius
) implements Geometry{
    @Override
    public int getArea() {
        return (int) (Math.PI * radius * radius);
    }

    @Override
    public GeometryType getType() {
        return GeometryType.CIRCLE;
    }
}
