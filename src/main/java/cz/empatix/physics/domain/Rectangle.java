package cz.empatix.physics.domain;

import cz.empatix.physics.service.AABB;
import org.joml.Vector2f;

public record Rectangle(
        int width,
        int height
) implements Geometry{
    public static Rectangle from(final int width, final int height){
        return new Rectangle(width, height);
    }
    @Override
    public int getMass() {
        return width * height;
    }

    @Override
    public boolean canAABB() {
        return true;
    }

    @Override
    public boolean canSAT() {
        return true;
    }

    @Override
    public AABB getAABB(final Vector2f translation) {
        return new AABB(
                translation.x,
                translation.x + width,
                translation.y,
                translation.y + height
        );
    }

}
