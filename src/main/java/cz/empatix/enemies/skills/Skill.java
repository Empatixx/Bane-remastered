package cz.empatix.enemies.skills;

import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderableState;

public interface Skill {
    void activate();
    void update(final float dt);
    void collectSubRenderables(RenderableState state);
}
