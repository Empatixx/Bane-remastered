package cz.empatix.render.attributes;

import cz.empatix.render.attributes.grouping.BatchRenderableGroup;
import cz.empatix.render.attributes.grouping.RenderableGroup;
import cz.empatix.render.domain.RenderableState;

import java.util.List;

public interface DrawableBatch extends Renderable {
    List<Drawable> getDrawables();
    @Override
    default RenderableGroup getRenderableGroup() {
        return new BatchRenderableGroup(this);
    }
    @Override
    default void collectSubRenderables(RenderableState state) {
        for (Drawable drawable : getDrawables()) {
            drawable.collectSubRenderables(state);
        }
    }
}
