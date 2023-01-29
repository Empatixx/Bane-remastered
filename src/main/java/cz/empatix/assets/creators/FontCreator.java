package cz.empatix.assets.creators;

import cz.empatix.assets.builders.TextureBuilder;
import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.domain.font.Glyph;
import cz.empatix.assets.domain.font.RawFont;
import cz.empatix.assets.loaders.FontLoader;
import cz.empatix.handlers.WindowHandler;
import org.lwjgl.stb.STBTTFontinfo;
import org.lwjgl.system.MemoryStack;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.ByteBuffer;
import java.nio.IntBuffer;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

import static org.lwjgl.stb.STBTruetype.*;

public class FontCreator implements AssetCreator {
    private static final Logger LOGGER = LoggerFactory.getLogger(FontCreator.class);
    private final Map<String, Map<Integer, Font>> fonts;
    public final static int FONT_SIZE_16 = 16;
    public final static int FONT_SIZE_32 = 32;
    public final static int FONT_SIZE_42 = 42;

    private boolean ready;

    public FontCreator() {
        fonts = new HashMap<>();
    }

    public Font get(final String name, final int size) {
        return fonts.get(FontLoader.PATH.resolve(name).toString()).get(size);
    }

    public void add(final Map<String, RawFont> data) {
        data.entrySet().stream()
                .map(entry -> {
                    final String path = entry.getKey();
                    final RawFont rawFont = entry.getValue();

                    final Map<Integer, Font> fontSizes = Arrays.stream(new int[]{FONT_SIZE_16, FONT_SIZE_32, FONT_SIZE_42})
                            .mapToObj(size -> Map.entry(size, transform(rawFont, size)))
                            .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
                    return Map.entry(path, fontSizes);
                }).forEach(entry -> fonts.put(entry.getKey(), entry.getValue()));
    }

    private Font transform(final RawFont font, int size) {
        final STBTTFontinfo fontInfo = font.fontInfo();
        final Map<Character, Glyph> glyphs = new HashMap<>();
        final float scaleT = stbtt_ScaleForPixelHeight(fontInfo, size * 1440/360f);
        final float scale = stbtt_ScaleForPixelHeight(fontInfo, size);

        for (char c = 0; c <= 255; c++) {
            try (final MemoryStack memoryStack = MemoryStack.stackPush()) {
                final IntBuffer bufWidth = memoryStack.mallocInt(1);
                final IntBuffer bufHeight = memoryStack.mallocInt(1);
                final IntBuffer bufBearingX = memoryStack.mallocInt(1);
                final IntBuffer bufAdvance = memoryStack.mallocInt(1);
                final IntBuffer bufMaxAscent = memoryStack.mallocInt(1);
                final IntBuffer bufMaxDescent = memoryStack.mallocInt(1);
                final IntBuffer bufLineGap = memoryStack.mallocInt(1);
                final IntBuffer bufDescent = memoryStack.mallocInt(1);
                final IntBuffer bufAscent = memoryStack.mallocInt(1);

                stbtt_GetCodepointHMetrics(fontInfo, c, bufAdvance, bufBearingX);
                stbtt_GetFontVMetrics(fontInfo, bufMaxAscent, bufMaxDescent, bufLineGap);
                final ByteBuffer bitmap = stbtt_GetCodepointBitmap(fontInfo, 0, scaleT, c, bufWidth, bufHeight, null, null);
                stbtt_GetCodepointBox(fontInfo, c, null, bufDescent, null, bufAscent);

                final int width = bufWidth.get();
                final int height = bufHeight.get();

                Glyph.GlyphBuilder glyphBuilder = Glyph.builder();
                if (bitmap != null) {
                    final int textureId = TextureBuilder.builder()
                            .width(width)
                            .height(height)
                            .channels(1)
                            .applyNearestFilters()
                            .buffer(bitmap)
                            .build();
                    glyphBuilder = glyphBuilder.textureId(textureId);
                } else {
                    LOGGER.warn("Bitmap for character '{}' is null", c);
                }
                final Glyph glyph = glyphBuilder
                        .width(width)
                        .height(height)
                        .bearingX(Math.round(bufBearingX.get() * scale))
                        .advance(Math.round(bufAdvance.get() * scale))
                        .ascent(Math.round(bufAscent.get() * scale))
                        .descent(-Math.round(bufDescent.get() * scale))
                        .lineGap(bufLineGap.get())
                        .build();
                glyphs.put(c, glyph);
            }
        }
        return Font.builder()
                .scale(scale)
                .glyphs(glyphs)
                .build();
    }

    @Override
    public boolean isReady() {
        return ready;
    }

    @Override
    public void ready() {
        ready = true;
    }

    @Override
    public void destroy() {
        LOGGER.info("Destroying fonts");
        final List<Font> fonts = this.fonts.values().stream()
                .flatMap(map -> map.values().stream())
                .toList();
        fonts.forEach(Font::destroy);
    }
}
