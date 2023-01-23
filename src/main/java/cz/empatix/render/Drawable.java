package cz.empatix.render;

import cz.empatix.render.domain.RenderData;

import java.util.List;

public interface Drawable
{
    RenderData renderData();
    boolean shouldRender();
    default List<Drawable> getSubDrawables() {
        return List.of();
    }
}
