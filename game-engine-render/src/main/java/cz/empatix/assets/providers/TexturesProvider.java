package cz.empatix.assets.providers;
import cz.empatix.core.AssetProvider;

import cz.empatix.assets.loaders.TexturesLoader;
import cz.empatix.textureatlas.domain.RawTextureAtlas;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

public class TexturesProvider implements AssetProvider {
    private final static Logger LOGGER = LoggerFactory.getLogger(TexturesProvider.class);
    public final static Path PATH = Path.of("./assets/textures/");
    private final TexturesLoader texturesLoader;
    private final List<TextureAtlas> textureAtlases;

    public TexturesProvider() {
        textureAtlases = new ArrayList<>();
        texturesLoader = new TexturesLoader();
    }

    public TextureAtlas.Entry get(final String name) {
        final String key = PATH.resolve(name).toString();
        return textureAtlases
                .stream()
                .filter(atlas -> atlas.texturesInfos().containsKey(key))
                .findAny()
                .map(atlas -> atlas.texturesInfos().get(key))
                .orElseThrow(() -> new IllegalArgumentException("Texture " + key + " not found"));
    }

    @Override
    public void create() {
        final long start = System.currentTimeMillis();
        int totalInited = 0;
        while (!texturesLoader.getAssets().isEmpty()) {
            final RawTextureAtlas rawTextureAtlas = texturesLoader.getAssets().poll();
            textureAtlases.add(TextureAtlas.from(rawTextureAtlas));
            totalInited++;
        }
        final long end = System.currentTimeMillis();
        LOGGER.info("Inited " + totalInited + " textures in " + (end - start) + "ms");
    }

    @Override
    public void load() {
        texturesLoader.load();
    }

    @Override
    public void preload() {
        texturesLoader.preload();
    }

    @Override
    public void destroy() {
        LOGGER.info("Destroying textures");
        textureAtlases.forEach(TextureAtlas::destroy);
    }
}

