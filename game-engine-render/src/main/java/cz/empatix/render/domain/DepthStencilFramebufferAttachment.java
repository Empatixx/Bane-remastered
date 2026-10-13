package cz.empatix.render.domain;
import cz.empatix.core.Viewport;

import org.joml.Vector2i;

import static org.lwjgl.opengl.GL30.*;

public class DepthStencilFramebufferAttachment implements FramebufferAttachment{
    private final int depthStencilBuffer;
    public DepthStencilFramebufferAttachment(){
        depthStencilBuffer = glGenRenderbuffers();
    }
    @Override
    public void attach() {
        glBindRenderbuffer(GL_RENDERBUFFER, depthStencilBuffer);
        glRenderbufferStorage(GL_RENDERBUFFER, GL_DEPTH24_STENCIL8,  Viewport.WIDTH, Viewport.HEIGHT);
        glFramebufferRenderbuffer(GL_FRAMEBUFFER, GL_DEPTH_STENCIL_ATTACHMENT, GL_RENDERBUFFER, depthStencilBuffer);
        glBindRenderbuffer(GL_RENDERBUFFER, 0);
    }

    @Override
    public void destroy() {
        glDeleteRenderbuffers(depthStencilBuffer);
    }

    @Override
    public int getFlushingBits() {
        return GL_DEPTH_BUFFER_BIT | GL_STENCIL_BUFFER_BIT;
    }

    @Override
    public void update(Framebuffer framebuffer) {
        final Vector2i size = framebuffer.getSize();
        resize(size.x, size.y);
    }
    private void resize(final int width, final int height){
        glBindRenderbuffer(GL_RENDERBUFFER, depthStencilBuffer);
        glRenderbufferStorage(GL_RENDERBUFFER, GL_DEPTH24_STENCIL8, width, height);
        glBindRenderbuffer(GL_RENDERBUFFER, 0);
    }
}
