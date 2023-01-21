package cz.empatix.render.domain;

import java.util.function.Consumer;

public record RenderData (
        int verticesVbo,
        int textureVbo,
        int textureId,
        Shader shader,
        Consumer<Shader> prepareShaderFunc
        ){
}
