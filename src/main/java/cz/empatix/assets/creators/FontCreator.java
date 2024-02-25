package cz.empatix.assets.creators;

import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.domain.font.RawFont;
import cz.empatix.assets.loaders.FontLoader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.stream.Collectors;

public class FontCreator implements AssetCreator {
    private static final Logger LOGGER = LoggerFactory.getLogger(FontCreator.class);
    private final Map<String, Map<FontSize, Font>> fonts;
    private final FontLoader fontLoader;

    public FontCreator() {
        fonts = new HashMap<>();
        fontLoader = new FontLoader();
    }

    public Font get(final String name, final FontSize size) {
        return fonts.get(FontLoader.PATH.resolve(name).toString()).get(size);
    }

    @Override
    public void create() {
        int totalInited = 0;
        final long start = System.currentTimeMillis();
        while (!fontLoader.getAssets().isEmpty()) {
            final RawFont rawFont = fontLoader.getAssets().poll();
            fonts.computeIfAbsent(rawFont.path().toString(), s -> new HashMap<>())
                    .put(rawFont.size(), Font.from(rawFont));
            totalInited++;
        }
        final long end = System.currentTimeMillis();
        LOGGER.info("Inited " + totalInited + " fonts in " + (end - start) + "ms");
    }

    @Override
    public void load() {
        fontLoader.load();
    }

    @Override
    public void preload() {
        fontLoader.preload();
    }

    @Override
    public void destroy() {
        LOGGER.info("Destroying fonts");
        fonts.forEach((path, sizedFonts) -> sizedFonts.forEach((size, font) -> {
            font.destroy();
        }));
    }

}
