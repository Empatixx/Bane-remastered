package cz.empatix.render.domain;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.RenderScene;

public record DrawableData(
        RenderData renderData,
        RenderScene targetScene,
        boolean canRender,
        int zIndex
){
    public static DrawableData fromDrawable(Drawable d){
        return new DrawableData(d.renderData(), RenderScene.SCENE, d.canRender(), d.getZIndex());
    }
}
