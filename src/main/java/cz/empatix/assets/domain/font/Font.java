package cz.empatix.assets.domain.font;

import java.util.Map;

public record Font (
        float scale,
        Map<Character,Glyph> glyphs
){
}
