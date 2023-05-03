package cz.empatix.hud.service;

import cz.empatix.render.attributes.Drawable;

public interface HUDElement extends Drawable {
    void update(final double dt);
}
