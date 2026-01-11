package cz.empatix.render.domain;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.render.service.*;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.joml.Vector3f;

import java.util.ArrayList;
import java.util.List;

import static org.lwjgl.opengl.GL30.*;

public class Framebuffer implements Observer {
    public static final List<Vertex> VERTICES = List.of(
            new Vertex(new Vector3f(0, 0,100), new Vector2f(0, 1)),
            new Vertex(new Vector3f(WindowHandler.TARGET_WIDTH, 0,100), new Vector2f(1, 1)),
            new Vertex(new Vector3f(WindowHandler.TARGET_WIDTH, WindowHandler.TARGET_HEIGHT,100), new Vector2f(1, 0)),
            new Vertex(new Vector3f(0, WindowHandler.TARGET_HEIGHT,100), new Vector2f(0, 0))
    );
    private final int fboId;
    private final Vector2i size;
    private final List<FramebufferAttachment> attachments;
    private ColorFramebufferAttachment colorAttachment; //temp solution todo; remove

    public Framebuffer(){
        fboId = glGenFramebuffers();
        size = new Vector2i(WindowHandler.WIDTH, WindowHandler.HEIGHT);
        attachments = new ArrayList<>();
    }

    public void attach(final FramebufferAttachment attachment){
        bind();
        attachment.attach();
        attachments.add(attachment);
        if (attachment instanceof ColorFramebufferAttachment colorAttachment){
            this.colorAttachment = colorAttachment;
        }
        unbind();
    }

    public void bind(){
        glBindFramebuffer(GL_FRAMEBUFFER, fboId);
    }

    public void unbind(){
        glBindFramebuffer(GL_FRAMEBUFFER, 0);
    }

    public Vector2i getSize(){
        return size;
    }

    public void destroy(){
        glDeleteFramebuffers(fboId);
        for (FramebufferAttachment attachment : attachments){
            attachment.destroy();
        }
    }


    public int colorAttachment(){
        return colorAttachment.textureId();
    }


    public void flush(){
        bind();
        clear();
        unbind();
    }

    public void clear(){
        glClearColor(0.0f, 0.0f, 0.0f, 0.0f);
        int flushingBits = 0;
        for (FramebufferAttachment attachment : attachments){
            flushingBits |= attachment.getFlushingBits();
        }
        glClear(flushingBits);
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
    private void resize(final int width, final int height){
        if (width <= 0 || height <= 0){
            return;
        }
        if (size.x != width || size.y != height){
            size.set(width, height);
            bind();
            attachments.forEach(attachment -> attachment.update(this));
            unbind();
        }
    }
}
