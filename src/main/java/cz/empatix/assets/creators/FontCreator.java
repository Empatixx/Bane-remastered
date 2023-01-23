package cz.empatix.assets.creators;

import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.domain.font.Glyph;
import cz.empatix.assets.domain.font.RawFont;
import cz.empatix.assets.loaders.FontLoader;
import org.lwjgl.stb.STBTTFontinfo;
import org.lwjgl.system.MemoryStack;

import java.nio.ByteBuffer;
import java.nio.IntBuffer;
import java.util.*;
import java.util.stream.Collectors;

import static org.lwjgl.stb.STBTruetype.*;
import static org.lwjgl.opengl.GL30.*;

public class FontCreator implements AssetCreator {
    private final Map<String, Map<Integer, Font>> fonts;
    private final static int[] FONT_SIZES = {16, 32, 42};
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

                    final Map<Integer, Font> fontSizes = Arrays.stream(FONT_SIZES)
                            .mapToObj(size -> Map.entry(size, transform(rawFont, size)))
                            .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
                    return Map.entry(path, fontSizes);
                }).forEach(entry -> fonts.put(entry.getKey(),entry.getValue()));
    }

    private Font transform(final RawFont font, int size) {
        final STBTTFontinfo fontInfo = font.fontInfo();
        final Map<Character,Glyph> glyphs = new HashMap<>();
        final float scale = stbtt_ScaleForPixelHeight(fontInfo, size);
        for (char c = 'A'; c <= 'z'; c++) {
            try (final MemoryStack memoryStack = MemoryStack.stackPush()) {
                final IntBuffer bufWidth = memoryStack.mallocInt(1);
                final IntBuffer bufHeight = memoryStack.mallocInt(1);
                final IntBuffer bufBearingX = memoryStack.mallocInt(1);
                final IntBuffer bufAdvance = memoryStack.mallocInt(1);
                final IntBuffer bufAscent = memoryStack.mallocInt(1);
                final IntBuffer bufDescent = memoryStack.mallocInt(1);
                final IntBuffer bufLineGap = memoryStack.mallocInt(1);

                stbtt_GetCodepointHMetrics(fontInfo, c, bufAdvance, bufBearingX);
                stbtt_GetFontVMetrics(fontInfo, bufAscent, bufDescent, bufLineGap);
                final ByteBuffer bitmap = stbtt_GetCodepointBitmap(fontInfo, 0, scale, c, bufWidth, bufHeight, null, null);
                final int textureID = glGenTextures();

                final int width = bufWidth.get();
                final int height = bufHeight.get();

                glBindTexture(GL_TEXTURE_2D, textureID);
                glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
                glTexImage2D(GL_TEXTURE_2D, 0, GL_RED, width, height, 0, GL_RED, GL_UNSIGNED_BYTE, bitmap);
                glGenerateMipmap(GL_TEXTURE_2D);

                final Glyph glyph = new Glyph(
                        width,
                        height,
                        (int)(bufBearingX.get() * scale),
                        (int)(bufAdvance.get() * scale),
                        (int)(bufAscent.get() * scale),
                        (int)(bufDescent.get() * scale),
                        bufLineGap.get(),
                        textureID
                );
                glyphs.put(c,glyph);
            }
        }
        return new Font(scale,glyphs);
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

    }
}
