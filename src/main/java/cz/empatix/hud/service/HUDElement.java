package cz.empatix.hud.service;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.RenderDataType;

public interface HUDElement extends Drawable {
    void update(final double dt);
    @Override
    default RenderDataType renderType() {
        return RenderDataType.DYNAMIC;
    }
    @Override
    default RenderScene renderScene() {
        return RenderScene.HUD;
    }
}
