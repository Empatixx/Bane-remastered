package cz.empatix.tiles.domain;

import cz.empatix.common.assets.domain.TextureImage;
import cz.empatix.physics.IntersectionResolver;
import cz.empatix.render.Camera;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.shaderbindings.CommonShaderBinding;
import cz.empatix.render.domain.Vertex;
import org.joml.Vector2f;

import java.util.List;

public record Tile(
        List<Vertex> vertices,
        int textureId,
        Vector2f position
) implements Drawable {
    private final static int TILE_SIZE = 32;
    private final static int TOTAL_ROWS = 2;
    private final static int TOTAL_COLUMNS = 21;

    public Tile(final int tCol,
                final int tRow,
                final int tilePosX,
                final int tilePosY,
                final TextureImage texture) {
        this(tCol, tRow, new Vector2f(tilePosX * TILE_SIZE, tilePosY * TILE_SIZE), texture);
    }

    public Tile(final int tCol,
                final int tRow,
                final Vector2f position,
                final TextureImage texture) {
        this(Vertex.builder()
                        .position(new Vector2f(position))
                        .posDimensions(TILE_SIZE, TILE_SIZE)
                        .textureDimensions(TOTAL_COLUMNS, TOTAL_ROWS)
                        .texturePosition(tCol, tRow)
                        .offsetTexture(texture.width(), texture.height())
                        .buildQuad(),
                texture.textureId(),
                position);
    }
    @Override
    public RenderData renderData() {
        return new RenderData(
                vertices,
                textureId,
                new CommonShaderBinding(Camera.getProjection()),
                RenderData.RenderDataType.DYNAMIC
        );
    }

    @Override
    public boolean shouldRender() {
        return IntersectionResolver.newInstance()
                .addShape(position, new Vector2f(TILE_SIZE, TILE_SIZE))
                .addShape(Camera.getPosition(), new Vector2f(Camera.getTargetWidth(), Camera.getTargetHeight()))
                .resolve();
    }


    @Override
    public int getZIndex() {
        return 1;
    }
}
