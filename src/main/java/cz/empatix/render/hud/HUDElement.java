package cz.empatix.render.hud;

import cz.empatix.render.Drawable;

public interface HUDElement extends Drawable {
    void update(final double dt);
}
