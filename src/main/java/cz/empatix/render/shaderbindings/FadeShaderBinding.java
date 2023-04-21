package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;

public class FadeShaderBinding implements ShaderBinding{
    private float transparency;
    @Override
    public Shader apply() {
        final Shader shader = AssetsService.getShader("fade");
        shader.setUniform1f("transparency", transparency);
        shader.setUniform1f("sampler", 0);
        return shader;
    }

    public void setTransparency(float transparency) {
        this.transparency = transparency;
    }
}
