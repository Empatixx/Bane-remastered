package cz.empatix.utils;

public class InterpolatedAngle implements InterpolatedValue<Float> {
    private static final float TWO_PI = (float) (2 * Math.PI);
    private static final float PI = (float) Math.PI;

    private float current;
    private float target;
    private final float speed;

    public InterpolatedAngle(final float initial, final float speed) {
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
        // Normalize difference to [-PI, PI] for shortest path rotation
        float diff = target - current;
        while (diff > PI) {
            diff -= TWO_PI;
        }
        while (diff < -PI) {
            diff += TWO_PI;
        }

        final float t = Math.min(1f, speed * dt);
        current = current + diff * t;
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
