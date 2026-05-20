package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.ShaderProgram;

public record ShadowBinding(
        float shadowIntensity
) implements ShaderProgramBinding {

    public ShadowBinding() {
        this(0.7f);
    }

    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("shadow");
        shader.bind();
        shader.setUniform1f("u_shadowIntensity", shadowIntensity);
        return shader;
    }
}
