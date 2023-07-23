package cz.empatix.hud.service;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.render.domain.RenderData;
import org.joml.Vector2f;

public class CommonBackground implements HUDElement {
    private final CommonImage image;

    public CommonBackground(final String texture) {
        this.image = new CommonImage(texture, new Vector2f(0,0), WindowHandler.TARGET_WIDTH, WindowHandler.TARGET_HEIGHT);
    }
    @Override
    public RenderData renderData() {
        return image.renderData();
    }
    @Override
    public boolean canRender() {
        return true;
    }

    @Override
    public int getZIndex() {
        return -1;
    }

    @Override
    public void update(double dt) {

    }
}
