package cz.empatix.tiles.domain;

import cz.empatix.render.Camera;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.renderdata.CommonRenderData;
import cz.empatix.render.domain.renderdata.RenderData;
import org.joml.Vector2f;

import java.util.function.Consumer;

public record Tile(
        int vao,
        int textureId,
        Shader Shader,
        int textureCol,
        int textureRow,
        Shader shader,
        Consumer<Shader> prepareShaderFunc,
        Vector2f position
) implements Drawable {

    public Tile {
        if (textureCol < 0 || textureRow < 0) {
            throw new IllegalArgumentException("Texture coordinates must be positive");
        }
    }

    public Tile(final int vao,
                final int textureId,
                final Shader shader,
                final int textureCol,
                final int textureRow,
                final int totalColumns,
                final int totalRows
                ) {
        this(vao, textureId, shader, textureCol, textureRow, totalColumns, totalRows, new Vector2f());
    }

    public Tile(final int vao,
                final int textureId,
                final Shader shader,
                final int textureCol,
                final int textureRow,
                final int totalColumns,
                final int totalRows,
                final Vector2f position) {
        this(vao, textureId, shader, textureCol, textureRow, shader,
                shader1 -> prepareShader(shader1, totalColumns, totalRows, textureCol, textureRow, position), position);
    }

    private static void prepareShader(final Shader shader,
                                      final int totalColumns,
                                      final int totalRows,
                                      final int textureCol,
                                      final int textureRow,
                                      final Vector2f position) {
        shader.setUniform("sampler", 0);
        shader.setUniform("totalColumns", totalColumns);
        shader.setUniform("totalRows", totalRows);
        shader.setUniform("currentColumn", textureCol);
        shader.setUniform("currentRow", textureRow);
        shader.setUniform("projection", Camera.getProjection().translate(position.x,position.y,0));
    }

    @Override
    public RenderData renderData() {
        return new CommonRenderData(
                vao,
                textureId,
                shader,
                prepareShaderFunc
        );
    }

    @Override
    public boolean shouldRender() {
        return position.x + 64 > Camera.getX() && position.x < Camera.getTargetWidth() + Camera.getX() &&
                position.y + 64 > Camera.getY() && position.y < Camera.getTargetHeight() + Camera.getY();
    }


    @Override
    public int getZIndex() {
        return 1;
    }
}
