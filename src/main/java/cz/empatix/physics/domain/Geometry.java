package cz.empatix.physics.domain;

import cz.empatix.physics.service.AABB;
import org.joml.Vector2f;

public interface Geometry {
    int getMass();
    boolean canAABB();
    boolean canSAT();
    AABB getAABB(final Vector2f translation);
}
