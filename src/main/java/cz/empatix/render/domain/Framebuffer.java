package cz.empatix.render.domain;

import cz.empatix.common.handlers.WindowHandler;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;

import static org.lwjgl.opengl.GL30.*;

public record Framebuffer(
        int fbo,
        int colorAttachment,
        int channels,
        Vector2i size
) {
    private final static int RGBA_CHANNELS = 4;
    private final static int RGB_CHANNELS = 3;
    public static final List<Vertex> VERTICES = List.of(
            new Vertex(new Vector2f(0, 0), new Vector2f(0, 1)),
            new Vertex(new Vector2f(WindowHandler.TARGET_WIDTH, 0), new Vector2f(1, 1)),
            new Vertex(new Vector2f(WindowHandler.TARGET_WIDTH, WindowHandler.TARGET_HEIGHT), new Vector2f(1, 0)),
            new Vertex(new Vector2f(0, WindowHandler.TARGET_HEIGHT), new Vector2f(0, 0))
    );
    private final static Logger LOGGER = LoggerFactory.getLogger(Framebuffer.class);

    public void destroy() {
        glDeleteFramebuffers(fbo);
    }

    public static Framebuffer createRgb() {
        return create(RGB_CHANNELS);
    }

    public static Framebuffer createRgba() {
        return create(RGBA_CHANNELS);
    }

    private static Framebuffer create(final int channels) {
        final int fbo = glGenFramebuffers();
        final int textureId = glGenTextures();
        try {
            glBindFramebuffer(GL_FRAMEBUFFER, fbo);
            bindTexBuffer(textureId, channels);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
            glFramebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, textureId, 0);
            if (glCheckFramebufferStatus(GL_FRAMEBUFFER) != GL_FRAMEBUFFER_COMPLETE) {
                LOGGER.error("Framebuffer not complete");
                throw new Exception("Framebuffer not complete");
            }
            glBindFramebuffer(GL_FRAMEBUFFER, 0);
        } catch (Exception e) {
            LOGGER.error("Exception while creatine framebuffer", e);
            glDeleteFramebuffers(fbo);
            glDeleteTextures(textureId);
        }
        return new Framebuffer(fbo, textureId, channels, new Vector2i(WindowHandler.WIDTH, WindowHandler.HEIGHT));
    }

    public void bind() {
        resize();
        glBindFramebuffer(GL_FRAMEBUFFER, fbo);
    }

    public void resize() {
        if (size.x() == WindowHandler.WIDTH && size.y() == WindowHandler.HEIGHT) {
            return;
        }
        size.set(WindowHandler.WIDTH, WindowHandler.HEIGHT);
        bindTexBuffer(colorAttachment, channels);
        glBindTexture(GL_TEXTURE_2D, 0);
    }

    private static void bindTexBuffer(final int colorAttachment, final int channels) {
        glBindTexture(GL_TEXTURE_2D, colorAttachment);
        switch (channels) {
            case RGBA_CHANNELS -> {
                glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, WindowHandler.WIDTH, WindowHandler.HEIGHT, 0, GL_RGBA, GL_UNSIGNED_BYTE, 0);
            }
            case RGB_CHANNELS -> {
                glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, WindowHandler.WIDTH, WindowHandler.HEIGHT, 0, GL_RGB, GL_UNSIGNED_BYTE, 0);
            }
        }
    }

    public void unbind() {
        glBindFramebuffer(GL_FRAMEBUFFER, 0);
    }

    public int getColorAttachment() {
        return colorAttachment;
    }

    public void flush() {
        glBindFramebuffer(GL_FRAMEBUFFER, fbo);
        glClearColor(0.0f, 0.0f, 0.0f, 0.0f);
        glClear(GL_COLOR_BUFFER_BIT);
        glBindFramebuffer(GL_FRAMEBUFFER, 0);
    }
}
