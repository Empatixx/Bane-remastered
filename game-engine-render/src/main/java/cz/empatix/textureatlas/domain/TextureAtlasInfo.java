package cz.empatix.textureatlas.domain;

import com.google.common.base.Preconditions;
import org.joml.Vector2i;

public record TextureAtlasInfo (
        Vector2i textureStartPositions,
        Vector2i textureDimensions
){
    public TextureAtlasInfo(final int x, final int y, final int width, final int height){
        this(new Vector2i(x, y), new Vector2i(width, height));
    }
    public TextureAtlasInfo{
        Preconditions.checkArgument(textureStartPositions.x >= 0, "Texture start positions x must be positive");
        Preconditions.checkArgument(textureStartPositions.y >= 0, "Texture start positions y must be positive");
    }
}
