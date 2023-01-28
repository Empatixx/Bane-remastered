package cz.empatix.render.domain.renderdata;

import cz.empatix.render.domain.Shader;

import java.util.function.Consumer;

public record CommonRenderData(
        int vao,
        int textureId,
        Shader shader,
        Consumer<Shader> prepareShaderFunc
) implements RenderData {

    @Override
    public RenderDataType type() {
        return RenderDataType.COMMON;
    }
}
