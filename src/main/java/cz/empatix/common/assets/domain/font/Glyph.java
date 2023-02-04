package cz.empatix.common.assets.domain.font;

import org.lwjgl.opengl.GL30;

public record Glyph (
        int width,
        int height,
        int bearingX,
        int advance,
        int ascent,
        int descent,
        int lineGap,
        int textureId
){
    public void destroy(){
        GL30.glDeleteTextures(textureId);
    }
    public static GlyphBuilder builder(){
        return new GlyphBuilder();
    }
    public void bindTexture(){
        GL30.glBindTexture(GL30.GL_TEXTURE_2D, textureId);
    }

    public static class GlyphBuilder{
        private int width;
        private int height;
        private int bearingX;
        private int advance;
        private int ascent;
        private int descent;
        private int lineGap;
        private int textureId;

        public GlyphBuilder(){
            textureId = 0;
        }
        public GlyphBuilder width(final int width){
            this.width = width;
            return this;
        }
        public GlyphBuilder height(final int height){
            this.height = height;
            return this;
        }
        public GlyphBuilder bearingX(final int bearingX){
            this.bearingX = bearingX;
            return this;
        }
        public GlyphBuilder advance(final int advance){
            this.advance = advance;
            return this;
        }
        public GlyphBuilder ascent(final int ascent){
            this.ascent = ascent;
            return this;
        }
        public GlyphBuilder descent(final int descent){
            this.descent = descent;
            return this;
        }
        public GlyphBuilder lineGap(final int lineGap){
            this.lineGap = lineGap;
            return this;
        }
        public GlyphBuilder textureId(final int textureId){
            this.textureId = textureId;
            return this;
        }
        public Glyph build(){
            return new Glyph(width, height, bearingX, advance, ascent, descent, lineGap, textureId);
        }
    }
}
