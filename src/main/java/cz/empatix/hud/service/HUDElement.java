package cz.empatix.hud.service;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderScene;

public interface HUDElement extends Drawable {
    void update(final double dt);
    @Override
    default RenderScene renderScene() {
        return RenderScene.HUD;
    }
}
