package cz.empatix.enemies.targetting;

import org.joml.Vector2f;

public interface Targetable {
    Vector2f getPosition();
    int getWidth();
    int getHeight();
    default float getX() {
        return getPosition().x;
    }
    default float getY() {
        return getPosition().y;
    }
}
