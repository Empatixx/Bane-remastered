package cz.empatix.core;

import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class InterpolatedPosition {
    private final static Logger LOGGER = LoggerFactory.getLogger(InterpolatedPosition.class);
    private final Vector2f currentPosition;
    private final Vector2f nextPosition;
    private final Vector2f previousPosition;

    private float t = 0;
    private final float delta;

    public InterpolatedPosition(final float delta) {
        this.delta = delta;
        this.currentPosition = new Vector2f();
        this.nextPosition = new Vector2f();
        this.previousPosition = new Vector2f();
    }
    public InterpolatedPosition() {
        this.delta = (float) EngineTimestep.FIXED_DELTA_TIME;
        this.currentPosition = new Vector2f();
        this.nextPosition = new Vector2f();
        this.previousPosition = new Vector2f();
    }


    public void setInterpolatedPosition(final float x, final float y) {
        this.nextPosition.x = x;
        this.nextPosition.y = y;

        this.previousPosition.x = currentPosition.x;
        this.previousPosition.y = currentPosition.y;
        t = 0;
    }
    public void setPosition(final float x, final float y) {
        this.nextPosition.x = x;
        this.nextPosition.y = y;

        this.previousPosition.x = x;
        this.previousPosition.y = y;
        this.currentPosition.x = x;
        this.currentPosition.y = y;
        t = 0;
    }

    public float getInterpolatedX() {
        return currentPosition.x;
    }

    public float getInterpolatedY() {
        return currentPosition.y;
    }

    public Vector2f getInterpolatedPosition() {
        return currentPosition;
    }

    public float getY() {
        return nextPosition.y;
    }

    public float getX() {
        return nextPosition.x;
    }

    public Vector2f getPosition() {
        return nextPosition;
    }

    public void update(final float dt) {
        t += dt / delta;
        if (t > 1) {
            t = 1;
        }
        this.previousPosition.lerp(nextPosition, t, currentPosition);
    }

    public boolean isSamePosition() {
        return currentPosition.equals(nextPosition);
    }

    public float getT() {
        return t;
    }

}
