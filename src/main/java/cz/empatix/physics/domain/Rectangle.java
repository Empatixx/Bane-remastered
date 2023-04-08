package cz.empatix.physics.domain;

public record Rectangle(
        int width,
        int height
) implements Geometry{
    public static Rectangle from(final int width, final int height){
        return new Rectangle(width, height);
    }
    @Override
    public boolean isIntersecting(final Geometry other) {
        if(other instanceof Rectangle){
            return this.isIntersecting((Rectangle) other);
        } else {
            throw new IllegalArgumentException("Unsupported geometry type");
        }
    }

    @Override
    public int getMass() {
        return width * height;
    }
    private boolean isIntersecting(final Rectangle other){
        return this.width <= other.width + other.height &&
                this.width + this.height >= other.width &&
                this.height <= other.height + other.height &&
                this.height + this.height >= other.height;
    }
}
