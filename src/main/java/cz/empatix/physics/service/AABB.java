package cz.empatix.physics.service;

import cz.empatix.physics.domain.Geometry;
import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.domain.Rectangle;
import org.joml.Vector2f;

public class AABB {
    public static boolean isIntersecting(final PhysicsBody body,
                                         final PhysicsBody other,
                                         final float dt) {
        final Geometry geometry = body.getGeometry();
        final Geometry otherGeometry = other.getGeometry();
        if (geometry instanceof Rectangle && otherGeometry instanceof Rectangle) {
            return isIntersecting(body, other, (Rectangle) geometry, (Rectangle) otherGeometry, dt);
        } else {
            throw new IllegalArgumentException("Unsupported geometry type");
        }
    }

    private static boolean isIntersecting(final PhysicsBody body,
                                          final PhysicsBody other,
                                          final Rectangle geometry,
                                          final Rectangle otherGeometry,
                                          final float dt) {
        final Vector2f translation = body.getTranslation();
        final Vector2f otherTranslation = other.getTranslation();

        final Vector2f velocity = body.getVelocity();
        final Vector2f otherVelocity = other.getVelocity();

        final float x = translation.x() + velocity.x() * dt;
        final float y = translation.y() + velocity.y() * dt;

        final float otherX = otherTranslation.x() + otherVelocity.x() * dt;
        final float otherY = otherTranslation.y() + otherVelocity.y() * dt;

        return x < otherX + otherGeometry.width() &&
                x + geometry.width() > otherX &&
                y < otherY + otherGeometry.height() &&
                y + geometry.height() > otherY;
    }
}
