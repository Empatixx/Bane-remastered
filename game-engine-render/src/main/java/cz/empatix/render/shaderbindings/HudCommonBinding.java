package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.ShaderProgram;

public record HudCommonBinding() implements ShaderProgramBinding {
    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("hudCommon");
        shader.bind();
        shader.setUniform1i("texSampler", 0);
        return shader;
    }
}
