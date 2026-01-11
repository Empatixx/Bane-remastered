package cz.empatix.render.attributes;

import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.render.attributes.grouping.RenderableGroup;
import cz.empatix.render.attributes.grouping.SingleRenderableGroup;

public interface Drawable extends Renderable
{
    RenderData renderData();
    RenderScene renderScene();
    int getZIndex();
    boolean canRender();
    @Override
    default RenderableGroup getRenderableGroup() {
        return new SingleRenderableGroup(this);
    }
    @Override
    default void collectSubRenderables(RenderableState state) {
        // Empty by default - no sub-renderables
    }
}
