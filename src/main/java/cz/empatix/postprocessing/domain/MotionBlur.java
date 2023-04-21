package cz.empatix.postprocessing.domain;

import cz.empatix.render.domain.RenderData;

public class MotionBlur implements Postprocess{





    @Override
    public void apply(float dt) {

    }

    @Override
    public boolean shouldApply() {
        return false;
    }

    @Override
    public RenderData renderData() {
        return null;
    }

    @Override
    public boolean shouldRender() {
        return false;
    }

    @Override
    public int getZIndex() {
        return 0;
    }
}
