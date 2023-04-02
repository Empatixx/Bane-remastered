package cz.empatix.physics.domain;

public interface Moveable {
    void move(final double dt);
    void postPhysicsUpdate(final float x, final float y);
}
