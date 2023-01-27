package cz.empatix.render.renders;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.hud.Text;

public class TextRenderer implements Renderer{
    @Override
    public boolean accepts(final Drawable drawable) {
        return drawable instanceof Text;
    }

    @Override
    public void render(final Drawable drawable) {
        return;
    }
}
