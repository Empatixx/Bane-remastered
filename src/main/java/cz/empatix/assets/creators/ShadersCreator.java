package cz.empatix.assets.creators;

import cz.empatix.assets.domain.RawShader;
import cz.empatix.assets.loaders.ShadersLoader;
import cz.empatix.render.Shader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.atomic.AtomicBoolean;

public class ShadersCreator implements AssetCreator<RawShader, Shader> {
    private static final Logger LOGGER = LoggerFactory.getLogger(ShadersCreator.class);
    private final AtomicBoolean ready;
    private final Map<String, Shader> shaders;

    public ShadersCreator() {
        this.shaders = new HashMap<>();
        ready = new AtomicBoolean(false);
    }

    @Override
    public Shader get(final String name) {
        return shaders.get(name);
    }

    @Override
    public void add(final Map<String, RawShader> data) {
        final long start = System.currentTimeMillis();
        final int loaded = shaders.size();
        data.entrySet().stream()
                .filter(entry -> !shaders.containsKey(entry.getKey()))
                .map(entry -> {
                    final RawShader rawShader = entry.getValue();
                    final Shader shader = new Shader(rawShader.vertex(), rawShader.fragment());
                    return Map.entry(entry.getKey(), shader);
                })
                .forEach(entry -> shaders.put(entry.getKey(), entry.getValue()));
        LOGGER.info("Total {} shaders vbos created in {} ms", shaders.size() - loaded, System.currentTimeMillis() - start);
    }

    @Override
    public void destroy() {
        shaders.values().forEach(Shader::destroy);

    }

    @Override
    public boolean getReady() {
        return ready.get();
    }

    public void ready() {
        this.ready.set(true);
    }
}

