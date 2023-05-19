package cz.empatix.render.domain.grouping;

import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.shaderbindings.ShaderBinding;

public record SingleDrawableGroup(
        DrawableData drawable
) implements DrawableGroup{
    @Override
    public int getZIndex() {
        return drawable.zIndex();
    }

    @Override
    public ShaderBinding getShaderBinding() {
        return drawable.renderData().shaderBinding();
    }
}
