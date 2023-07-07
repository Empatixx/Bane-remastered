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
    public boolean intersects(final Vector2f point) {
        return point.x >= position.x && point.x <= position.x + dimensions.x
                && point.y >= position.y && point.y <= position.y + dimensions.y;
    }

    public void interact(final Set<Integer> keys) {
        action.accept(keys);
    }
}
