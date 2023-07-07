package cz.empatix.interaction.domain;

import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.HashSet;
import java.util.Set;

public record InteractionEntity(
        Vector2f position,
        Vector2i dimensions,
        Set<Integer> pressedKeys
) {
    public InteractionEntity(final Vector2f position, final int width, final int height){
        this(position, new Vector2i(width,height), new HashSet<>());
    }
    public void setPosition(final Vector2f position) {
        this.position.set(position);
    }
    public void keyPress(final int key) {
        pressedKeys.add(key);
    }
}
