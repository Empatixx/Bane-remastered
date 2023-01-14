package cz.empatix.assets.creators;

import cz.empatix.assets.domain.RawShader;
import cz.empatix.render.Shader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Map;
import java.util.stream.Collectors;

public class ShadersCreator implements AssetCreator<RawShader,Shader> {
    private static final Logger LOGGER = LoggerFactory.getLogger(AudioCreator.class);
    private Map<String, Shader> shaders;
    @Override
    public Shader get(final String name) {
        return shaders.get(name);
    }

    @Override
    public void create(final Map<String, RawShader> data) {
        LOGGER.info("Creating shaders...");
        final long start = System.currentTimeMillis();
        this.shaders = data.entrySet().stream()
                .map(entry -> {
                    final RawShader rawShader = entry.getValue();
                    final Shader shader = new Shader(rawShader.vertex(), rawShader.fragment());
                    return Map.entry(entry.getKey(), shader);
                })
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
        LOGGER.info("Shaders created in {} ms", System.currentTimeMillis() - start);
    }

    @Override
    public void destroy() {
        shaders.values().forEach(Shader::destroy);

    }
}

