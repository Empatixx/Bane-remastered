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
    private final Map<String, Map<FontSize, RawFont>> rawFonts;
    private final FontLoader fontLoader;

    public FontCreator() {
        fonts = new HashMap<>();
        rawFonts = new ConcurrentHashMap<>();
        fontLoader = new FontLoader(rawFonts);
    }

    public Font get(final String name, final FontSize size) {
        return fonts.get(FontLoader.PATH.resolve(name).toString()).get(size);
    }

    @Override
    public void create() {
        final Iterator<Map.Entry<String, Map<FontSize, RawFont>>> iterator = rawFonts.entrySet().iterator();
        while (iterator.hasNext()) {
            final Map.Entry<String, Map<FontSize, RawFont>> entry = iterator.next();

            final String path = entry.getKey();
            final Map<FontSize, RawFont> rawSizedFonts = entry.getValue();
            final Map<FontSize, Font> fonts = rawSizedFonts.entrySet()
                    .stream()
                    .collect(Collectors.toMap(Map.Entry::getKey, fontEntry -> Font.from(fontEntry.getValue())));
            this.fonts.put(path, fonts);
            iterator.remove();
        }
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
