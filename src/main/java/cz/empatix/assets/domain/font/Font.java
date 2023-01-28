package cz.empatix.assets.domain.font;

import cz.empatix.assets.builders.VaoBuilder;
import cz.empatix.assets.builders.VboBuilder;
import org.lwjgl.opengl.GL30;

import java.util.Map;

public record Font (
        int vao,
        int verticesVbo,
        int textureVbo,
        float scale,
        Map<Character,Glyph> glyphs
){
    public Font(final float scale, final Map<Character,Glyph> glyphs){
        this(0,0,0, scale, glyphs);
    }
    public Font(final int vao, final int verticesVbo, final int textureVbo, final float scale, final Map<Character,Glyph> glyphs){
        this.verticesVbo = VboBuilder.builder()
                .defaultNormalizedBuffer()
                .build();
        this.textureVbo = VboBuilder.builder()
                .defaultNormalizedBuffer()
                .build();
        this.vao = VaoBuilder.builder()
                .addVbo(0, this.verticesVbo)
                .addVbo(1, this.textureVbo)
                .build();
        this.scale = scale;
        this.glyphs = glyphs;
    }
    public void destroy(){
        GL30.glDeleteVertexArrays(vao);
        GL30.glDeleteBuffers(verticesVbo);
        GL30.glDeleteBuffers(textureVbo);
        glyphs.forEach((c,g) -> g.destroy());
    }
}
