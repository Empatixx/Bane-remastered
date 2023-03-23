package cz.empatix.assets.creators;

import cz.empatix.assets.loaders.TexturesLoader;
import cz.empatix.textureatlas.domain.RawTextureAtlas;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.Queue;
import java.util.concurrent.ConcurrentLinkedQueue;

public class TexturesCreator implements AssetCreator {
    private final static Logger LOGGER = LoggerFactory.getLogger(TexturesCreator.class);
    public final static Path PATH = Path.of("./assets/textures/");
    private final TexturesLoader texturesLoader;
    private final List<TextureAtlas> textureAtlases;
    private final Queue<RawTextureAtlas> rawTextureAtlases;

    public TexturesCreator() {
        textureAtlases = new ArrayList<>();
        rawTextureAtlases = new ConcurrentLinkedQueue<>();
        texturesLoader = new TexturesLoader(rawTextureAtlases);
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
        final Iterator<RawTextureAtlas> iterator = rawTextureAtlases.iterator();
        while (iterator.hasNext()){
            final RawTextureAtlas rawAtlas = iterator.next();
            textureAtlases.add(TextureAtlas.from(rawAtlas));
            iterator.remove();
        }
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

