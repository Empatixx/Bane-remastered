package cz.empatix.render.attributes;

import cz.empatix.render.domain.RenderData;

import java.util.List;

public interface Drawable
{
    RenderData renderData();
    boolean shouldRender();
    int getZIndex();
    default List<Drawable> getSubDrawables() {
        return List.of();
    }
}
