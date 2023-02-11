package cz.empatix.tiles.domain;

import cz.empatix.common.assets.domain.TextureImage;
import cz.empatix.render.Camera;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.Vertex;
import org.joml.Vector2f;

import java.util.List;
import java.util.function.Consumer;

public record Tile(
        List<Vertex> vertices,
        int textureId,
        Shader shader,
        Consumer<Shader> prepareShaderFunc,
        Vector2f position
) implements Drawable {
    private final static int TILE_SIZE = 32;
    private final static int TOTAL_ROWS = 2;
    private final static int TOTAL_COLUMNS = 21;

    public Tile(final int tCol,
                final int tRow,
                final Shader shader,
                final int tilePosX,
                final int tilePosY,
                final TextureImage texture) {
        this(tCol, tRow, shader, new Vector2f(tilePosX * TILE_SIZE, tilePosY * TILE_SIZE), texture);
    }

    public Tile(final int tCol,
                final int tRow,
                final Shader shader,
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
                shader,
                Tile::prepareShader,
                position);
    }

    private static void prepareShader(final Shader shader) {
        shader.setUniform4x4f("projection", Camera.getProjection());
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                vertices,
                textureId,
                shader,
                prepareShaderFunc,
                RenderData.RenderDataType.DYNAMIC
        );
    }

    @Override
    public boolean shouldRender() {
        return position.x + TILE_SIZE > Camera.getX() && position.x < Camera.getTargetWidth() + Camera.getX() &&
                position.y + TILE_SIZE > Camera.getY() && position.y < Camera.getTargetHeight() + Camera.getY();
    }


    @Override
    public int getZIndex() {
        return 1;
    }
}
