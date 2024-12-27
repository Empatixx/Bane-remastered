package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Matrix4f;

public record RedVignetteBinding (
        Matrix4f projection,
        float vignetteAmount,
        boolean timeBased,
        float time
) implements ShaderProgramBinding {

    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("redVignette");
        shader.bind();
        shader.setUniform4x4f("u_projection", projection);
        shader.setUniform1i("u_texture", 0);
        shader.setUniform1f("u_vignetteAmount", vignetteAmount);
        shader.setUniform1i("u_timeBased", timeBased ? 1 : 0);
        shader.setUniform1f("u_time", time );
        return shader;
    }
}
