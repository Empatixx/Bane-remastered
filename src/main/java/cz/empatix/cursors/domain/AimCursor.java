package cz.empatix.cursors.domain;

import cz.empatix.hud.service.Image;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import org.joml.Vector2f;

public class AimCursor implements Cursor {
    private final static int X_OFFSET = 32;
    private final static int Y_OFFSET = 32;
    private final Image image = new Image("ingame/crosshair.tga", new Vector2f(), 1f, Integer.MAX_VALUE, RenderScene.OVERLAYS);

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
