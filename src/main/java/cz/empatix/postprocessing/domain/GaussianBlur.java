package cz.empatix.postprocessing.domain;

import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.GaussianBlurBinding;

import java.util.Set;

public class GaussianBlur implements Postprocess{
    private final static Set<RenderScene> SUPPORTED = Set.of(RenderScene.SCENE, RenderScene.HUD);
    private int textureId;
    private float blurAmount = 10f;
    @Override
    public void apply(final float dt) {

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
                new GaussianBlurBinding(blurAmount, Camera.getOriginWithoutZoomProjection()));
    }

    @Override
    public boolean canRender() {
        return false;
    }

}
