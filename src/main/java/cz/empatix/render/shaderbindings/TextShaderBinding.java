package cz.empatix.render.shaderbindings;

import cz.empatix.common.assets.service.AssetsService;
import cz.empatix.render.Camera;
import cz.empatix.render.domain.Shader;

public class TextShaderBinding implements ShaderBinding{
    @Override
    public Shader apply() {
        final Shader shader = AssetsService.getShader("text");
        shader.bind();
        shader.setUniform4x4f("projection", Camera.getProjection());
        return shader;
    }
}
