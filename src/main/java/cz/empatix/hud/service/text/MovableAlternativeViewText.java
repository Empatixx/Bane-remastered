package cz.empatix.hud.service.text;

import cz.empatix.hud.service.HUDElement;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderData;
import org.joml.Vector2f;

import java.util.List;

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
    public List<Renderable> getSubRenderables() {
        return text.getSubRenderables();
    }
}
