package cz.empatix.postprocessing.domain;

import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.GaussianBlurBinding;

import java.util.EnumSet;
import java.util.Set;

public class GaussianBlur implements Postprocess{
    private final static int MAX_BLUR_AMOUNT = 5;
    private final static float MAX_DARKEN_AMOUNT = 0.3f;
    private final static float BLUR_STEP = 5f;
    private final static float DARKNESS_STEP = 0.5f;
    private final static Set<RenderScene> SUPPORTED = EnumSet.of(RenderScene.SCENE, RenderScene.HUD);
    private int textureId;
    private static float blurAmount = 0;
    private static float darkness = 0;
    private static boolean enabled = false;
    @Override
    public void apply(final float dt) {
        if (blurAmount < MAX_BLUR_AMOUNT) {
            blurAmount += BLUR_STEP * dt;
        }
        if (darkness < MAX_DARKEN_AMOUNT) {
            darkness += DARKNESS_STEP * dt;
        }
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
                new GaussianBlurBinding(blurAmount, 1f - darkness));
    }

    @Override
    public boolean canRender() {
        return enabled;
    }

    public static void setEnabled(final boolean enabled) {
        GaussianBlur.enabled = enabled;
        if(!enabled) {
            blurAmount = 0;
            darkness = 0;
        }
    }
}
