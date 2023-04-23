package cz.empatix.postprocessing.domain;

import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderData;

public class MotionBlur implements Postprocess{





    @Override
    public void apply(float dt) {

    }
    @Override
    public void setInput(Framebuffer inputFBO) {

    }

    @Override
    public RenderData renderData() {
        return null;
    }

    @Override
    public boolean canRender() {
        return false;
    }

}
