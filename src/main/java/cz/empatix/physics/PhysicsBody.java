package cz.empatix.physics;

import com.google.common.base.Preconditions;
import cz.empatix.render.domain.InterpolatedPosition;
import org.dyn4j.dynamics.Body;
import org.dyn4j.geometry.Transform;
import org.joml.Vector2f;

public final class PhysicsBody extends Body {
    private final InterpolatedPosition position;

    public PhysicsBody(final InterpolatedPosition position) {
        Preconditions.checkNotNull(position);
        this.position = position;
    }

    public void setPosition(final float x, final float y) {
        this.transform.setTranslationX(x);
        this.transform.setTranslationY(y);
    }

    public void updatePosition(final float dt) {
        position.update(dt);
    }

    public Vector2f getInterpolatedPosition() {
        return position.getInterpolatedPosition();
    }

    public float getX() {
        return position.getX();
    }

    public float getY() {
        return position.getY();
    }

    public void setVelocity(final float x, final float y){
        this.setLinearVelocity(x,y);
    }

    void updatePosition() {
        final Transform tranform = getTransform();
        position.setPosition((float) tranform.getTranslationX(), (float) transform.getTranslationY());
    }

}
