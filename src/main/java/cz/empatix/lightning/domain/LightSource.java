package cz.empatix.lightning.domain;

import com.google.common.base.Preconditions;
import cz.empatix.render.service.Camera;
import cz.empatix.render.service.ObservePayload;
import cz.empatix.render.service.ObserveSubject;
import cz.empatix.render.service.Observer;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;


public final class LightSource implements ObserveSubject {
    private final static int ADDITIONAL_FRUSTRUM = 300;
    private final Vector2f position;
    private final Vector3f color;
    private final float radius;
    private float intensity;
    private boolean isDestroyed = false;
    private final List<Observer> observers = new ArrayList<>();

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
        if (isDestroyed){
            return;
        }
        if (position.x == x && position.y == y){
            return;
        }
        this.position.set(x, y);
        notifyObservers(null);
    }
    public boolean canDisplay(){
        return !isDestroyed;
    }

    public boolean isOnCamera() {
        return Camera.getInstance().getX() < position.x + radius * intensity + ADDITIONAL_FRUSTRUM &&
                Camera.getInstance().getX() + Camera.getTargetWidth() > position.x - radius * intensity - ADDITIONAL_FRUSTRUM &&
                Camera.getInstance().getY() < position.y + radius * intensity + ADDITIONAL_FRUSTRUM &&
                Camera.getInstance().getY() + Camera.getTargetHeight() > position.y - radius * intensity - ADDITIONAL_FRUSTRUM;
    }

    public void setIntensity(final float intensity) {
        if (isDestroyed){
            return;
        }
        if (this.intensity == intensity){
            return;
        }
        this.intensity = intensity;
        notifyObservers(null);
    }


    @Override
    public void addObserver(Observer observer) {
        observers.add(observer);
    }

    @Override
    public void removeObserver(Observer observer) {
        observers.add(observer);
    }

    @Override
    public void clearObservers() {
        observers.clear();
    }

    @Override
    public void notifyObservers(ObservePayload payload) {
        observers.forEach(observer -> observer.update(this, payload));
    }
}
