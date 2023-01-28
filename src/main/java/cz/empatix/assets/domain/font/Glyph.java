package cz.empatix.assets.domain.font;

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
}
