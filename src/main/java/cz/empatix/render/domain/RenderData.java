package cz.empatix.render.domain;

import java.util.List;
import java.util.function.Consumer;

public record RenderData (
    List<Vertex> vertices,
    int textureId,
    Shader shader,
    Consumer<Shader> prepareShaderFunc,
    RenderDataType type
){
    public enum RenderDataType{
        STATIC,
        TEXT,
        DYNAMIC
    }
}
