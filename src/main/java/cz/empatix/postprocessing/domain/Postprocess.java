package cz.empatix.postprocessing.domain;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;

public interface Postprocess extends Drawable {
    void apply(final float dt);

    boolean shouldApply();
}
