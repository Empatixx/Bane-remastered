package cz.empatix.enemies.skills;

import cz.empatix.render.domain.RenderableState;

public interface Skill {
    void activate();
    void update(final float dt);
    default void fixedUpdate(final float dt){

    }
    void collectSubRenderables(RenderableState state);
}
