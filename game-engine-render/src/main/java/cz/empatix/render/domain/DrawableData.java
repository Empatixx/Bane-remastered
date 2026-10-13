package cz.empatix.render.domain;

import cz.empatix.render.attributes.Drawable;

import java.util.function.Supplier;

public record DrawableData(
        Supplier<RenderData> renderDataSupplier,
        RenderScene target,
        boolean canRender,
        int zIndex
){
    public static DrawableData from(final Drawable drawable){
        return new DrawableData(drawable::renderData, drawable.renderScene(), drawable.canRender(), drawable.getZIndex());
    }
    public RenderData renderData(){
        return renderDataSupplier.get();
    }
}
