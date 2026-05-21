package cz.empatix.shaderbindings;

import cz.empatix.render.shaderbindings.ShaderProgramBinding;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Matrix4f;

public record SpawnBinding (
        float time
) implements ShaderProgramBinding {
    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("spawn");
        shader.bind();
        shader.setUniform1i("uTexSample", 0);
        shader.setUniform1f("uTime", time);
        return shader;
    }
}
