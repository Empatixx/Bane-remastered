package cz.empatix.collision;

import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.PhysicsService;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.HashSet;
import java.util.List;
import java.util.Set;

/**
 * Game-side collision query helpers. Lives in app because inverting a set of
 * {@link CollisionType}s requires knowing the full set of categories, which is
 * game knowledge the engine must not own.
 */
public final class CollisionQueries {
    private CollisionQueries() {
    }

    /** Query for bodies whose category is one of {@code types} (ignoring all others). */
    public static List<PhysicsObject> queryType(final Vector2f position, final Vector2i dimensions,
                                                final Set<CollisionType> types) {
        final Set<CollisionType> ignored = new HashSet<>(Set.of(CollisionType.values()));
        ignored.removeAll(types);
        return PhysicsService.getInstance().query(position, dimensions, ignored);
    }
}
