package cz.empatix.assets.domain.font;

import com.google.common.base.Preconditions;
import cz.empatix.textureatlas.domain.RawTextureAtlas;

import java.nio.file.Path;
import java.util.Map;

public record RawFont(
        Path path,
        FontSize size,
        Map<Character, RawGlyph> glyphs,
        RawTextureAtlas atlas
) {
    public RawFont{
        Preconditions.checkArgument(glyphs != null, "glyphs cannot be null");
        Preconditions.checkArgument(glyphs.size() > 0, "glyphs cannot be empty");
    }
}
