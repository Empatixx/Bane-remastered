package cz.empatix.interaction.domain;

import org.joml.Vector2f;

import java.util.Set;

public interface InteractEntity {
    Set<Integer> getInteractKeys();

    Vector2f getPosition();

    int getHeight();

    int getWidth();

    boolean canRemove();

}
