package cz.empatix.cursors.domain;

import cz.empatix.hud.service.Image;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import org.joml.Vector2f;

public class AimCursor implements Cursor {
    private final static float SCALE = 0.5f;
    private final static float X_OFFSET = 32 * SCALE;
    private final static float Y_OFFSET = 32 * SCALE;
    private final Image image = new Image("ingame/crosshair.tga", new Vector2f(), SCALE, Integer.MAX_VALUE, RenderScene.OVERLAYS);

    @Override
    public RenderData renderData() {
        return image.renderData();
    }

    @Override
    public boolean canRender() {
        return true;
    }

    @Override
    public void updateMouse(float x, float y) {
        image.setPosition(x - X_OFFSET, y - Y_OFFSET);
    }
}
