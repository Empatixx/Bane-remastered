package cz.empatix.roomobjects.domain;

import cz.empatix.render.attributes.Drawable;

public interface RoomObject extends Drawable {
    void update(final double dt);
    boolean canRemove();
}
