package cz.empatix.render;

import cz.empatix.render.domain.RenderData;

public interface Drawable
{
    RenderData renderData();
    boolean shouldRender();
}
