package cz.empatix.postprocessing.domain;

import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.FadeBinding;

import java.util.EnumSet;
import java.util.Set;

public class Fade implements Postprocess {
    private final static Set<RenderScene> SUPPORTED = EnumSet.of(RenderScene.HUD, RenderScene.SCENE);
    private final static float MAX_VALUE = 5f;
    private final static float MIN_VALUE = 0f;
    private final static float DEFAULT_TWEEN = 0f;
    private static float value = MIN_VALUE;
    private static float tween = DEFAULT_TWEEN;
    private int textureId;

    @Override
    public void apply(final float dt) {
        value += tween * dt;
        fixBounds();
    }

    @Override
    public boolean accepts(final RenderScene scene) {
        return SUPPORTED.contains(scene);
    }

    @Override
    public void setInput(final Framebuffer inputFBO) {
        textureId = inputFBO.colorAttachment();
    }


    private void fixBounds() {
        if (value > MAX_VALUE) {
            value = MAX_VALUE;
        } else if (value < MIN_VALUE) {
            value = MIN_VALUE;
        }
    }

    @Override
    public RenderData renderData() {
        return new RenderData(Framebuffer.VERTICES, textureId,
                new FadeBinding(value, Camera.getProjectionMatrix()));
    }

    @Override
    public boolean canRender() {
        return tween != 0 || MIN_VALUE < value;
    }
    public static float getValue() {
        return value;
    }
    public static void setTween(final float tween) {
        Fade.tween = tween;
    }
}
