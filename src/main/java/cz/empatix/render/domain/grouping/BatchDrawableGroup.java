package cz.empatix.render.domain.grouping;

import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.shaderbindings.ShaderBinding;

import java.util.List;

public class BatchDrawableGroup implements DrawableGroup{
    private final List<DrawableData> drawables;
    public BatchDrawableGroup(List<DrawableData> drawables) {
        this.drawables = drawables;
    }
    public BatchDrawableGroup(DrawableData... drawables) {
        this.drawables = List.of(drawables);
    }

    @Override
    public int getZIndex() {
        return drawables.get(0).zIndex();
    }

    @Override
    public ShaderBinding getShaderBinding() {
        return drawables.get(0).renderData().shaderBinding();
    }
}
