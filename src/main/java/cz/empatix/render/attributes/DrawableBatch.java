package cz.empatix.render.attributes;

import cz.empatix.render.domain.grouping.BatchRenderableGroup;
import cz.empatix.render.domain.grouping.RenderableGroup;
import cz.empatix.render.shaderbindings.ShaderBinding;

import java.util.List;

public interface DrawableBatch extends Renderable {
    List<Drawable> getDrawables();
    @Override
    default RenderableGroup getRenderableGroup() {
        return new BatchRenderableGroup(this);
    }
    @Override
    default List<Renderable> getSubRenderables() {
        return List.of();
    }
}
