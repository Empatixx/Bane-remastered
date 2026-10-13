package cz.empatix.tiles.domain;

import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;

import java.util.List;

public record CarpetTile(
        List<Vertex> vertices,
        int textureId,
        Vector2f position
) implements Tile {
    private final static int TILE_SIZE = 32;
    private final static int SCALE = 2;
    private final static int SCALED_TILE_SIZE = TILE_SIZE * SCALE;
    private final static int TOTAL_COLUMNS = 4;
    private final static int TOTAL_ROWS = 4;
    @Override
    public RenderData renderData() {
        return new RenderData(
                vertices,
                textureId,
                new CommonLightBinding()
        );    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int getZIndex() {
        return 1;
    }

    @Override
    public boolean canRender() {
        return Camera.getInstance().isInProjection(position, SCALED_TILE_SIZE, SCALED_TILE_SIZE);
    }

    public static CarpetTile from(final int tileNumber,
                                  final int tilePosX,
                                  final int tilePosY,
                                  final int xOff,
                                  final int yOff,
                                  final TextureAtlas.Entry texture) {
        final int tCol = tileNumber % TOTAL_COLUMNS;
        final int tRow = tileNumber / TOTAL_COLUMNS;

        final Vector2f pos = new Vector2f(xOff + tilePosX * TILE_SIZE * SCALE, yOff + tilePosY * TILE_SIZE * SCALE);
        return new CarpetTile(Vertex.builder()
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
                pos);
    }

    @Override
    public void destroy() {
    }
}
