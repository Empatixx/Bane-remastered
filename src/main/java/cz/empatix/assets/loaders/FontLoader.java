package cz.empatix.assets.loaders;

import cz.empatix.assets.domain.RawImage;
import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.domain.font.RawFont;
import cz.empatix.assets.domain.font.RawGlyph;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.common.utils.BufferFactory;
import cz.empatix.textureatlas.domain.RawTextureAtlas;
import cz.empatix.textureatlas.domain.TextureAtlas;
import cz.empatix.textureatlas.factory.TextureAtlasFactory;
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
import java.util.Queue;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import static org.lwjgl.stb.STBTruetype.*;

public class FontLoader implements AssetLoader<RawFont> {
    private final static Logger LOGGER = LoggerFactory.getLogger(FontLoader.class);
    public final static Path PATH = Path.of("./assets/fonts/");
    private final Queue<RawFont> fonts;
    private final TextureAtlasFactory textureAtlasFactory;

    public FontLoader() {
        fonts = new ConcurrentLinkedQueue<>();
        this.textureAtlasFactory = new TextureAtlasFactory();
    }

    @Override
    public int load() {
        LOGGER.info("Loading font files from {}", PATH);
        final long start = System.currentTimeMillis();
        final AtomicInteger total = new AtomicInteger();
        try (final Stream<Path> paths = Files.walk(PATH)) {
            paths
                    .filter(Files::isRegularFile)
                    .filter(p -> p.toString().endsWith(".ttf"))
                    .forEach(p -> {
                        final RawFont font16 = decodeFont(p, FontSize.FONTSIZE_16);
                        fonts.add(font16);

                        final RawFont font22 = decodeFont(p, FontSize.FONTSIZE_22);
                        fonts.add(font22);

                        final RawFont font27 = decodeFont(p, FontSize.FONTSIZE_27);
                        fonts.add(font27);

                        final RawFont font32 = decodeFont(p, FontSize.FONTSIZE_32);
                        fonts.add(font32);

                        total.incrementAndGet();
                    });
            return total.get();
        } catch (IOException e) {
            LOGGER.error("Failed to load font files from {}", PATH);
            throw new RuntimeException(e);
        } finally {
            LOGGER.info("Loaded {} font files in {} ms", total.get(), System.currentTimeMillis() - start);
        }
    }

    @Override
    public int preload() {
        return 0;
    }

    @Override
    public Queue<RawFont> getAssets() {
        return fonts;
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
            final RawTextureAtlas atlas = getAtlas(glyphs);
            return new RawFont(path, fontSize, glyphs, atlas);
        } catch (Exception e) {
            LOGGER.error("Failed to load font file {}", path);
            throw new RuntimeException(e);
        }
    }

    private RawTextureAtlas getAtlas(final Map<Character, RawGlyph> glyphs) {
        final Map<String, RawImage> images = glyphs
                .entrySet()
                .stream()
                .map(e -> {
                    final RawGlyph glyph = e.getValue();
                    final RawImage rawTexture = new RawImage(glyph.bitmapWidth(), glyph.bitmapHeight(), TextureAtlas.R_CHANNELS, glyph.bitmap());
                    return Map.entry(String.valueOf(e.getKey()), rawTexture);
                })
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
        return textureAtlasFactory.createAtlas(images);
    }

}