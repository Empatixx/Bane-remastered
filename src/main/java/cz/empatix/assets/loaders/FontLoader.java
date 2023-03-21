package cz.empatix.assets.loaders;

        import cz.empatix.assets.domain.font.FontSize;
        import cz.empatix.assets.domain.font.RawFont;
        import cz.empatix.assets.domain.font.RawGlyph;
        import cz.empatix.common.handlers.WindowHandler;
        import cz.empatix.common.utils.BufferFactory;
        import org.lwjgl.stb.STBTTFontinfo;
        import org.lwjgl.stb.STBTruetype;
        import org.lwjgl.system.MemoryStack;
        import org.slf4j.Logger;
        import org.slf4j.LoggerFactory;

        import java.io.IOException;
        import java.nio.ByteBuffer;
        import java.nio.IntBuffer;
        import java.nio.file.Files;
        import java.nio.file.Path;
        import java.util.HashMap;
        import java.util.Map;
        import java.util.concurrent.atomic.AtomicInteger;
        import java.util.stream.Stream;

        import static org.lwjgl.stb.STBTruetype.*;

public class FontLoader implements AssetLoader {
    private final static Logger LOGGER = LoggerFactory.getLogger(FontLoader.class);
    public final static Path PATH = Path.of("./assets/fonts/");
    private final Map<String, Map<FontSize, RawFont>> fonts;

    public FontLoader(final Map<String, Map<FontSize, RawFont>> fonts) {
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
                    .forEach(p -> {
                        final Map<FontSize, RawFont> fontSizes = Map.of(
                                FontSize.FONTSIZE_16, decodeFont(p, FontSize.FONTSIZE_16),
                                FontSize.FONTSIZE_20, decodeFont(p, FontSize.FONTSIZE_20),
                                FontSize.FONTSIZE_24, decodeFont(p, FontSize.FONTSIZE_24));
                        fonts.put(p.toString(), fontSizes);
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

    private RawFont decodeFont(final Path path, final FontSize fontSize) {
        try {
            final STBTTFontinfo info = STBTTFontinfo.create();
            final byte[] bytesFont = Files.readAllBytes(path);
            final ByteBuffer buffer = BufferFactory.convertByteArray(bytesFont);
            buffer.flip();
            if (!STBTruetype.stbtt_InitFont(info, buffer, 0)) {
                throw new IllegalStateException("Failed to initialize font information.");
            }
            final Map<Character, RawGlyph> glyphs = new HashMap<>();
            final float scale = stbtt_ScaleForPixelHeight(info, fontSize.getSize());
            final float scaleForBitmap = stbtt_ScaleForPixelHeight(info, fontSize.getSize() * WindowHandler.getMaxHeight() / WindowHandler.TARGET_HEIGHT);
            for (char c = 0; c <= 255; c++) {
                try (final MemoryStack memoryStack = MemoryStack.stackPush()) {
                    final IntBuffer bufBitmapWidth = memoryStack.mallocInt(1);
                    final IntBuffer bufBitmapHeight = memoryStack.mallocInt(1);
                    final IntBuffer bufWidth = memoryStack.mallocInt(1);
                    final IntBuffer bufHeight = memoryStack.mallocInt(1);
                    final IntBuffer bufBearingX = memoryStack.mallocInt(1);
                    final IntBuffer bufAdvance = memoryStack.mallocInt(1);
                    final IntBuffer bufMaxAscent = memoryStack.mallocInt(1);
                    final IntBuffer bufMaxDescent = memoryStack.mallocInt(1);
                    final IntBuffer bufLineGap = memoryStack.mallocInt(1);
                    final IntBuffer bufDescent = memoryStack.mallocInt(1);
                    final IntBuffer bufAscent = memoryStack.mallocInt(1);

                    final ByteBuffer bitmap = stbtt_GetCodepointBitmap(info, 0, scaleForBitmap, c, bufBitmapWidth, bufBitmapHeight, null, null);

                    stbtt_GetCodepointBitmap(info, 0, scale, c, bufWidth, bufHeight, null, null);
                    stbtt_GetCodepointBox(info, c, null, bufDescent, null, bufAscent);
                    stbtt_GetCodepointHMetrics(info, c, bufAdvance, bufBearingX);
                    stbtt_GetFontVMetrics(info, bufMaxAscent, bufMaxDescent, bufLineGap);

                    final RawGlyph glyph = RawGlyph.builder()
                            .width(bufWidth.get())
                            .height(bufHeight.get())
                            .bearingX((int) Math.ceil(bufBearingX.get() * scale))
                            .advance((int) Math.ceil(bufAdvance.get() * scale))
                            .ascent((int) Math.ceil(bufAscent.get() * scale))
                            .descent(-(int) Math.ceil(bufDescent.get() * scale))
                            .lineGap((int) Math.ceil(bufLineGap.get() * scale))
                            .bitmap(bitmap)
                            .bitmapWidth(bufBitmapWidth.get())
                            .bitmapHeight(bufBitmapHeight.get())
                            .build();
                    glyphs.put(c, glyph);
                }
            }
            return new RawFont(glyphs);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

}
