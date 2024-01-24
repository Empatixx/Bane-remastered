package cz.empatix.newphysics.attributes;

import org.joml.Vector2f;

public record Rect(
        int width,
        int height,
        Vector2f position
) implements AABB{
    @Override
    public float xMin() {
        return position.x();
    }

    @Override
    public float yMin() {
        return position.y();
    }

    @Override
    public float xMax() {
        return position.x() + width;
    }

    @Override
    public float yMax() {
        return position.y() + height;
    }
    public static Rect of(final int width, final int height){
        return new Rect(width, height, new Vector2f());
    }

    @Override
    public void setAngle(double angle) {

    }

    @Override
    public void setPosition(float x, float y) {
        position.set(x,y);
    }

    @Override
    public Vector2f getPosition() {
        return position;
    }

    @Override
    public float getAngle() {
        return 0;
    }

    @Override
    public boolean supportsSAT() {
        return true;
    }

    @Override
    public boolean supportsAABB() {
        return true;
    }

    @Override
    public float getMass() {
        return width * height;
    }
}
