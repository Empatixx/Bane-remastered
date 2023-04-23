package cz.empatix.lightning.domain;

import com.google.common.base.Preconditions;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.util.Objects;


public final class LightSource {
    private final Vector2f position;
    private final Vector3f color;
    private final float radius;
    private final float intensity;
    private boolean isDestroyed = false;

    public LightSource(final Vector2f position,
                       final Vector3f color,
                       final float radius,
                       final float intensity) {
        Preconditions.checkArgument(radius > 0, "Radius must be greater than 0");
        Preconditions.checkArgument(intensity > 0, "Intensity must be greater than 0");
        Preconditions.checkNotNull(position, "Position must not be null");
        this.position = position;
        this.radius = radius;
        this.intensity = intensity;
        this.color = color;
    }

    public Vector2f getPosition() {
        return position;
    }

    public float getRadius() {
        return radius;
    }

    public float getIntensity() {
        return intensity;
    }

    public Vector3f getColor() {
        return color;
    }

    @Override
    public boolean equals(Object obj) {
        if (obj == this) return true;
        if (obj == null || obj.getClass() != this.getClass()) return false;
        var that = (LightSource) obj;
        return Objects.equals(this.position, that.position) &&
                Float.floatToIntBits(this.radius) == Float.floatToIntBits(that.radius) &&
                Float.floatToIntBits(this.intensity) == Float.floatToIntBits(that.intensity);
    }

    public boolean isDestroyed() {
        return isDestroyed;
    }
    public void destroy(){
        isDestroyed = true;
    }

    @Override
    public int hashCode() {
        return Objects.hash(position, radius, intensity);
    }

    @Override
    public String toString() {
        return "LightSource[" +
                "position=" + position + ", " +
                "radius=" + radius + ", " +
                "intensity=" + intensity + ']';
    }

    public void setPosition(final Vector2f translation) {
        setPosition(translation.x, translation.y);
    }
    public void setPosition(final float x, final float y){
        this.position.set(x, y);
    }
    public boolean canDisplay(){
        return !isDestroyed;
    }
}
