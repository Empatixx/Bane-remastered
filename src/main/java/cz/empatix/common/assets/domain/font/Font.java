package cz.empatix.common.assets.domain.font;

import cz.empatix.common.assets.builders.VaoBuilder;
import cz.empatix.common.assets.builders.VboBuilder;
import org.lwjgl.opengl.GL30;

import java.util.Map;
import java.util.Objects;

public record Font (
        int vao,
        int verticesVbo,
        int textureVbo,
        float scale,
        Map<Character,Glyph> glyphs
){
    public void destroy(){
        GL30.glDeleteVertexArrays(vao);
        GL30.glDeleteBuffers(verticesVbo);
        GL30.glDeleteBuffers(textureVbo);
        glyphs.forEach((c,g) -> g.destroy());
    }

    public static FontBuilder builder(){
        return new FontBuilder();
    }

    public static class FontBuilder{
        private float scale;
        private Map<Character,Glyph> glyphs;

        public FontBuilder scale(final float scale){
            this.scale = scale;
            return this;
        }
        public FontBuilder glyphs(final Map<Character,Glyph> glyphs){
            this.glyphs = glyphs;
            return this;
        }
        public Font build(){
            final int verticesVbo = VboBuilder.builder()
                    .defaultNormalizedBuffer()
                    .dynamic()
                    .build();
            final int textureVbo = VboBuilder.builder()
                    .glDefaultNormalizedBuffer()
                    .build();
            final int vao = VaoBuilder.builder()
                    .addVbo(0, verticesVbo)
                    .addVbo(1, textureVbo)
                    .build();
            return new Font(vao, verticesVbo, textureVbo, scale, glyphs);
        }
    }
    public float getWidth(final String text){
        return (float) text.chars()
                .mapToObj(c -> glyphs.get((char) c))
                .filter(Objects::nonNull)
                .mapToDouble(g -> g.width() + g.bearingX())
                .sum();
    }
}
