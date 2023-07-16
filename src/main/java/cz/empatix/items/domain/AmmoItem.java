package cz.empatix.items.domain;

import cz.empatix.guns.domain.GunType;
import cz.empatix.interaction.domain.InteractionZone;
import cz.empatix.interaction.service.InteractionService;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.concurrent.ThreadLocalRandom;

public class AmmoItem implements ItemDrop{
    private final static int DIMENSIONS = 64;
    private final InteractionZone interactionZone;
    private final GunType type;
    private final float percentAmount;
    private final Vector2f position;
    private boolean pickedUp = false;

    public AmmoItem(){
        this.position = new Vector2f();
        this.type = GunType.values()[ThreadLocalRandom.current().nextInt(GunType.values().length)];
        this.percentAmount = ThreadLocalRandom.current().nextFloat(0,0.05f);
        this.interactionZone = new InteractionZone(position, new Vector2i(DIMENSIONS), pressedKeys -> {

        });
        InteractionService.getInstance().addZone(interactionZone);
    }

    @Override
    public void update(float dt) {

    }

    @Override
    public boolean canRemove() {
        return pickedUp;
    }
    @Override
    public void setPosition(float x, float y) {
        position.set(x, y);
    }
}
