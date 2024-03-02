package cz.empatix.render.attributes;

import cz.empatix.render.attributes.grouping.BatchRenderableGroup;
import cz.empatix.render.attributes.grouping.RenderableGroup;

import java.util.ArrayList;
import java.util.List;

public interface DrawableBatch extends Renderable {
    List<Drawable> getDrawables();
    @Override
    default RenderableGroup getRenderableGroup() {
        return new BatchRenderableGroup(this);
    }
    @Override
    default List<Renderable> getSubRenderables() {
        final List<Renderable> allSubRenderables = new ArrayList<>();
        for (Drawable drawable : getDrawables()) {
            List<Renderable> subRenderables = drawable.getSubRenderables();
            if (!subRenderables.isEmpty()) {
                allSubRenderables.addAll(subRenderables);
            }
        }
        return allSubRenderables;
    }
}
