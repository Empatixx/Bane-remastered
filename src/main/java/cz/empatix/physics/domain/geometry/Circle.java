package cz.empatix.physics.domain.geometry;

import org.joml.Vector2f;

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

    @Override
    public boolean isColliding(Vector2f translation, Geometry other, Vector2f otherTranslation) {
        throw new UnsupportedOperationException("Not implemented yet");
    }
}
