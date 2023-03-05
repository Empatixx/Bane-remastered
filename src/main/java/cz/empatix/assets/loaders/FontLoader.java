package cz.empatix.assets.loaders;

import cz.empatix.assets.creators.AssetCreator;
import cz.empatix.assets.creators.FontCreator;
import cz.empatix.assets.domain.font.RawFont;
import cz.empatix.common.utils.BufferFactory;
import org.lwjgl.stb.STBTTFontinfo;
import org.lwjgl.stb.STBTruetype;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.Map;
import java.util.stream.Stream;

public class FontLoader implements AssetLoader {
    private final static Logger LOGGER = LoggerFactory.getLogger(FontLoader.class);

    public final static Path PATH = Path.of("./assets/fonts/");
    private final Map<String, RawFont> fonts;
    private final FontCreator fontCreator;

    public FontLoader() {
        fonts = new HashMap<>();
        fontCreator = new FontCreator();
    }

    @Override
    public int load() {
        try (final Stream<Path> paths = Files.walk(PATH)) {
            paths.filter(Files::isRegularFile)
                    .filter(p -> p.toString().endsWith(".ttf"))
                    .parallel()
                    .map(path -> Map.entry(path.toString(), decodeFont(path)))
                    .forEach(entry -> this.fonts.put(entry.getKey(), entry.getValue()));
            return fonts.size();
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public int preload() {
        return 0;
    }

    @Override
    public void submit() {
        fontCreator.add(fonts);
    }

    @Override
    public AssetCreator getCreator() {
        return fontCreator;
    }

    @Override
    public AssetLoader call() throws Exception {
        final long start = System.currentTimeMillis();
        final int loaded = load();
        LOGGER.info("Loaded {} fonts files in {} ms", loaded, System.currentTimeMillis() - start);
        return this;
    }

    private RawFont decodeFont(final Path path) {
        try {
            final STBTTFontinfo info = STBTTFontinfo.create();
            final byte[] bytesFont = Files.readAllBytes(path);
            final ByteBuffer buffer = BufferFactory.convertByteArray(bytesFont);
            buffer.flip();
            if (!STBTruetype.stbtt_InitFont(info, buffer,0)) {
                throw new IllegalStateException("Failed to initialize font information.");
            }
            return new RawFont(info);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

}
