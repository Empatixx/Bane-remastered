package cz.empatix.cursors.domain;

import cz.empatix.hud.service.image.AnimatedImage;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import org.joml.Vector2f;

public class ReloadCursor implements Cursor {
    private final static float SCALE = 0.5f;
    private final static float X_OFFSET = 32 * SCALE + 1;
    private final static float Y_OFFSET = 32 * SCALE + 1;
    private final AnimatedImage image = new AnimatedImage("ingame/reload_cursor.tga", new Vector2f(), SCALE, Integer.MAX_VALUE, RenderScene.OVERLAYS, 3);

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
        image.update(dt);
        image.setPosition(x - X_OFFSET, y - Y_OFFSET);
    }
    @Override
    public CursorType getType() {
        return CursorType.RELOAD;
    }
}
