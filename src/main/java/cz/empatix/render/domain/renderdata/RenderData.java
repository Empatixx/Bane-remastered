package cz.empatix.render.domain.renderdata;

import cz.empatix.render.domain.Shader;

import java.util.function.Consumer;

public interface RenderData {
    int vao();
    Shader shader();
    Consumer<Shader> prepareShaderFunc();

    RenderDataType type();
    enum RenderDataType{
        TEXT,
        COMMON
    }

}
