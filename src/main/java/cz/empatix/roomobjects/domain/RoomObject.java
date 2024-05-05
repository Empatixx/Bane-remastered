package cz.empatix.roomobjects.domain;

import cz.empatix.render.attributes.Drawable;
import org.dyn4j.geometry.AABB;
import org.joml.Vector2f;

public interface RoomObject extends Drawable {
    void update(final double dt);
    boolean canRemove();
    default void destroy(){
    }
    int getWidth();
    int getHeight();
    Vector2f getPosition();
    default AABB getAABB(){
        final float width = getWidth();
        final float height = getHeight();
        final Vector2f position = getPosition();
        final float xMin = position.x;
        final float xMax = position.x + width;
        final float yMin = position.y;
        final float yMax = position.y + height;
        return new AABB(Math.min(xMin, xMax), Math.min(yMin, yMax), Math.max(xMin, xMax), Math.max(yMin, yMax));
    }
    default float getX(){
        return getPosition().x;
    }
    default float getY(){
        return getPosition().y;
    }
}
