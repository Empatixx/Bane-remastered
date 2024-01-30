package cz.empatix.artefacts.domain;

import cz.empatix.render.attributes.Renderable;

import java.util.List;

public interface Artefact {
    void activate();
    void update(final double dt);
    void fixedUpdate(final double dt);
    List<Renderable> getRenderables();
    List<Renderable> hudToRender();
}
