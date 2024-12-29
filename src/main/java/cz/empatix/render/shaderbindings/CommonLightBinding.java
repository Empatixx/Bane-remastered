package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.lightning.domain.LightSource;
import cz.empatix.render.domain.ShaderProgram;
import cz.empatix.render.service.Camera;

import java.util.List;

public record CommonLightBinding(
        List<LightSource> lightSources
) implements ShaderProgramBinding{
    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("commonLight");
        shader.bind();
        shader.setUniform1i("texSampler", 0);
        for(int i = 0;i<lightSources.size();i++) {
            shader.setUniform2f("lights[" + i + "].position", lightSources.get(i).getPosition());
            shader.setUniform1f("lights[" + i + "].intensity", lightSources.get(i).getIntensity());
            shader.setUniform3f("lights[" + i + "].color", lightSources.get(i).getColor());
            shader.setUniform1f("lights[" + i + "].radius", lightSources.get(i).getRadius());
        }
        shader.setUniform1i("numLights", lightSources.size());
        return shader;
    }
}
