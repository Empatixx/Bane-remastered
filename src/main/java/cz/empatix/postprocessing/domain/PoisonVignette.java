package cz.empatix.postprocessing.domain;

import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.PoisonVignetteBinding;
import cz.empatix.render.shaderbindings.RedVignetteBinding;

import java.util.Set;

public class PoisonVignette implements Postprocess{
    private final static Set<RenderScene> SUPPORTED = Set.of(RenderScene.SCENE);
    private int textureId;
    private static float vignetteAmount;

    @Override
    public void apply(float dt) {
    }

    @Override
    public boolean accepts(RenderScene scene) {
        return SUPPORTED.contains(scene);
    }

    @Override
    public void setInput(Framebuffer inputFBO) {
        textureId = inputFBO.getColorAttachment();
    }

    @Override
    public RenderData renderData() {
        final float timeInSeconds = System.currentTimeMillis() % 3000 / 1000.0f;
        final float timeForShader = timeInSeconds * (2.0f * (float) Math.PI) / 3.0f;
        return new RenderData(Framebuffer.VERTICES, textureId,
                new PoisonVignetteBinding(Camera.getOriginWithoutZoomProjection(), vignetteAmount, timeForShader));

    }

    @Override
    public boolean canRender() {
        return vignetteAmount > 0;
    }
    public static void setTimeBased(){
        PoisonVignette.vignetteAmount = 0.5f;
    }
}
