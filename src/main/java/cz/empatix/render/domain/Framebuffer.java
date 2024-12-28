package cz.empatix.render.domain;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.render.service.Observer;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.util.List;

public interface Framebuffer extends Observer {
    int RGBA_CHANNELS = 4;
    int RGB_CHANNELS = 3;
    void destroy();
    void bind();
    void unbind();
    void flush();
    int colorAttachment();
    List<Vertex> VERTICES = List.of(
            new Vertex(new Vector3f(0, 0,100), new Vector2f(0, 1)),
            new Vertex(new Vector3f(WindowHandler.TARGET_WIDTH, 0,100), new Vector2f(1, 1)),
            new Vertex(new Vector3f(WindowHandler.TARGET_WIDTH, WindowHandler.TARGET_HEIGHT,100), new Vector2f(1, 0)),
            new Vertex(new Vector3f(0, WindowHandler.TARGET_HEIGHT,100), new Vector2f(0, 0))
    );
}
