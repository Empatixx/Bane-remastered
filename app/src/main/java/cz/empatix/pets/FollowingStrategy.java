package cz.empatix.pets;

import cz.empatix.enemies.targetting.Targetable;
import org.joml.Vector2f;

public interface FollowingStrategy {
    boolean hasTarget();
    void follow(Targetable followTarget);
    void update(final float dt);
    void fixedUpdate(final float dt);
    Vector2f getDirection();
}
