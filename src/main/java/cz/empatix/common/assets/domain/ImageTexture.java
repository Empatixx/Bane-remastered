package cz.empatix.common.assets.domain;

import org.lwjgl.opengl.GL33;

public record ImageTexture(
        int width,
        int height,
        int textureId
){
    public void destroy() {
        GL33.glDeleteTextures(textureId);
    }
}
