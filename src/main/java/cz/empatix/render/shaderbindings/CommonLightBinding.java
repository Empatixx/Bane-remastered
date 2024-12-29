package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.ShaderProgram;
import cz.empatix.textureatlas.domain.Texture;

public record CommonLightBinding(
) implements ShaderProgramBinding{


    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("commonLight");

        final Texture noiseTexture = AssetsService.getNoiseTexture("lightningNoise");
        noiseTexture.bind(1);

        shader.bind();
        shader.setUniform1i("texSampler", 0);
        shader.setUniform1i("noiseSampler", 1);
        return shader;
    }
}
