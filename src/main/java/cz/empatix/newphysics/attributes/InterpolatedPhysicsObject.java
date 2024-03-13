package cz.empatix.newphysics.attributes;

import cz.empatix.render.domain.InterpolatedPosition;
import org.dyn4j.geometry.Transform;
import org.dyn4j.geometry.Vector2;
import org.joml.Vector2f;

import java.lang.reflect.InvocationTargetException;

public class InterpolatedPhysicsObject extends PhysicsObject {
    private final InterpolatedPosition interpolatedPosition = new InterpolatedPosition();

    @Override
    public void setPhysicsTranslation(final float x, final float y) {
        super.setPhysicsTranslation(x, y);
        interpolatedPosition.setPosition(x, y);
    }

    @Override
    public void setTranslation(final float x, final float y) {
        super.setTranslation(x, y);
        interpolatedPosition.setPosition(x, y);
    }

    // sets physics translation and interpolated position for rendering
    public void setInterpolatedTranslation(final float x, final float y) {
        super.setTranslation(x, y);
        interpolatedPosition.setInterpolatedPosition(x, y);
    }

    public void updateInterpolatedPosition() {
        interpolatedPosition.setInterpolatedPosition(getPhysicsTranslationX(), getPhysicsTranslationY());
    }

    public float getTranslationX() {
        return interpolatedPosition.getInterpolatedX();
    }

    public float getTranslationY() {
        return interpolatedPosition.getInterpolatedY();
    }

    public Vector2f getTranslation() {
        return interpolatedPosition.getInterpolatedPosition();
    }

    @Override
    public void update(final float dt) {
        interpolatedPosition.update(dt);
    }
}
