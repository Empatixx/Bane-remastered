package cz.empatix.tiles.domain;

import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.dyn4j.geometry.Geometry;
import org.dyn4j.geometry.MassType;
import org.joml.Vector2f;
import org.w3c.dom.css.Rect;

import java.util.List;

public record Tile(
        List<Vertex> vertices,
        int textureId,
        Vector2f position,
        PhysicsObject collision
) implements Drawable {
    public final static int TILE_SIZE = 32;
    private final static int TOTAL_ROWS = 2;
    private final static int TOTAL_COLUMNS = 21;
    public final static int SCALE = 2;

    public static Tile from(final int tileNumber,
                            final int tilePosX,
                            final int tilePosY,
                            final int xOff,
                            final int yOff,
                            final TextureAtlas.Entry texture) {
        final int tCol = tileNumber % TOTAL_COLUMNS;
        final int tRow = tileNumber / TOTAL_COLUMNS;

        final Vector2f pos = new Vector2f(xOff + tilePosX * TILE_SIZE * SCALE, yOff + tilePosY * TILE_SIZE * SCALE);
        final PhysicsObject collision = tRow > 0 ? PhysicsObjectFactory.create(Geometry.createRectangle(TILE_SIZE * SCALE, TILE_SIZE * SCALE)) : null;
        if (collision != null) {
            collision.setTranslation(pos.x + TILE_SIZE * SCALE/2f, pos.y + TILE_SIZE * SCALE/2f);
            collision.setMass(MassType.INFINITE);
        }
        return new Tile(Vertex.builder()
                .texPosition(tCol, tRow)
                .texPositionsDimensions(TOTAL_COLUMNS, TOTAL_ROWS)
                .position(pos)
                .posDimensions(TILE_SIZE, TILE_SIZE)
                .offset(TILE_SIZE, TILE_SIZE)
                .texture(texture)
                .scale(SCALE)
                .buildQuad(),
                texture.textureId(),
                pos,
                collision);
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                vertices,
                textureId,
                new CommonBinding(Camera.getProjection())
        );
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }


    @Override
    public boolean canRender() {
        return Camera.isInProjection(position, TILE_SIZE * SCALE, TILE_SIZE * SCALE);
    }

    public void destroy() {
        if (collision != null) {
            collision.destroy();
        }
    }


    @Override
    public int getZIndex() {
        return 1;
    }
}
