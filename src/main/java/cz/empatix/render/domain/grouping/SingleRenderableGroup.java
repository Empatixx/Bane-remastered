package cz.empatix.render.domain.grouping;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.shaderbindings.ShaderBinding;

public record SingleRenderableGroup(
        DrawableData drawable
) implements RenderableGroup {
    public SingleRenderableGroup(final Drawable drawable) {
        this(DrawableData.from(drawable));
    }
    @Override
    public int getZIndex() {
        return drawable.zIndex();
    }

    @Override
    public ShaderBinding getShaderBinding() {
        return drawable.renderData().shaderBinding();
    }
}
