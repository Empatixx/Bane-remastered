package cz.empatix.physics.service;

import com.google.common.base.Preconditions;
import cz.empatix.physics.domain.Geometry;
import cz.empatix.physics.domain.Rectangle;
import org.joml.Vector2f;

public class AABB {
    public static boolean isIntersecting(final Geometry geometry,
                                         final Geometry other,
                                         final Vector2f translation,
                                         final Vector2f otherTranslation) {
        Preconditions.checkArgument(geometry.canAABB() && other.canAABB());
        if (geometry instanceof Rectangle && other instanceof Rectangle) {
            return isIntersecting((Rectangle) geometry, (Rectangle) other, translation, otherTranslation);
        } else {
            throw new IllegalArgumentException("Unsupported geometry type");
        }
    }

    private static boolean isIntersecting(final Rectangle geometry,
                                          final Rectangle other,
                                          final Vector2f translation,
                                          final Vector2f otherTranslation) {
        return translation.x() < otherTranslation.x() + other.width() &&
                translation.x() + geometry.width() > otherTranslation.x() &&
                translation.y() < otherTranslation.y() + other.height() &&
                translation.y() + geometry.height() > otherTranslation.y();
    }
}
