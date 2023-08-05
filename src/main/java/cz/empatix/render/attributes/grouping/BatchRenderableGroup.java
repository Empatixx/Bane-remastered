package cz.empatix.render.attributes.grouping;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.shaderbindings.ShaderBinding;

import java.util.List;

public record BatchRenderableGroup(
        List<DrawableData> drawableData
) implements RenderableGroup {
    public BatchRenderableGroup(final DrawableBatch batch) {
        this(batch.getDrawables()
                .stream()
                .filter(Drawable::canRender)
                .map(DrawableData::from)
                .toList());
    }

    @Override
    public int getZIndex() {
        return drawableData.get(0).zIndex();
    }

    @Override
    public ShaderBinding getShaderBinding() {
        return drawableData.get(0).renderData().shaderBinding();
    }

    @Override
    public RenderScene getRenderScene() {
        return drawableData.get(0).target();
    }

    @Override
    public List<DrawableData> getDrawableData() {
        return drawableData;
    }

    @Override
    public boolean isRenderable() {
        return !drawableData.isEmpty();
    }

}
