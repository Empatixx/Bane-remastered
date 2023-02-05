package cz.empatix.render.domain;

import cz.empatix.common.GameLoop;
import org.joml.Vector3f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class InterpolatedPosition {
    private final static Logger LOGGER = LoggerFactory.getLogger(InterpolatedPosition.class);
    private final Vector3f currentPosition;
    private final Vector3f nextPosition;

    public InterpolatedPosition() {
        this.currentPosition = new Vector3f();
        this.nextPosition = new Vector3f();
    }

    public void setPosition(final float x, final float y) {
        this.nextPosition.x += x;
        this.nextPosition.y += y;
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
        final float t = dt / (float)GameLoop.FIXED_DELTA_TIME;
        this.currentPosition.lerp(nextPosition, t, currentPosition);
    }

}
