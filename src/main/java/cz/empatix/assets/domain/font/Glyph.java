package cz.empatix.assets.domain.font;

public record Glyph (
        int width,
        int height,
        int bearingX,
        int advance,
        int ascent,
        int descent,
        int lineGap,
        int textureId
){
}
