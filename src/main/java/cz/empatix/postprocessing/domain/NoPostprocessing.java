package cz.empatix.postprocessing.domain;

import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;

public class NoPostprocessing implements Postprocess{
    private int textureId;
    @Override
    public void apply(float dt) {
    }

    @Override
    public boolean accepts(final RenderScene scene) {
        return true;
    }

    @Override
    public void setInput(final Framebuffer inputFBO) {
        textureId = inputFBO.colorAttachment();
    }

    @Override
    public RenderData renderData() {
        return new RenderData(Framebuffer.VERTICES, textureId, new CommonBinding(Camera.getProjectionMatrix()));
    }

    @Override
    public boolean canRender() {
        return true;
    }
}
