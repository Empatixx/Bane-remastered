package cz.empatix.interaction.domain;

import org.joml.Vector2f;

import java.util.HashSet;
import java.util.Set;

public class CommonInteractEntity implements InteractEntity{

    private final Set<Integer> keyPressed = new HashSet<>(20);
    private final int width;
    private final int height;
    private final Vector2f position;
    public CommonInteractEntity(final int width,
                                final int height,
                                final Vector2f position){
        this.width = width;
        this.height = height;
        this.position = position;
    }
    @Override
    public Set<Integer> getInteractKeys() {
        return keyPressed;
    }

    @Override
    public Vector2f getPosition() {
        return position;
    }

    @Override
    public int getHeight() {
        return height;
    }

    @Override
    public int getWidth() {
        return width;
    }

    @Override
    public boolean canRemove() {
        throw new IllegalStateException();
    }

    public void addKey(final int key) {
        keyPressed.add(key);
    }
}
