package cz.empatix.assets.creators;

import cz.empatix.assets.domain.RawShader;
import cz.empatix.assets.loaders.ShadersLoader;
import cz.empatix.render.domain.Shader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.Callable;
import java.util.concurrent.ConcurrentHashMap;

public class ShadersCreator implements AssetCreator {
    private static final Logger LOGGER = LoggerFactory.getLogger(ShadersCreator.class);
    private final Map<String, RawShader> rawShaders;
    private final Map<String, Shader> shaders;
    private final ShadersLoader shadersLoader;

    public ShadersCreator() {
        this.shaders = new HashMap<>();
        this.rawShaders = new ConcurrentHashMap<>();
        shadersLoader = new ShadersLoader(rawShaders);
    }

    public Shader get(final String name) {
        return shaders.get(name);
    }

    @Override
    public void create() {
        final long start = System.currentTimeMillis();
        final int loaded = shaders.size();
        rawShaders.entrySet().stream()
                .filter(entry -> !shaders.containsKey(entry.getKey()))
                .map(entry -> Map.entry(entry.getKey(), Shader.from(entry.getValue())))
                .forEach(entry -> shaders.put(entry.getKey(), entry.getValue()));
        LOGGER.info("Total {} shaders vbos created in {} ms", shaders.size() - loaded, System.currentTimeMillis() - start);
    }

    @Override
    public void destroy() {
        LOGGER.info("Destroying shaders");
        shaders.values().forEach(Shader::destroy);
    }
    @Override
    public Callable<Void> load() {
        return null;
    }
}

