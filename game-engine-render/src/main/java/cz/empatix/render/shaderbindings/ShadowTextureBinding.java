package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.ShaderProgram;

public record ShadowTextureBinding(
        float shadowIntensity
) implements ShaderProgramBinding {

    public ShadowTextureBinding() {
        this(0.5f);
    }

    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("shadowTexture");
        shader.bind();
        shader.setUniform1f("u_shadowIntensity", shadowIntensity);
        shader.setUniform1i("texSampler", 0);
        return shader;
    }
}
