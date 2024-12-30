package cz.empatix.render.domain;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.textureatlas.domain.TextureArray;
import org.joml.Vector2i;

import static org.lwjgl.opengl.GL30.*;

public class TextureArrayFramebufferAttachment implements FramebufferAttachment{
    private final TextureArray textureArray;
    public TextureArrayFramebufferAttachment(final int channels, final int layers){
        this.textureArray = new TextureArray(WindowHandler.WIDTH, WindowHandler.HEIGHT, layers, channels);
    }
    @Override
    public void attach() {
        textureArray.bind();
        textureArray.attachFramebuffer(GL_COLOR_ATTACHMENT0, 0);
        textureArray.unbind();
    }

    @Override
    public void destroy() {
        textureArray.destroy();
    }

    @Override
    public int getFlushingBits() {
        return GL_COLOR_BUFFER_BIT;
    }

    @Override
    public void update(Framebuffer framebuffer) {
        final Vector2i size = framebuffer.getSize();
        textureArray.resize(size.x, size.y);
    }
}
