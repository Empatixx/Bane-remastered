package cz.empatix.assets.domain;

import org.lwjgl.opengl.GL33;

public record TextureImage(
        int width,
        int height,
        int textureId
){
    public void destroy() {
        GL33.glDeleteTextures(textureId);
    }
}
