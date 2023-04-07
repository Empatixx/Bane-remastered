package cz.empatix.physics.domain;

import org.joml.Vector2f;

public interface Moveable {
    void move(double dt);
    Vector2f getVelocity();
}
