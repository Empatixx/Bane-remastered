package cz.empatix.render.attributes;

import cz.empatix.render.attributes.grouping.BatchRenderableGroup;
import cz.empatix.render.attributes.grouping.RenderableGroup;

import java.util.List;

public interface DrawableBatch extends Renderable {
    List<Drawable> getDrawables();
    @Override
    default RenderableGroup getRenderableGroup() {
        return new BatchRenderableGroup(this);
    }
    @Override
    default List<Renderable> getSubRenderables() {
        return getDrawables().stream()
                .flatMap(drawable -> drawable.getSubRenderables().stream())
                .toList();
    }
}
