package cz.empatix.postprocessing.domain;

import cz.empatix.lightning.service.LightningService;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.ForwardLightningBinding;


public class ForwardLightning implements Postprocess {
    private final LightningService lightningService = LightningService.getInstance();
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
        return new RenderData(Framebuffer.VERTICES, textureId,
                new ForwardLightningBinding(lightningService.getLightSources(), Camera.getOriginWithoutZoomProjection()),
                RenderData.RenderDataType.DYNAMIC);
    }

    @Override
    public boolean canRender() {
        return !lightningService.getLightSources().isEmpty();
    }

}
