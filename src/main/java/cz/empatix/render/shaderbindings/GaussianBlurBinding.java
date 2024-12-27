package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Matrix4f;
import org.joml.Vector2f;

public record GaussianBlurBinding(
        float blurAmount,
        float darknessAmount,
        Matrix4f projectionMatrix
) implements ShaderProgramBinding {

    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("gaussianBlur");
        shader.bind();
        shader.setUniform1i("uTexture", 0);
        shader.setUniform1f("uBlurAmount", blurAmount);
        shader.setUniform1f("uDarknessAmount", darknessAmount);
        shader.setUniform4x4f("projection", projectionMatrix);
        shader.setUniform2f("uTexSize", new Vector2f(WindowHandler.WIDTH, WindowHandler.HEIGHT));
        return shader;
    }
}
