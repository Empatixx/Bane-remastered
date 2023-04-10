package cz.empatix.physics.domain.transform;

import cz.empatix.common.GameLoop;
import org.joml.Vector2f;

public class InterpolatedTransform implements Transform {
    private final Vector2f currentPosition = new Vector2f();
    private final Vector2f nextPosition = new Vector2f();
    private final Vector2f previousPosition = new Vector2f();
    private float t = 0;
    @Override
    public void translate(float x, float y) {
        this.nextPosition.x = nextPosition.x + x;
        this.nextPosition.y = nextPosition.y + y;

        this.previousPosition.x = currentPosition.x;
        this.previousPosition.y = currentPosition.y;
        t = 0;
    }

    @Override
    public void setTranslation(float x, float y) {
        this.nextPosition.x = x;
        this.nextPosition.y = y;

        this.previousPosition.x = currentPosition.x;
        this.previousPosition.y = currentPosition.y;
        t = 0;
    }

    @Override
    public Vector2f getTranslation() {
        return nextPosition;
    }

    @Override
    public Vector2f getTranslationToRender() {
        return currentPosition;
    }

    @Override
    public void update(final float dt) {
        t += dt / (float) GameLoop.FIXED_DELTA_TIME;
        this.previousPosition.lerp(nextPosition, t, currentPosition);
    }
}
