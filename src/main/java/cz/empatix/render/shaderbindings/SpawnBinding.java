package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Matrix4f;

public record SpawnBinding (
        Matrix4f projectionMatrix,
        float time
) implements ShaderBinding{
    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("spawn");
        shader.bind();
        shader.setUniform4x4f("uProjection", projectionMatrix);
        shader.setUniform1i("uTexSample", 0);
        shader.setUniform1f("uTime", time);
        return shader;
    }
}
