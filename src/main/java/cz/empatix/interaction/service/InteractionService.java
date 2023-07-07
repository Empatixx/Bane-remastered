package cz.empatix.interaction.service;

import cz.empatix.interaction.domain.InteractionEntity;
import cz.empatix.interaction.domain.InteractionZone;

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
    private final List<InteractionZone> interactionZones = new ArrayList<>(20);
    private final List<InteractionEntity> interactionEntities = new ArrayList<>();

    public void fixedUpdate(final float dt) {
        A: for (final InteractionEntity entity : interactionEntities) {
            for (final InteractionZone interactionZone : interactionZones) {
                if (interactionZone.intersects(entity.position())) {
                    interactionZone.interact(entity.pressedKeys());
                    break A;
                }
            }
            entity.pressedKeys().clear();
        }
    }
    public void clean() {
        interactionEntities.clear();
        interactionZones.clear();
    }

    public void addEntity(final InteractionEntity interactionEntity) {
        interactionEntities.add(interactionEntity);
    }
    public void removeEntity(final InteractionEntity interactionEntity) {
        interactionEntities.remove(interactionEntity);
    }
    public void addZone(final InteractionZone interactionZone) {
        interactionZones.add(interactionZone);
    }
    public void removeZone(final InteractionZone interactionZone) {
        interactionZones.remove(interactionZone);
    }
}
