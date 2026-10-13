package cz.empatix.utils;

public class InterpolatedFloat implements InterpolatedValue<Float> {
    private float current;
    private float target;
    private final float speed;

    public InterpolatedFloat(final float initial, final float speed) {
        this.current = initial;
        this.target = initial;
        this.speed = speed;
    }

    @Override
    public void setTarget(final Float value) {
        this.target = value;
    }

    @Override
    public void update(final float dt) {
        final float t = Math.min(1f, speed * dt);
        current = current + (target - current) * t;
    }

    @Override
    public Float get() {
        return current;
    }

    @Override
    public Float getTarget() {
        return target;
    }
}
