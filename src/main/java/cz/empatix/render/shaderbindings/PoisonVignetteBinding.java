package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;
import org.joml.Matrix4f;

public record PoisonVignetteBinding(
        Matrix4f projection,
        float vignetteAmount,
        float time
) implements ShaderBinding{
    @Override
    public Shader apply() {
        final Shader shader = AssetsService.getShader("poison");
        shader.bind();
        shader.setUniform4x4f("u_projection", projection);
        shader.setUniform1i("u_texture", 0);
        shader.setUniform1f("u_vignetteAmount", vignetteAmount);
        shader.setUniform1f("u_time", time );
        return shader;
    }
}
