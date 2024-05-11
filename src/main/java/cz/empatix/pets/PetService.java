package cz.empatix.pets;

import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;

import java.util.List;

public class PetService {
    private final Pet pet;

    public PetService(final Player owner) {
        this.pet = new Pet();
        pet.setOwner(owner);
        pet.setPosition(owner.getX(), owner.getY());
    }
    public void update(final float dt) {
        pet.update(dt);
    }
    public void fixedUpdate(final float dt) {
        pet.fixedUpdate(dt);
    }

    public List<? extends Renderable> toRender() {
        return List.of(pet);
    }
}
