package cz.empatix.render.domain;
import cz.empatix.core.Viewport;

import cz.empatix.textureatlas.domain.Texture;
import org.joml.Vector2i;

import static org.lwjgl.opengl.GL30.*;

public class ColorFramebufferAttachment implements FramebufferAttachment {
    private final Texture texture;
    public ColorFramebufferAttachment(final int channels){
        this.texture = new Texture(null, Viewport.WIDTH, Viewport.HEIGHT, channels);
    }

    @Override
    public void attach() {
        texture.bind();
        texture.attachFramebuffer(GL_COLOR_ATTACHMENT0);
        texture.unbind();
    }

    @Override
    public void destroy() {
        texture.destroy();
    }

    @Override
    public int getFlushingBits() {
        return GL_COLOR_BUFFER_BIT;
    }

    public int textureId(){
        return texture.getTextureId();
    }


    @Override
    public void update(Framebuffer framebuffer) {
        final Vector2i size = framebuffer.getSize();
        texture.resize(size.x, size.y);
    }
}
