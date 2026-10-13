package cz.empatix.utils;

public interface InterpolatedValue<T> {
    void setTarget(T value);
    void update(float dt);
    T get();
    T getTarget();
}
