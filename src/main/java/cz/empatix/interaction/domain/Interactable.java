package cz.empatix.interaction.domain;

import cz.empatix.enemies.targetting.Targetable;
import org.joml.Vector2f;

import java.util.Set;

public interface Interactable {
    void interact(final Set<Integer> keys);
    boolean canInteract();

    default boolean intersects(final InteractEntity e) {
        final Vector2f entityPos = e.getPosition();
        final Vector2f pos = getPosition();

        return entityPos.x() + e.getWidth() >= pos.x() &&
                entityPos.x() <= pos.x() + getWidth() &&
                entityPos.y() + e.getHeight() >= pos.y() &&
                entityPos.y() <= pos.y() + getHeight();
    }

    Vector2f getPosition();

    int getWidth();

    int getHeight();

    boolean canRemove();
}
