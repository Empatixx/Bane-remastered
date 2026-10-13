package cz.empatix.shadow.domain;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.shaderbindings.ShadowBinding;

import java.util.List;

public class PlayerShadow implements Drawable {
    private static final int Z_INDEX = 3;
    private static final ShadowBinding SHADOW_BINDING = new ShadowBinding();

    private final List<Vertex> vertices;

    public PlayerShadow(final List<Vertex> vertices) {
        this.vertices = vertices;
    }

    @Override
    public RenderData renderData() {
        return new RenderData(vertices, 0, SHADOW_BINDING);
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int getZIndex() {
        return Z_INDEX;
    }

    @Override
    public boolean canRender() {
        return !vertices.isEmpty();
    }
}
