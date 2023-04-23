package cz.empatix.render.shaderbindings;

import cz.empatix.lightning.domain.LightSource;
import cz.empatix.render.domain.Shader;

import java.util.List;

public class ForwardLightningBinding implements ShaderBinding{
    private final List<LightSource> lightSources;
    public ForwardLightningBinding(final List<LightSource> lightSources) {
        this.lightSources = lightSources;
    }
    @Override
    public Shader apply() {
        return null;
    }
}
