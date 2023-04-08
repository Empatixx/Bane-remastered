package cz.empatix.physics.domain.transform;

import org.joml.Vector2f;

public class CommonTransform implements Transform{
    private final Vector2f translation = new Vector2f();
    @Override
    public void translate(final float x, final float y) {
        translation.add(x,y);
    }

    @Override
    public void setTranslation(final float x, final float y) {
        translation.set(x,y);
    }

    @Override
    public Vector2f getTranslation() {
        return translation;
    }

    @Override
    public Vector2f getTranslationToRender() {
        return translation;
    }

    @Override
    public void update(float dt) {
    }
}
