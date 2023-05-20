package cz.empatix.tiles.domain;

import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.domain.Rectangle;
import cz.empatix.physics.service.PhysicsBodyFactory;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;

import java.util.List;

public record Tile(
        List<Vertex> vertices,
        int textureId,
        Vector2f position,
        PhysicsBody collision
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
        final PhysicsBody collision = tRow > 0 ? PhysicsBodyFactory.createStatic(Rectangle.from(TILE_SIZE * SCALE, TILE_SIZE * SCALE)) : null;
        if (collision != null) {
            collision.setTranslation(pos.x(), pos.y());
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
        final Vector2f cameraPos = Camera.getPosition();
        final Vector2f cameraDimensions = new Vector2f(Camera.getTargetWidth(), Camera.getTargetHeight());

        return position.x() + TILE_SIZE * SCALE > cameraPos.x()
                && position.x() < cameraPos.x() + cameraDimensions.x()
                && position.y() + TILE_SIZE * SCALE > cameraPos.y()
                && position.y() < cameraPos.y() + cameraDimensions.y();
    }


    @Override
    public int getZIndex() {
        return 1;
    }
}
