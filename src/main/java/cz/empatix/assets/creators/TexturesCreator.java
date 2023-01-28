package cz.empatix.assets.creators;

import cz.empatix.assets.builders.TextureBuilder;
import cz.empatix.assets.domain.RawImage;
import cz.empatix.assets.loaders.TexturesLoader;
import org.lwjgl.opengl.GL30;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.Map;

public class TexturesCreator implements AssetCreator {
    private final static Logger LOGGER = LoggerFactory.getLogger(TexturesCreator.class);
    private boolean ready;
    private final Map<String, Integer> textures;

    public TexturesCreator() {
        textures = new HashMap<>();
    }

    public Integer get(final String name) {
        return textures.get(TexturesLoader.PATH.resolve(name).toString());
    }

    public void add(final Map<String, RawImage> data) {
        final long start = System.currentTimeMillis();
        final int loaded = textures.size();
        data.entrySet().stream()
                .filter(entry -> !textures.containsKey(entry.getKey()))
                .map(entry -> Map.entry(entry.getKey(), createTexture(entry.getValue())))
                .forEach(entry -> textures.put(entry.getKey(), entry.getValue()));
        LOGGER.info("Total {} textures assets vbos created in {} ms", textures.size() - loaded, System.currentTimeMillis() - start);
    }

    @Override
    public void destroy() {
        textures.forEach((s, integer) -> GL30.glDeleteTextures(integer));
    }

    private int createTexture(final RawImage image) {
        final int texture = TextureBuilder.builder()
                .width(image.width())
                .height(image.height())
                .channels(image.channels())
                .applyFilters()
                .applyWraps()
                .buffer(image.data())
                .build();
        image.destroy();
        return texture;
    }

    @Override
    public boolean isReady() {
        return ready;
    }

    @Override
    public void ready() {
        this.ready = true;
    }
}

