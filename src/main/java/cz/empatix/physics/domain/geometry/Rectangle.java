package cz.empatix.physics.domain.geometry;

import org.joml.Vector2f;

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

    @Override
    public boolean isColliding(Vector2f translation, Geometry other, Vector2f otherTranslation) {
        if (other.getType() == GeometryType.RECT) {
            return isCollidingWithRectangle(translation, (Rectangle) other, otherTranslation);
        }
        throw new UnsupportedOperationException("Not implemented yet");
    }

    private boolean isCollidingWithRectangle(final Vector2f translation, final Rectangle other, final Vector2f otherTranslation) {
        final float x1 = translation.x();
        final float y1 = translation.y();
        final float x2 = otherTranslation.x();
        final float y2 = otherTranslation.y();

        final float w1 = width;
        final float h1 = height;
        final float w2 = other.width;
        final float h2 = other.height;

        return x1 < x2 + w2 &&
                x1 + w1 > x2 &&
                y1 < y2 + h2 &&
                y1 + h1 > y2;
    }
}
