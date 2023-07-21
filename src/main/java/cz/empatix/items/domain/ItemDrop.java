package cz.empatix.items.domain;

import cz.empatix.interaction.domain.Interactable;
import cz.empatix.render.attributes.Drawable;

public interface ItemDrop extends Drawable, Interactable {
    void update(final float dt);
    boolean canRemove();
    void setPosition(final float x, final float y);
    void setDespawnable(final boolean despawn);
    int getWidth();
    int getHeight();
    void destroy();
}
