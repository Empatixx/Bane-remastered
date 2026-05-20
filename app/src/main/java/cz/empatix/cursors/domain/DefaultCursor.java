package cz.empatix.cursors.domain;

import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import org.joml.Vector2f;

public class DefaultCursor implements Cursor {
    private final static float SCALE = 0.5f;
    private final static float X_OFFSET = 12 * SCALE;
    private final static float Y_OFFSET = 7 * SCALE;
    private final CommonImage image = new CommonImage("ingame/cursor.tga", new Vector2f(),SCALE, 200, RenderScene.OVERLAYS);
    @Override
    public RenderData renderData() {
        return image.renderData();
    }

    @Override
    public boolean canRender() {
        return true;
    }

    @Override
    public void updateMouse(float x, float y, final float dt) {
        image.setPosition(x - X_OFFSET, y - Y_OFFSET);
    }

    @Override
    public CursorType getType() {
        return CursorType.DEFAULT;
    }
}
