package cz.empatix.interaction.domain;

import org.joml.Vector2f;

import java.util.Set;
import java.util.function.Consumer;
import java.util.function.Supplier;

public class CommonInteractable implements Interactable {
    private final Consumer<Set<Integer>> interactConsumer;
    private final Supplier<Boolean> canInteract;
    private final Supplier<Boolean> canRemoveSupplier;
    private final Vector2f position;
    private final int width;
    private final int height;

    public CommonInteractable(final Consumer<Set<Integer>> interactConsumer,
                              final Supplier<Boolean> canInteract,
                              final Supplier<Boolean> canRemoveSupplier,
                              final Vector2f position,
                              final int height,
                              final int width) {
        this.interactConsumer = interactConsumer;
        this.canRemoveSupplier = canRemoveSupplier;
        this.canInteract = canInteract;
        this.position = position;
        this.width = width;
        this.height = height;
    }

    @Override
    public void interact(final Set<Integer> keys) {
        interactConsumer.accept(keys);
    }

    @Override
    public boolean canInteract() {
        return canInteract.get();
    }

    @Override
    public Vector2f getPosition() {
        return position;
    }

    @Override
    public int getWidth() {
        return width;
    }

    @Override
    public int getHeight() {
        return height;
    }

    @Override
    public boolean canRemove() {
        return canRemoveSupplier.get();
    }

    public void setPosition(final float x, final float y) {
        position.set(x, y);
    }
}
