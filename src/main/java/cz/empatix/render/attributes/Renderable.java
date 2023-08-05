package cz.empatix.render.attributes;

import cz.empatix.render.attributes.grouping.RenderableGroup;

import java.util.List;

public interface Renderable {
    RenderableGroup getRenderableGroup();
    List<Renderable> getSubRenderables();
}
