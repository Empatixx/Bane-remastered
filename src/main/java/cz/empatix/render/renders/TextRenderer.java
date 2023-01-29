package cz.empatix.render.renders;

import cz.empatix.assets.domain.font.Glyph;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.renderdata.RenderData;
import cz.empatix.render.domain.renderdata.TextRenderData;
import org.lwjgl.opengl.GL30;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.function.Consumer;

import static org.lwjgl.opengl.GL30.*;

public class TextRenderer implements Renderer {
    private final static Logger LOGGER = LoggerFactory.getLogger(TextRenderer.class);
    @Override
    public boolean accepts(final RenderData renderData) {
        return renderData.type() == RenderData.RenderDataType.TEXT;
    }

    @Override
    public void render(final RenderData renderData) {
        final TextRenderData textRenderData = (TextRenderData) renderData;

        final int vao = textRenderData.vao();
        final Map<Character, Glyph> glyphs = textRenderData.font().glyphs();
        final String text = textRenderData.text();
        final Shader shader = textRenderData.shader();
        final Consumer<Shader> prepareShaderFunc = textRenderData.prepareShaderFunc();

        glActiveTexture(GL_TEXTURE0);
        shader.bind();
        glBindVertexArray(vao);

        prepareShaderFunc.accept(shader);

        final List<Glyph> glyphsToRender = text.chars()
                .mapToObj(c -> {
                    final Glyph glyph = glyphs.get((char) c);
                    if(glyph == null){
                        LOGGER.error("Glyph for char '{}' not found",(char) c);
                        throw new RuntimeException("Glyph for char '"+(char) c+"' not found");
                    }
                    return glyph;
                })
                .filter(Objects::nonNull)
                .toList();
        int offset = 0;
        for(final Glyph glyph : glyphsToRender){
            glyph.bindTexture();
            final float[] vertices = {
                    offset+glyph.bearingX(), glyph.descent(),
                    offset+glyph.width()+glyph.bearingX(), glyph.descent(),
                    offset+glyph.width()+glyph.bearingX(), -glyph.ascent(),
                    offset+glyph.bearingX(), -glyph.ascent()
            };
            offset += glyph.advance();
            glBindBuffer(GL30.GL_ARRAY_BUFFER, textRenderData.font().verticesVbo());
            glBufferSubData(GL30.GL_ARRAY_BUFFER, 0, vertices);
            glBindBuffer(GL30.GL_ARRAY_BUFFER, 0);

            glDrawArrays(GL_QUADS, 0, 4);
        }

        glBindVertexArray(0);
        shader.unbind();
    }
}
