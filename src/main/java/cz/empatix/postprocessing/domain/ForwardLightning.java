package cz.empatix.postprocessing.domain;

import cz.empatix.lightning.service.LightningService;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.shaderbindings.ShaderBinding;


public class ForwardLightning implements Postprocess {
    private final LightningService lightningService = LightningService.getInstance();
    private final ForwardLightningBinding shaderBinding = new ForwardLightningBinding(lightningService);
    private int textureId;
    @Override
    public void apply(float dt) {
    }

    @Override
    public void setInput(Framebuffer inputFBO) {
        textureId = inputFBO.getColorAttachment();
    }

    @Override
    public RenderData renderData() {
        return new RenderData(Framebuffer.VERTICES, textureId, shaderBinding, RenderData.RenderDataType.DYNAMIC);
    }

    @Override
    public boolean shouldRender() {
        return false;
    }

}
