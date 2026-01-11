package cz.empatix.hud.service.text;

import cz.empatix.hud.service.HUDElement;
import cz.empatix.render.attributes.grouping.ContainerRenderableGroup;
import cz.empatix.render.attributes.grouping.RenderableGroup;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderableState;
import org.joml.Vector2f;


public class MovableAlternativeViewText implements HUDElement {
    private final AlternativeViewText text;
    private final Vector2f direction;
    public MovableAlternativeViewText(final AlternativeViewText text, final Vector2f direction) {
        this.text = text;
        this.direction = direction;
    }
    public void update(final double dt) {
        text.update(dt);
        text.setPosition(text.getPosition().add((float) (direction.x() * dt), (float) (direction.y() * dt)));
    }

    @Override
    public RenderData renderData() {
        return text.renderData();
    }

    @Override
    public int getZIndex() {
        return text.getZIndex();
    }

    @Override
    public boolean canRender() {
        return text.canRender();
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        text.collectSubRenderables(state);
    }

    @Override
    public RenderableGroup getRenderableGroup() {
        return ContainerRenderableGroup.INSTANCE;
    }
}
