package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Matrix4f;

public record FadeBinding (
        float transparency
) implements ShaderProgramBinding {
    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("fade");
        shader.bind();
        shader.setUniform1f("uValue", transparency);
        shader.setUniform1i("uTexSampler", 0);
        return shader;
    }

}
