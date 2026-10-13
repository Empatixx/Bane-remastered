package cz.empatix.shadow.domain;

import cz.empatix.player.Player;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.shaderbindings.ShadowTextureBinding;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.util.ArrayList;
import java.util.List;

public class PlayerTextureShadow implements Drawable {
    private static final int Z_INDEX = 3;

    private final Player player;
    private final List<Vertex> relativeVertices;  // Relative to player feet (0, 0)
    private final int textureId;
    private final float intensity;

    public PlayerTextureShadow(final Player player, final List<Vertex> relativeVertices,
                               final int textureId, final float intensity) {
        this.player = player;
        this.relativeVertices = relativeVertices;
        this.textureId = textureId;
        this.intensity = intensity;
    }

    @Override
    public RenderData renderData() {
        // Get CURRENT player feet position
        final Vector2f pos = player.getRenderPosition();
        final float feetX = pos.x() + player.getWidth() / 2f;
        final float feetY = pos.y() + player.getHeight();

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
        return !relativeVertices.isEmpty();
    }
}
