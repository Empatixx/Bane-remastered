package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;
import org.joml.Matrix4f;

public class FadeBinding implements ShaderBinding{
    private final float transparency;
    private final Matrix4f projection;
    public FadeBinding(final float transparency, final Matrix4f projection){
        this.transparency = transparency;
        this.projection = projection;
    }
    @Override
    public Shader apply() {
        final Shader shader = AssetsService.getShader("fade");
        shader.bind();
        shader.setUniform1f("uValue", transparency);
        shader.setUniform4x4f("projection", projection);
        shader.setUniform1i("uTexSampler", 0);
        return shader;
    }

}
