package cz.empatix.physics.domain.geometry;

import org.joml.Vector2f;

import java.util.Vector;

public interface Geometry {
    int getArea();
    GeometryType getType();

    boolean isColliding(final Vector2f translation, final Geometry other, final Vector2f otherTranslation);
}
