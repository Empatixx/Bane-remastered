package cz.empatix.render.attributes;

import cz.empatix.render.attributes.grouping.RenderableGroup;
import cz.empatix.render.domain.RenderableState;

public interface Renderable {
    RenderableGroup getRenderableGroup();
    void collectSubRenderables(RenderableState state);
}
