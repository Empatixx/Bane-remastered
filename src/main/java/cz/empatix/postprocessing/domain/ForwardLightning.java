package cz.empatix.postprocessing.domain;

import cz.empatix.lightning.service.LightningService;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.ForwardLightningBinding;

import java.util.EnumSet;
import java.util.Set;


public class ForwardLightning implements Postprocess {
    private final static Set<RenderScene> SUPPORTED = EnumSet.of(RenderScene.SCENE);
    private final LightningService lightningService = LightningService.getInstance();
    private int textureId;

    @Override
    public void apply(float dt) {
    }

    @Override
    public boolean accepts(final RenderScene scene) {
        return SUPPORTED.contains(scene);
    }

    @Override
    public void setInput(final Framebuffer inputFBO) {
        textureId = inputFBO.getColorAttachment();
    }

    @Override
    public RenderData renderData() {
        return new RenderData(Framebuffer.VERTICES, textureId,
                new ForwardLightningBinding(lightningService.lightsOnCamera(), Camera.getOriginWithoutZoomProjection()));
    }

    @Override
    public boolean canRender() {
        return !lightningService.lightsOnCamera().isEmpty();
    }

}
