package cz.empatix.render.attributes.grouping;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.shaderbindings.ShaderProgramBinding;

import java.util.List;

public record SingleRenderableGroup(
        DrawableData drawableData
) implements RenderableGroup {
    public SingleRenderableGroup(final Drawable drawable) {
        this(DrawableData.from(drawable));
    }
    @Override
    public int getZIndex() {
        return drawableData.zIndex();
    }

    @Override
    public ShaderProgramBinding getShaderBinding() {
        return drawableData.renderData().shaderProgramBinding();
    }

    @Override
    public RenderScene getRenderScene() {
        return drawableData.target();
    }

    @Override
    public List<DrawableData> getDrawableData() {
        return List.of(drawableData);
    }

    @Override
    public boolean isRenderable() {
        return drawableData.canRender();
    }
}
