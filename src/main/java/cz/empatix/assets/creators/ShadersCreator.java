package cz.empatix.assets.creators;

import cz.empatix.assets.domain.RawShader;
import cz.empatix.assets.loaders.ShadersLoader;
import cz.empatix.render.domain.Shader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Map;

public class ShadersCreator implements AssetCreator {
    private static final Logger LOGGER = LoggerFactory.getLogger(ShadersCreator.class);
    private final Map<String, Shader> shaders;
    private final ShadersLoader shadersLoader;

    public ShadersCreator() {
        this.shaders = new HashMap<>();
        shadersLoader = new ShadersLoader();
    }

    public Shader get(final String name) {
        return shaders.get(name);
    }

    @Override
    public void create() {
        final long start = System.currentTimeMillis();
        int totalInited = 0;
        while(!shadersLoader.getAssets().isEmpty()){
            final List<RawShader> rawShaders = shadersLoader.getAssets().poll();
            shaders.put(rawShaders.get(0).getName(), Shader.from(rawShaders));
            totalInited++;
        }
        final long end = System.currentTimeMillis();
        LOGGER.info("Inited " + totalInited + " shaders in " + (end - start) + "ms");

    }

    @Override
    public void destroy() {
        LOGGER.info("Destroying shaders");
        shaders.values().forEach(Shader::destroy);
    }
    @Override
    public void load() {
        shadersLoader.load();
    }

    @Override
    public void preload() {
        shadersLoader.preload();
    }
}

