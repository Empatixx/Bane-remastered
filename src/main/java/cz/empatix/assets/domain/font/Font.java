package cz.empatix.assets.domain.font;


import cz.empatix.render.attributes.Drawable;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.stream.Collectors;

public record Font(
        Map<Character, Glyph> glyphs
) {
    public void destroy() {
        glyphs.forEach((c, g) -> g.destroy());
    }

    public List<Drawable> getDrawables(final String text,
                                       final Vector2f position,
                                       final Vector3f color,
                                       final int zDepth) {
        final List<Drawable> glyphDrawables = new ArrayList<>();
        final Vector2f positionShifting = new Vector2f(position);
        for(final char c : text.toCharArray()){
            if(!glyphs.containsKey(c)){
                throw new IllegalArgumentException("Font does not contain glyph for character: " + c);
            }
            final Glyph glyph = glyphs.get(c);
            glyphDrawables.add(glyph.toDrawable(new Vector2f(positionShifting), color, zDepth));
            positionShifting.x += glyph.advance();
        }
        return glyphDrawables;
    }

    public float getWidth(final String text) {
        return (float) text.chars()
                .mapToObj(c -> glyphs.get((char) c))
                .filter(Objects::nonNull)
                .mapToDouble(g -> g.width() + g.bearingX())
                .sum();
    }
    public static Font from(final RawFont font){
        final Map<Character, Glyph> glyphs = font
                .glyphs()
                .entrySet()
                .stream()
                .map(e -> Map.entry(e.getKey(), Glyph.from(e.getValue())))
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
        return new Font(glyphs);
    }
}
