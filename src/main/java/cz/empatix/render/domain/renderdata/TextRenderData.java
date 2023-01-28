package cz.empatix.render.domain.renderdata;

import cz.empatix.assets.domain.font.Glyph;
import cz.empatix.render.domain.Shader;

import java.util.Map;
import java.util.function.Consumer;

public record TextRenderData(
        int vao,
        Map<Character, Glyph> glyphs,
        String text,
        Shader shader,
        Consumer<Shader> prepareShaderFunc
) implements RenderData {

    @Override
    public RenderDataType type() {
        return RenderDataType.TEXT;
    }
}
