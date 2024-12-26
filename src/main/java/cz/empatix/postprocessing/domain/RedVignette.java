package cz.empatix.postprocessing.domain;

import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.RedVignetteBinding;

import java.util.EnumSet;
import java.util.Set;

public class RedVignette implements Postprocess {
    private final static Set<RenderScene> SUPPORTED = EnumSet.of(RenderScene.SCENE);
    private final static float VIGNETTE_TWEEN = 2f;
    private static float vignetteAmount;
    private static boolean timeBased;
    private int textureId;

    public static void reset() {
        vignetteAmount = 0;
        timeBased = false;
    }

    public static boolean isTimeBased() {
        return timeBased;
    }

    @Override
    public void apply(final float dt) {
        if (timeBased) {
            return;
        }
        vignetteAmount -= vignetteAmount * dt * VIGNETTE_TWEEN;
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
        final float timeInSeconds = System.currentTimeMillis() % 3000 / 1000.0f;
        final float timeForShader = timeInSeconds * (2.0f * (float) Math.PI) / 3.0f;
        return new RenderData(Framebuffer.VERTICES, textureId,
                new RedVignetteBinding(Camera.getProjectionMatrix(), vignetteAmount, timeBased, timeForShader));
    }

    @Override
    public boolean canRender() {
        return vignetteAmount > 0 || timeBased;
    }

    public static void setMaxVignette() {
        RedVignette.vignetteAmount = 1;
    }

    public static void setTimeBased(final boolean timeBased) {
        RedVignette.timeBased = timeBased;
    }

}
