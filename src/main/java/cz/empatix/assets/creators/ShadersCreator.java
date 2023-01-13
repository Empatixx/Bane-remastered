package cz.empatix.assets.creators;

import cz.empatix.assets.domain.RawShader;
import cz.empatix.render.Shader;

import java.util.Map;
import java.util.stream.Collectors;

public class ShadersCreator implements AssetCreator<RawShader,Shader> {
    private Map<String, Shader> shaders;

    @Override
    public Shader get(final String name) {
        return shaders.get(name);
    }

    @Override
    public void create(final Map<String, RawShader> data) {
        this.shaders = data.entrySet().stream()
                .map(entry -> {
                    final RawShader rawShader = entry.getValue();
                    final Shader shader = new Shader(rawShader.vertex(), rawShader.fragment());
                    return Map.entry(entry.getKey(), shader);
                })
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
    }

    @Override
    public void destroy() {
        shaders.values().forEach(Shader::destroy);

    }
}

