package cz.empatix.render.renders;

import cz.empatix.render.domain.renderdata.RenderData;

public interface Renderer {
    boolean accepts(RenderData renderData);
    void render(RenderData renderData);
}
