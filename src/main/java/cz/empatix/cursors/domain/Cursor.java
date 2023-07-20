package cz.empatix.cursors.domain;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderScene;
import org.joml.Vector2f;

public interface Cursor extends Drawable {
    @Override
    default RenderScene renderScene() {
        return RenderScene.OVERLAYS;
    }

    @Override
    default int getZIndex() {
        return 200;
    }

    void updateMouse(final float x, final float y, final float dt);
    CursorType getType();
}
