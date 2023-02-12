package cz.empatix.render.renders;

import cz.empatix.render.domain.RenderData;

import java.util.List;

public interface Renderer {
    void render(RenderData renderData);
    void batchRender(List<RenderData> renderData);
}
