package cz.empatix.interaction.service;

import cz.empatix.interaction.domain.InteractEntity;
import cz.empatix.interaction.domain.Interactable;

import java.util.ArrayList;
import java.util.List;

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
            for (final Interactable interactable : interactables) {
                if (interactable.intersects(entity) && interactable.canInteract()) {
                    interactable.interact(entity.getInteractKeys());
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
}
