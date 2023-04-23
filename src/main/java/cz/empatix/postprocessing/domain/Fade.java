package cz.empatix.postprocessing.domain;

import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.shaderbindings.FadeBinding;

public class Fade implements Postprocess {

    private final static float MAX_TRANSPARENCY = 1f;
    private final static float MIN_TRANSPARENCY = 0f;
    private final static float DEFAULT_TWEEN = 0.01f;
    private static float transparency = MIN_TRANSPARENCY;
    private static float tween = DEFAULT_TWEEN;
    private static boolean incremental = true;
    private int textureId;

    @Override
    public void apply(float dt) {
        transparency += incremental ? transparency * tween : -transparency * tween;
        fixBounds();
    }

    @Override
    public void setInput(Framebuffer inputFBO) {
        textureId = inputFBO.colorAttachment();
    }


    private void fixBounds() {
        if (transparency > MAX_TRANSPARENCY) {
            transparency = MAX_TRANSPARENCY;
        } else if (transparency < MIN_TRANSPARENCY) {
            transparency = MIN_TRANSPARENCY;
        }
    }

    public static void fadeIn() {
        transparency = MIN_TRANSPARENCY;
        incremental = true;
    }

    public static void fadeOut() {
        transparency = MAX_TRANSPARENCY;
        incremental = false;
    }

    @Override
    public RenderData renderData() {
        return new RenderData(Framebuffer.VERTICES, textureId,
                new FadeBinding(transparency), RenderData.RenderDataType.DYNAMIC);
    }

    @Override
    public boolean canRender() {
        return false;
    }
}
