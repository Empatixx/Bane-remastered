package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.render.domain.Shader;
import org.joml.Matrix4f;
import org.joml.Vector2f;

public class GaussianBlurBinding implements ShaderBinding {
    private final float blurAmount;
    private final float darknessAmount;
    private final Matrix4f projectionMatrix;

    public GaussianBlurBinding(final float blurAmount,
                               final float darknessAmount,
                               final Matrix4f projectionMatrix) {
        this.blurAmount = blurAmount;
        this.darknessAmount = darknessAmount;
        this.projectionMatrix = projectionMatrix;
    }

    @Override
    public Shader apply() {
        final Shader shader = AssetsService.getShader("gaussianBlur");
        shader.bind();
        shader.setUniform1i("uTexture", 0);
        shader.setUniform1f("uBlurAmount", blurAmount);
        shader.setUniform1f("uDarknessAmount", darknessAmount);
        shader.setUniform4x4f("projection", projectionMatrix);
        shader.setUniform2f("uTexSize", new Vector2f(WindowHandler.WIDTH, WindowHandler.HEIGHT));
        return shader;
    }
}
