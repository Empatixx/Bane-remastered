package cz.empatix.render.renders;

import cz.empatix.render.domain.RenderData;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class StaticRenderer implements Renderer{
    final Map<Class,List<RenderData>> renderDataMap;
    public StaticRenderer(){
        renderDataMap = new HashMap<>();
    }
    @Override
    public boolean accepts(RenderData renderData) {
        return renderData.type() == RenderData.RenderDataType.STATIC;
    }

    @Override
    public void render(RenderData renderData) {
        batchRender(List.of(renderData));
    }

    @Override
    public void batchRender(List<RenderData> renderData) {
        final int hash = renderData.hashCode();
    }
}
