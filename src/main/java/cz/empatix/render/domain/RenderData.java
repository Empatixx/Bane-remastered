package cz.empatix.render.domain;

import java.util.function.Consumer;

public record RenderData (
        int vao,
        int textureId,
        Shader shader,
        Consumer<Shader> prepareShaderFunc
        ){
}
