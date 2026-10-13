package cz.empatix.shadow.domain;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.shaderbindings.ShadowTextureBinding;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.util.ArrayList;
import java.util.List;

/**
 * Shadow renderable for generic objects (enemies, room objects, items, pets).
 */
public class ObjectShadow implements Drawable {
    private static final int Z_INDEX = 2;  // Below most objects

    private final ShadowCaster caster;
    private final List<Vertex> relativeVertices;
    private final int textureId;
    private final float intensity;

    public ObjectShadow(final ShadowCaster caster, final List<Vertex> relativeVertices,
                        final int textureId, final float intensity) {
        this.caster = caster;
        this.relativeVertices = relativeVertices;
        this.textureId = textureId;
        this.intensity = intensity;
    }

    @Override
    public RenderData renderData() {
        // Get current object feet position
        final Vector2f pos = caster.getShadowPosition();
        final float feetX = pos.x() + caster.getShadowWidth() / 2f;
        final float feetY = pos.y() + caster.getShadowHeight() + caster.getShadowFeetOffsetY();

        // Translate relative vertices to current position
        final List<Vertex> absoluteVertices = new ArrayList<>();
        for (final Vertex v : relativeVertices) {
            absoluteVertices.add(new Vertex(
                    new Vector3f(v.position().x() + feetX, v.position().y() + feetY, v.position().z()),
                    v.texCoords()
            ));
        }

        return new RenderData(absoluteVertices, textureId, new ShadowTextureBinding(intensity));
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
        return !relativeVertices.isEmpty() && caster.canRender();
    }
}
