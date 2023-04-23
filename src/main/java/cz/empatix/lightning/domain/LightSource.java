package cz.empatix.lightning.domain;

import com.google.common.base.Preconditions;
import org.joml.Vector2f;

import java.util.Objects;


public final class LightSource {
    private final Vector2f position;
    private final float radius;
    private final float intensity;
    private boolean isDestroyed = false;

    public LightSource(Vector2f position, float radius, float intensity) {
        Preconditions.checkArgument(radius > 0, "Radius must be greater than 0");
        Preconditions.checkArgument(intensity > 0, "Intensity must be greater than 0");
        Preconditions.checkNotNull(position, "Position must not be null");
        this.position = position;
        this.radius = radius;
        this.intensity = intensity;
    }

    public Vector2f position() {
        return position;
    }

    public float radius() {
        return radius;
    }

    public float intensity() {
        return intensity;
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

}
