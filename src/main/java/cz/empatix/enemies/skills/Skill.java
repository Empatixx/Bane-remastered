package cz.empatix.enemies.skills;

import cz.empatix.render.attributes.Renderable;

import java.util.List;

public interface Skill {
    void activate();
    void update(final float dt);
    default void fixedUpdate(final float dt){

    }
    List<Renderable> getSubRenderables();
}
