package cz.empatix.render.hud;


import cz.empatix.render.domain.RenderData;

public class Text implements HUDElement {
    @Override
    public RenderData renderData() {
        return null;
    }

    @Override
    public boolean shouldRender() {
        return false;
    }

    @Override
    public void update(double dt) {

    }
}
