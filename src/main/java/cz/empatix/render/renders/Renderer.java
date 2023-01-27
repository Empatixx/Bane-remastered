package cz.empatix.render.renders;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;

public interface Renderer {
    boolean accepts(Drawable drawable);
    void render(Drawable drawable);
}
