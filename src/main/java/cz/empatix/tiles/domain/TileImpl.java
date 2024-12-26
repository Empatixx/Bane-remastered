package cz.empatix.tiles.domain;

import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.dyn4j.geometry.Geometry;
import org.dyn4j.geometry.MassType;
import org.joml.Vector2f;

import java.util.List;

public record TileImpl(
        List<Vertex> vertices,
        int textureId,
        Vector2f position,
        PhysicsObject collision
) implements Tile {
    public TileImpl {
        if (collision != null) {
            collision.setUserData(this);
        }
    }
    public final static int TILE_SIZE = 32;
    private final static int TOTAL_ROWS = 2;
    private final static int TOTAL_COLUMNS = 21;
    public final static int SCALE = 2;
    public final static int SCALE_TILE_SIZE = TILE_SIZE * SCALE;

    public static TileImpl from(final int tileNumber,
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
            collision.setType(CollisionType.TILE);
        }
        return new TileImpl(Vertex.builder()
                .zDepth(1)
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
        return Camera.isInProjection(position, SCALE_TILE_SIZE, SCALE_TILE_SIZE);
    }
    @Override
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
