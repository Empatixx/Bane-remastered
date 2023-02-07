package cz.empatix.render.domain;

import cz.empatix.common.GameLoop;
import org.joml.Vector3f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class InterpolatedPosition {
    private final static Logger LOGGER = LoggerFactory.getLogger(InterpolatedPosition.class);
    private final Vector3f currentPosition;
    private final Vector3f nextPosition;
    private final Vector3f previousPosition;
    private float t = 0;
    public InterpolatedPosition() {
        this.currentPosition = new Vector3f();
        this.nextPosition = new Vector3f();
        this.previousPosition = new Vector3f();
    }

    public void setPosition(final float x, final float y) {
        this.nextPosition.x += x;
        this.nextPosition.y += y;

        this.previousPosition.x = currentPosition.x;
        this.previousPosition.y = currentPosition.y;
        t = 0;
    }

    public float getX() {
        return currentPosition.x;
    }

    public float getY() {
        return currentPosition.y;
    }
    public Vector3f getPosition(){
        return currentPosition;
    }

    public void update(final float dt) {
        t += dt / (float)GameLoop.FIXED_DELTA_TIME;
        this.previousPosition.lerp(nextPosition, t, currentPosition);
    }

}
