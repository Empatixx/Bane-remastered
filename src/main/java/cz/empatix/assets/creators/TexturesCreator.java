package cz.empatix.assets.creators;

import cz.empatix.assets.builders.TextureBuilder;
import cz.empatix.assets.domain.RawImage;
import cz.empatix.assets.domain.TextureImage;
import cz.empatix.assets.loaders.TexturesLoader;
import cz.empatix.textureatlas.domain.RawTextureAtlas;
import cz.empatix.textureatlas.factory.TextureAtlasFactory;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.imageio.ImageIO;
import java.awt.image.BufferedImage;
import java.io.File;
import java.util.HashMap;
import java.util.Map;

public class TexturesCreator implements AssetCreator {
    private final static Logger LOGGER = LoggerFactory.getLogger(TexturesCreator.class);
    private boolean ready;
    private final Map<String, TextureImage> textures;

    public TexturesCreator() {
        textures = new HashMap<>();
    }

    public TextureImage get(final String name) {
        return textures.get(TexturesLoader.PATH.resolve(name).toString());
    }

    public void add(final Map<String, RawImage> data) {
        final long start = System.currentTimeMillis();
        final int loaded = textures.size();

        try{
            final RawTextureAtlas atlas = TextureAtlasFactory.createAtlas(data);
            byte[] atlasData = atlas.pixels();
            BufferedImage image = new BufferedImage(atlas.width(), atlas.height(), BufferedImage.TYPE_4BYTE_ABGR);
            image.getRaster().setDataElements(0, 0, atlas.width(), atlas.height(), atlasData);
            ImageIO.write(image, "png", new File("atlas.png"));
        } catch (Exception e){
            throw new RuntimeException(e);
        }

        data.entrySet().stream()
                .filter(entry -> !textures.containsKey(entry.getKey()))
                .map(entry -> Map.entry(entry.getKey(), createTexture(entry.getValue())))
                .forEach(entry -> textures.put(entry.getKey(), entry.getValue()));
        LOGGER.info("Total {} textures assets vbos created in {} ms", textures.size() - loaded, System.currentTimeMillis() - start);
    }

    @Override
    public void destroy() {
        LOGGER.info("Destroying textures");
        textures.forEach((name, img) -> img.destroy());
    }

    private TextureImage createTexture(final RawImage image) {
        final int texture = TextureBuilder.builder()
                .width(image.width())
                .height(image.height())
                .channels(image.channels())
                .applyNearestFilters()
                .applyClampToEdgeWrap()
                .buffer(image.pixels())
                .build();
        return new TextureImage(image.width(), image.height(), texture);
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

