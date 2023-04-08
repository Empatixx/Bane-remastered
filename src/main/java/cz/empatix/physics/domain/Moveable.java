package cz.empatix.physics.domain;

import org.joml.Vector2f;

public interface Moveable {
    void move(final float dt);
    Vector2f getVelocity();
}
