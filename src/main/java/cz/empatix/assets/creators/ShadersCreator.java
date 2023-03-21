package cz.empatix.assets.creators;

import cz.empatix.assets.domain.RawShader;
import cz.empatix.assets.loaders.ShadersLoader;
import cz.empatix.render.domain.Shader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;
import java.util.concurrent.Callable;
import java.util.concurrent.ConcurrentHashMap;

public class ShadersCreator implements AssetCreator {
    private static final Logger LOGGER = LoggerFactory.getLogger(ShadersCreator.class);
    private final Map<String, Shader> shaders;
    private final Map<String, RawShader> rawShaders;
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
        final Iterator<Map.Entry<String, RawShader>> iterator = rawShaders.entrySet().iterator();
        while (iterator.hasNext()) {
            final Map.Entry<String, RawShader> entry = iterator.next();
            shaders.put(entry.getKey(), Shader.from(entry.getValue()));
            iterator.remove();
        }
    }

    @Override
    public void destroy() {
        LOGGER.info("Destroying shaders");
        shaders.values().forEach(Shader::destroy);
    }
    @Override
    public Runnable load() {
        return shadersLoader::load;
    }

    @Override
    public void preload() {
        shadersLoader.preload();
    }
}

