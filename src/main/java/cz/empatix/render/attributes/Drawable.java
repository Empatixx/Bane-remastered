package cz.empatix.render.attributes;

import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderDataType;

import java.util.List;

public interface Drawable
{
    RenderData renderData();
    RenderScene renderScene();
    RenderDataType renderType();
    boolean canRender();
    int getZIndex();
    default List<Drawable> getSubDrawables() {
        return List.of();
    }
}
