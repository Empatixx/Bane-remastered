package cz.empatix.roomobjects.domain;

import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;

public class Barrel implements RoomObject{
    @Override
    public RenderData renderData() {
        return null;
    }

    @Override
    public RenderScene renderScene() {
        return null;
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
    public void update(double dt) {

    }

    @Override
    public boolean canRemove() {
        return false;
    }
}
