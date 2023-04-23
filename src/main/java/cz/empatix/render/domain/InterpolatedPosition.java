package cz.empatix.render.domain;

import cz.empatix.common.GameLoop;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class InterpolatedPosition {
    private final static Logger LOGGER = LoggerFactory.getLogger(InterpolatedPosition.class);
    private final Vector2f currentPosition;
    private final Vector2f nextPosition;
    private final Vector2f previousPosition;
    private float t = 0;
    public InterpolatedPosition() {
        this.currentPosition = new Vector2f();
        this.nextPosition = new Vector2f();
        this.previousPosition = new Vector2f();
    }

    public void setPosition(final float x, final float y) {
        this.nextPosition.x = x;
        this.nextPosition.y = y;

        this.previousPosition.x = currentPosition.x;
        this.previousPosition.y = currentPosition.y;
        t = 0;
    }

    public float getInterpolatedX() {
        return currentPosition.x;
    }

    public float getInterpolatedY() {
        return currentPosition.y;
    }
    public Vector2f getInterpolatedPosition(){
        return currentPosition;
    }
    public float getY(){
        return nextPosition.y;
    }
    public float getX(){
        return nextPosition.x;
    }
    public Vector2f getPosition(){
        return nextPosition;
    }

    public void update(final float dt) {
        t += dt / (float)GameLoop.FIXED_DELTA_TIME;
        if (t > 1) {
            t = 1;
        }
        this.previousPosition.lerp(nextPosition, t, currentPosition);
    }

}
