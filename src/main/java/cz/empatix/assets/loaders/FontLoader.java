package cz.empatix.assets.loaders;

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
import java.util.Map;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.Stream;

public class FontLoader implements AssetLoader {
    private final static Logger LOGGER = LoggerFactory.getLogger(FontLoader.class);
    public final static Path PATH = Path.of("./assets/fonts/");
    private final Map<String, RawFont> fonts;

    public FontLoader(final Map<String, RawFont> fonts) {
        this.fonts = fonts;
    }

    @Override
    public int load() {
        LOGGER.info("Loading font files from {}", PATH);
        final long start = System.currentTimeMillis();
        final AtomicInteger total = new AtomicInteger();
        try (final Stream<Path> paths = Files.walk(PATH)) {
            paths
                    .parallel()
                    .filter(Files::isRegularFile)
                    .filter(p -> p.toString().endsWith(".ttf"))
                    .map(path -> Map.entry(path.toString(), decodeFont(path)))
                    .forEach(entry -> {
                        this.fonts.put(entry.getKey(), entry.getValue());
                        total.incrementAndGet();
                    });
            return total.get();
        } catch (IOException e) {
            throw new RuntimeException(e);
        } finally {
            LOGGER.info("Loaded {} font files in {} ms", total.get(), System.currentTimeMillis() - start);
        }
    }

    @Override
    public int preload() {
        return 0;
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
            return new RawFont(info, buffer);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

}
