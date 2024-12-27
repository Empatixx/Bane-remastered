package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Matrix4f;

public record FadeBinding (
    float transparency,
    Matrix4f projection

) implements ShaderBinding{
    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("fade");
        shader.bind();
        shader.setUniform1f("uValue", transparency);
        shader.setUniform4x4f("projection", projection);
        shader.setUniform1i("uTexSampler", 0);
        return shader;
    }

}
