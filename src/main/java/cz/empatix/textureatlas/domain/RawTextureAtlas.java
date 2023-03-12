package cz.empatix.textureatlas.domain;

import com.google.common.base.Preconditions;
import org.joml.Vector2i;

import java.util.Map;

import static cz.empatix.textureatlas.domain.TextureAtlas.RGBA_CHANNELS;

public record RawTextureAtlas(
        byte[] pixels,
        int width,
        int height,
        Map<String, TextureAtlasInfo> texturesInfos
) {
    public RawTextureAtlas {
        Preconditions.checkArgument(width > 0, "Width must be positive");
        Preconditions.checkArgument(height > 0, "Height must be positive");
        Preconditions.checkArgument(pixels.length == width * height * RGBA_CHANNELS, "Pixels must match width and height");
        Preconditions.checkArgument(!texturesInfos.isEmpty(), "Textures info must not be empty");
    }
    public int area() {
        return width * height;
    }
}
