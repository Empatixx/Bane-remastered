package cz.empatix.tiles.domain;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.renderdata.CommonRenderData;
import cz.empatix.render.domain.renderdata.RenderData;
import org.joml.Vector2f;

import java.util.function.Consumer;

public record Tile(
        int vao,
        int textureId,
        int Shader,
        int textureCol,
        int textureRow,
        Shader shader,
        Consumer<Shader> prepareShaderFunc,
        Vector2f position
) implements Drawable {
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
        return true;
    }


    @Override
    public int getZIndex() {
        return 1;
    }
}
