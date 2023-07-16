package cz.empatix.items.domain;

public interface ItemDrop{
    void update(final float dt);
    boolean canRemove();
    void setPosition(final float x, final float y);
}
