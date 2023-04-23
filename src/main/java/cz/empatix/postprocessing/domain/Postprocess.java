package cz.empatix.postprocessing.domain;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Framebuffer;

public interface Postprocess extends Drawable {
    void apply(final float dt);

    void setInput(Framebuffer inputFBO);

    @Override
    default int getZIndex() {
        return 1;
    }
}
