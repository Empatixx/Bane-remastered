package cz.empatix.tiles.domain;

import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;

public record EmptyTile() implements Tile{
    @Override
    public RenderData renderData() {
        throw new IllegalStateException("EmptyCarpetTile does not have render data");
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int getZIndex() {
        return 0;
    }

    @Override
    public boolean canRender() {
        return false;
    }

    @Override
    public void destroy() {

    }
}
