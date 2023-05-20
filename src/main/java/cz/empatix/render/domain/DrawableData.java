package cz.empatix.render.domain;

import cz.empatix.render.attributes.Drawable;

public record DrawableData(
        RenderData renderData,
        RenderScene target,
        boolean canRender,
        int zIndex
){
    public static DrawableData from(final Drawable drawable){
        return new DrawableData(drawable.renderData(), drawable.renderScene(), drawable.canRender(), drawable.getZIndex());
    }
}
