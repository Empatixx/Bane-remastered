package cz.empatix.items.domain;

import cz.empatix.render.attributes.Drawable;

public interface ItemDrop extends Drawable {
    void update(final float dt);
    boolean canRemove();
    void setPosition(final float x, final float y);
    int getWidth();
    int getHeight();
}
