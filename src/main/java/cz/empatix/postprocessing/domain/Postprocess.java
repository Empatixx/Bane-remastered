package cz.empatix.postprocessing.domain;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.RenderScene;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderDataType;

public interface Postprocess extends Drawable {
    void apply(final float dt);
    boolean accepts(final RenderScene scene);
    @Override
    default RenderDataType renderType() {
        return RenderDataType.DYNAMIC;
    }
    @Override
    default RenderScene renderScene() {
        return RenderScene.POSTPROCESSING;
    }

    void setInput(Framebuffer inputFBO);

    @Override
    default int getZIndex() {
        return 1;
    }
}
