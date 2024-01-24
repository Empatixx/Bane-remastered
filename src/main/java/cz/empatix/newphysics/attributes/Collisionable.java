package cz.empatix.newphysics.attributes;

import org.joml.Vector2f;

public interface Collisionable {
    void setAngle(double angle);
    void setPosition(float x, float y);
    Vector2f getPosition();
    float getAngle();
    boolean supportsSAT();
    boolean supportsAABB();
}
