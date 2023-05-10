package cz.empatix.render.domain;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.RenderScene;

public record DrawableData(
        RenderData renderData,
        RenderDataType type,
        RenderScene target,
        boolean canRender,
        int zIndex
){
    public static DrawableData from(final Drawable drawable){
        return new DrawableData(drawable.renderData(), drawable.renderType(), drawable.renderScene(), drawable.canRender(), drawable.getZIndex());
    }
}
