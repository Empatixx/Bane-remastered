package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.render.domain.Shader;
import org.joml.Matrix4f;
import org.joml.Vector2f;

import java.awt.*;

public class GaussianBlurBinding implements ShaderBinding{
    private final float blurAmount;
    private final Matrix4f projectionMatrix;

    public GaussianBlurBinding(final float blurAmount,
                               final Matrix4f projectionMatrix) {
        this.blurAmount = blurAmount;
        this.projectionMatrix = projectionMatrix;
    }

    @Override
    public Shader apply() {
        final Shader shader = AssetsService.getShader("gaussianBlur");
        shader.bind();
        shader.setUniform1i("uTexture", 0);
        shader.setUniform1f("uBlurAmount", blurAmount);
        shader.setUniform4x4f("projection", projectionMatrix);
        shader.setUniform2f("uTexSize", new Vector2f(WindowHandler.WIDTH, WindowHandler.HEIGHT));
        return shader;
    }
}
