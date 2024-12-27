package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.lightning.domain.LightSource;
import cz.empatix.render.domain.ShaderProgram;
import cz.empatix.render.service.Camera;
import org.joml.Matrix4f;
import org.joml.Vector2f;

import java.util.List;

public record ForwardLightningBinding(
        List<LightSource> lightSources,
        Matrix4f projectionMatrix
) implements ShaderProgramBinding {

    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("forwardLightning");

        final List<LightSource> lightSources = this.lightSources.stream()
                        .filter(LightSource::canDisplay)
                        .toList();
        shader.bind();
        shader.setUniform1i("texSampler", 0);
        shader.setUniform4x4f("projection", projectionMatrix);
        shader.setUniform2f("resolution", new Vector2f(Camera.getTargetWidth(), Camera.getTargetHeight()));
        shader.setUniform2f("cameraPosition", Camera.getInstance().getPosition());
        for(int i = 0;i<lightSources.size();i++) {
            shader.setUniform2f("lightPosition[" + i + "]", lightSources.get(i).getPosition());
            shader.setUniform1f("lightIntensity[" + i + "]", lightSources.get(i).getIntensity());
            shader.setUniform3f("lightColor[" + i + "]", lightSources.get(i).getColor());
            shader.setUniform1f("lightRadius[" + i + "]", lightSources.get(i).getRadius());
        }
        shader.setUniform1i("numLights", lightSources.size());
        return shader;
    }
}
