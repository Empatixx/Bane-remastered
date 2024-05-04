package cz.empatix.items.service;

import cz.empatix.render.domain.InterpolatedPosition;
import org.joml.Vector2f;

import java.util.concurrent.ThreadLocalRandom;

public class FloatingEffectService {
    private static final ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private static final float DEFAULT_OFFSET = 2.5f;
    private static final float DEFAULT_SPEED = 0.25f;
    private final InterpolatedPosition offset;
    private final float offsetValue;
    public FloatingEffectService(){
        this(DEFAULT_OFFSET, DEFAULT_SPEED);
    }
    public FloatingEffectService(final float offset, final float speed){
        this.offset = new InterpolatedPosition(speed);
        this.offset.setPosition(0, offset);
        this.offsetValue = offset;
        update(RANDOM.nextBoolean() ? Float.MAX_VALUE : 0); // Randomize the starting position
        update(RANDOM.nextFloat() * 0.5f); // Randomize the starting position
    }
    public void update(final float dt){
        offset.update(dt);
        if (offset.getT() >= 1){
            offset.setInterpolatedPosition(0, (offset.getInterpolatedY() >= 0) ? -offsetValue : offsetValue);
        }
    }
    public float getOffsetY(){
        return offset.getInterpolatedY();
    }
}
