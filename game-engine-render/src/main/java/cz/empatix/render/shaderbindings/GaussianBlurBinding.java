package cz.empatix.render.shaderbindings;
import cz.empatix.core.Viewport;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Matrix4f;
import org.joml.Vector2f;

public record GaussianBlurBinding(
        float blurAmount,
        float darknessAmount
) implements ShaderProgramBinding {

    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("gaussianBlur");
        shader.bind();
        shader.setUniform1i("uTexture", 0);
        shader.setUniform1f("uBlurAmount", blurAmount);
        shader.setUniform1f("uDarknessAmount", darknessAmount);
        shader.setUniform2f("uTexSize", new Vector2f(Viewport.WIDTH, Viewport.HEIGHT));
        return shader;
    }
}
