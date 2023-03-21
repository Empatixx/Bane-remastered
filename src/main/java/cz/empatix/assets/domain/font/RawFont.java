package cz.empatix.assets.domain.font;

import java.util.Map;

public record RawFont(
        Map<Character, RawGlyph> glyphs
) {
}
