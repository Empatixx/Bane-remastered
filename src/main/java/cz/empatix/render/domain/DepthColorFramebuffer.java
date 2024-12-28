package cz.empatix.render.domain;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.render.service.*;
import org.joml.Vector2i;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import static org.lwjgl.opengl.GL11.GL_LINEAR;
import static org.lwjgl.opengl.GL11.GL_RGB;
import static org.lwjgl.opengl.GL11.GL_RGBA;
import static org.lwjgl.opengl.GL11.GL_TEXTURE_2D;
import static org.lwjgl.opengl.GL11.GL_TEXTURE_MAG_FILTER;
import static org.lwjgl.opengl.GL11.GL_TEXTURE_MIN_FILTER;
import static org.lwjgl.opengl.GL11.GL_UNSIGNED_BYTE;
import static org.lwjgl.opengl.GL11.glBindTexture;
import static org.lwjgl.opengl.GL11.glDeleteTextures;
import static org.lwjgl.opengl.GL11.glGenTextures;
import static org.lwjgl.opengl.GL11.glTexImage2D;
import static org.lwjgl.opengl.GL11.glTexParameteri;
import static org.lwjgl.opengl.GL30.*;
import static org.lwjgl.opengl.GL30.GL_FRAMEBUFFER;

public class DepthColorFramebuffer implements Framebuffer {
    private static final Logger LOGGER = LoggerFactory.getLogger(ColorFramebuffer.class);
    private final int fbo;
    private final int textureColorAttachment;
    private final int rboDepthStencilAttachment;
    private final int channels;
    private final Vector2i size;

    public DepthColorFramebuffer(final int channels){
        this.channels = channels;
        size = new Vector2i(WindowHandler.WIDTH, WindowHandler.HEIGHT);
        fbo = glGenFramebuffers();
        textureColorAttachment = glGenTextures();
        rboDepthStencilAttachment = glGenRenderbuffers();
        try {
            initFramebuffer();
        } catch (Exception e) {
            LOGGER.error("Exception while creatine framebuffer", e);
            glDeleteFramebuffers(fbo);
            glDeleteTextures(textureColorAttachment);
        }
    }
    private void initFramebuffer() throws Exception {
        glBindFramebuffer(GL_FRAMEBUFFER, fbo);
        glBindTexture(GL_TEXTURE_2D, textureColorAttachment);
        switch (channels) {
            case RGBA_CHANNELS -> glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, WindowHandler.WIDTH, WindowHandler.HEIGHT, 0, GL_RGBA, GL_UNSIGNED_BYTE, 0);
            case RGB_CHANNELS -> glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, WindowHandler.WIDTH, WindowHandler.HEIGHT, 0, GL_RGB, GL_UNSIGNED_BYTE, 0);
        }
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
        glFramebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, textureColorAttachment, 0);
        if (glCheckFramebufferStatus(GL_FRAMEBUFFER) != GL_FRAMEBUFFER_COMPLETE) {
            LOGGER.error("Framebuffer not complete");
            throw new Exception("Framebuffer not complete");
        }
        glBindRenderbuffer(GL_RENDERBUFFER, rboDepthStencilAttachment);
        glRenderbufferStorage(GL_RENDERBUFFER, GL_DEPTH24_STENCIL8, WindowHandler.WIDTH, WindowHandler.HEIGHT);
        glFramebufferRenderbuffer(GL_FRAMEBUFFER, GL_DEPTH_STENCIL_ATTACHMENT, GL_RENDERBUFFER, rboDepthStencilAttachment);

        glBindFramebuffer(GL_FRAMEBUFFER, 0);
    }


    @Override
    public void destroy() {
        glDeleteFramebuffers(fbo);
    }

    @Override
    public void bind() {
        glBindFramebuffer(GL_FRAMEBUFFER, fbo);
    }

    @Override
    public void unbind() {
        glBindFramebuffer(GL_FRAMEBUFFER, 0);
    }

    @Override
    public void flush() {
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    }

    @Override
    public int colorAttachment() {
        return textureColorAttachment;
    }

    @Override
    public void update(ObserveSubject subject, ObservePayload payload) {
        if (subject instanceof Camera camera){
            if (payload instanceof CameraObservePayload cameraPayload){
                if (cameraPayload.change().equals(CameraChangeType.VIEWPORT)){
                    resize(WindowHandler.WIDTH, WindowHandler.HEIGHT);
                }
            }
        }
    }
    private void resize(final int width, final int height) {
        if (size.x() == width && size.y() == height) {
            return;
        }
        size.set(width, height);
        glBindTexture(GL_TEXTURE_2D, textureColorAttachment);
        switch (channels) {
            case RGBA_CHANNELS -> glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, GL_RGBA, GL_UNSIGNED_BYTE, 0);
            case RGB_CHANNELS -> glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, width, height, 0, GL_RGB, GL_UNSIGNED_BYTE, 0);
        }
        glBindTexture(GL_TEXTURE_2D, 0);

        glBindFramebuffer(GL_FRAMEBUFFER, fbo);
        glBindRenderbuffer(GL_RENDERBUFFER, rboDepthStencilAttachment);
        glRenderbufferStorage(GL_RENDERBUFFER, GL_DEPTH24_STENCIL8, width, height);
        glBindRenderbuffer(GL_RENDERBUFFER, 0);
        glBindFramebuffer(GL_FRAMEBUFFER, 0);
    }
}
