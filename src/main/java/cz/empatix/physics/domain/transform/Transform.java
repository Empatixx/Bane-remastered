package cz.empatix.physics.domain.transform;

import org.joml.Vector2f;

public interface Transform {
    void translate(final float x, final float y);
    void setTranslation(final float x, final float y);
    Vector2f getTranslation();
    Vector2f getTranslationToRender();
    void update(final float dt);
}
