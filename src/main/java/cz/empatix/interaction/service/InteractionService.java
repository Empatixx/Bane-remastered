package cz.empatix.interaction.service;

import cz.empatix.interaction.domain.InteractEntity;
import cz.empatix.interaction.domain.Interactable;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.lwjgl.opengl.GL11;
import org.lwjgl.opengl.GL30;

import java.util.ArrayList;
import java.util.List;
import java.util.Set;

public class InteractionService {
    private static InteractionService instance;
    public static InteractionService getInstance() {
        if(instance == null){
            instance = new InteractionService();
        }
        return instance;
    }
    private final List<Interactable> interactables = new ArrayList<>(20);
    private final List<InteractEntity> interactionEntities = new ArrayList<>();

    public void fixedUpdate(final float dt) {
        interactables.removeIf(Interactable::canRemove);
        interactionEntities.removeIf(InteractEntity::canRemove);

        A: for (final InteractEntity entity : interactionEntities) {
            final Set<Integer> keys = entity.getInteractKeys();
            for (final Interactable interactable : interactables) {
                if (interactable.intersects(entity) && interactable.canInteract(keys)) {
                    interactable.interact(keys);
                    entity.getInteractKeys().clear();
                    break A;
                }
            }
            entity.getInteractKeys().clear();
        }
    }
    public void clear() {
        interactionEntities.clear();
        interactables.clear();
    }
    public void addEntity(final InteractEntity entity){
        interactionEntities.add(entity);
    }

    public void removeEntity(final InteractEntity entity) {
        interactionEntities.remove(entity);
    }
    public void addInteractable(final Interactable interactable){
        interactables.add(interactable);
    }
    public void removeInteractable(final Interactable interactable){
        interactables.remove(interactable);
    }

    public List<Interactable> query(Vector2f position, Vector2i dimensions) {
        final List<Interactable> result = new ArrayList<>();
        for (final Interactable interactable : interactables) {
            final Vector2f interactablePosition = interactable.getPosition();
            if (interactablePosition.x() + interactable.getWidth() >= position.x &&
                    interactablePosition.x() <= position.x + dimensions.x &&
                    interactablePosition.y() + interactable.getHeight() >= position.y &&
                    interactablePosition.y() <= position.y + dimensions.y) {
                result.add(interactable);
            }
        }
        return result;
    }
}
