package cz.empatix.render.renders;

import cz.empatix.assets.domain.font.Glyph;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.renderdata.RenderData;
import cz.empatix.render.domain.renderdata.TextRenderData;

import java.util.Map;
import java.util.function.Consumer;

import static org.lwjgl.opengl.GL30.*;

public class TextRenderer implements Renderer{
    @Override
    public boolean accepts(final RenderData renderData) {
        return renderData.type() == RenderData.RenderDataType.TEXT;
    }

    @Override
    public void render(final RenderData renderData) {
        final TextRenderData textRenderData = (TextRenderData) renderData;

        final int vao = textRenderData.vao();
        final Map<Character, Glyph> glyphs = textRenderData.glyphs();
        final String text = textRenderData.text();
        final Shader shader = textRenderData.shader();
        final Consumer<Shader> prepareShaderFunc = textRenderData.prepareShaderFunc();

        shader.bind();
        prepareShaderFunc.accept(shader);

        glBindVertexArray(vao);
        glDrawArrays(GL_QUADS, 0, 4);
        glBindVertexArray(0);

        shader.unbind();
    }
}
