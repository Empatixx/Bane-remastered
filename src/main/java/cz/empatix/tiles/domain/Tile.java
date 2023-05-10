package cz.empatix.tiles.domain;

import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.domain.Rectangle;
import cz.empatix.physics.service.IntersectionResolver;
import cz.empatix.physics.service.PhysicsBodyFactory;
import cz.empatix.render.attributes.RenderScene;
import cz.empatix.render.domain.RenderDataType;
import cz.empatix.render.service.Camera;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Vertex;
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
                            final TextureAtlas.Entry texture) {
        final int tCol = tileNumber % TOTAL_COLUMNS;
        final int tRow = tileNumber / TOTAL_COLUMNS;

        final Vector2f pos = new Vector2f(tilePosX * TILE_SIZE * SCALE, tilePosY * TILE_SIZE * SCALE);
        final PhysicsBody collision = tRow > 0 ? PhysicsBodyFactory.createStatic(Rectangle.from(TILE_SIZE * SCALE, TILE_SIZE * SCALE)) : null;
        if(collision != null){
            collision.setTranslation(pos.x(), pos.y());
        }
        return new Tile(Vertex.builder()
                .texPosition(tCol, tRow)
                .texPositionsDimensions(TOTAL_COLUMNS, TOTAL_ROWS)
                .position(new Vector2f(tilePosX * TILE_SIZE * SCALE, tilePosY * TILE_SIZE * SCALE))
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
    public RenderDataType renderType() {
        return RenderDataType.STATIC;
    }

    @Override
    public boolean canRender() {
        return IntersectionResolver.newInstance()
                .addShape(position, new Vector2f(TILE_SIZE * SCALE, TILE_SIZE * SCALE))
                .addShape(Camera.getPosition(), new Vector2f(Camera.getTargetWidth(), Camera.getTargetHeight()))
                .resolve();
    }


    @Override
    public int getZIndex() {
        return 1;
    }
}
