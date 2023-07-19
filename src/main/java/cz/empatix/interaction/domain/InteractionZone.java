package cz.empatix.interaction.domain;

import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.Set;
import java.util.function.Consumer;

public record InteractionZone(
        Vector2f position,
        Vector2i dimensions,
        Consumer<Set<Integer>> action
) {
    public boolean intersects(final InteractionEntity entity){
        final Vector2f entityPosition = entity.position();
        final Vector2i entityDims = entity.dimensions();

        return entityPosition.x() + entityDims.x() >= position.x() && entityPosition.x() <= position.x() + dimensions.x()
                && entityPosition.y() + entityDims.y() >= position.y() && entityPosition.y() <= position.y() + dimensions.y();
    }

    public void interact(final Set<Integer> keys) {
        action.accept(keys);
    }
}
