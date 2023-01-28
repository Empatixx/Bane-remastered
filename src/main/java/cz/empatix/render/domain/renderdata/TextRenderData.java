package cz.empatix.render.domain.renderdata;

import cz.empatix.assets.domain.font.Font;
import cz.empatix.render.domain.Shader;

import java.util.function.Consumer;

public record TextRenderData(
        Font font,
        String text,
        Shader shader,
        Consumer<Shader> prepareShaderFunc
) implements RenderData {

    @Override
    public RenderDataType type() {
        return RenderDataType.TEXT;
    }

    @Override
    public int vao() {
        return font.vao();
    }
}
