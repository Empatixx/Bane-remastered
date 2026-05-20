package cz.empatix.shadow.domain;

import cz.empatix.render.domain.RenderData;
import org.joml.Vector2f;

/**
 * Interface for objects that can cast shadows from light sources.
 */
public interface ShadowCaster {
    Vector2f getShadowPosition();
    int getShadowWidth();
    int getShadowHeight();
    RenderData renderData();
    boolean canRender();

    /**
     * Offset from sprite bottom to actual feet position.
     */
    default float getShadowFeetOffsetY() {
        return 0f;
    }
}
