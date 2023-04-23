package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;

public class FadeBinding implements ShaderBinding{
    private final float transparency;
    public FadeBinding(final float transparency){
        this.transparency = transparency;
    }
    @Override
    public Shader apply() {
        final Shader shader = AssetsService.getShader("fade");
        shader.setUniform1f("transparency", transparency);
        shader.setUniform1f("texSampler", 0);
        return shader;
    }

}
