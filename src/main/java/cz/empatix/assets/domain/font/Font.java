package cz.empatix.assets.domain.font;


import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.builders.TextureBuilder;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

public record Font(
        Map<Character, Glyph> glyphs,
        TextureAtlas atlas
) {
    public void destroy() {
        atlas.destroy();
    }

    public List<Renderable> getDrawables(final String text,
                                         final Vector2f position,
                                         final Vector3f color,
                                         final int zDepth,
                                         final RenderScene renderScene,
                                         final Matrix4f cameraMatrix) {
        final List<Renderable> glyphDrawables = new ArrayList<>();
        final Vector2f positionShifting = new Vector2f(position);
        for (final char c : text.toCharArray()) {
            if (!glyphs.containsKey(c)) {
                throw new IllegalArgumentException("Font does not contain glyph for character: " + c);
            }
            final Glyph glyph = glyphs.get(c);
            glyphDrawables.add(glyph.toDrawable(new Vector2f(positionShifting), color, zDepth, renderScene, cameraMatrix));
            positionShifting.x += glyph.advance();
        }
        return glyphDrawables;
    }

    public float getHeight(final String text){
        return (int) text.chars()
                .mapToObj(c -> glyphs.getOrDefault((char) c, null))
                .mapToDouble(g -> g.ascent() + g.descent())
                .max()
                .orElse(0);
    }
    public float getWidth(final String text) {
        return (int) text.chars()
                .mapToObj(c -> glyphs.getOrDefault((char) c, null))
                .mapToDouble(g -> g.advance() + g.bearingX())
                .sum();
    }

    public static Font from(final RawFont font) {
        final TextureAtlas atlas = TextureAtlas.from(font.atlas(), TextureBuilder.BuildParam.NEAREST_FILTER);
        final Map<Character, Glyph> glyphs = font
                .glyphs()
                .entrySet()
                .stream()
                .map(e -> {
                    final char key = e.getKey();
                    final RawGlyph value = e.getValue();
                    return Map.entry(key, Glyph.from(value, atlas.texturesInfos().get(String.valueOf(key))));
                })
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
        return new Font(glyphs, atlas);
    }
}
