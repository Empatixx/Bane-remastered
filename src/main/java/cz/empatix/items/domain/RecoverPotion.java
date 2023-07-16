package cz.empatix.items.domain;

import org.joml.Vector2f;

import java.util.concurrent.ThreadLocalRandom;

public class RecoverPotion implements ItemDrop{
    private final RecoverType type;
    private final int amount;
    private final Vector2f position;

    public RecoverPotion(){
        this(RecoverType.values()[ThreadLocalRandom.current().nextInt(RecoverType.values().length)], ThreadLocalRandom.current().nextInt(10, 30));
    }

    public RecoverPotion(final RecoverType type, final int amount) {
        this.type = type;
        this.amount = amount;
        this.position = new Vector2f();
    }

    public RecoverType getType() {
        return type;
    }

    public int getAmount() {
        return amount;
    }

    @Override
    public void update(float dt) {

    }

    @Override
    public boolean canRemove() {
        return false;
    }

    @Override
    public void setPosition(float x, float y) {
        position.set(x, y);
    }
}
