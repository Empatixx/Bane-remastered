package cz.empatix.textureatlas.domain;

import com.google.common.base.Preconditions;
import cz.empatix.render.domain.RenderData;
import org.joml.Vector2i;
import org.lwjgl.opengl.GL30;

import java.util.Map;

public record TextureAtlas(
        int width,
        int height,
        int textureId,
        Map<String, Vector2i> textureStartPositions,
        Map<String, Vector2i> textureDimensions
) {
    public TextureAtlas {
        Preconditions.checkArgument(width > 0, "Width must be positive");
        Preconditions.checkArgument(height > 0, "Height must be positive");
        Preconditions.checkArgument(textureId != RenderData.NO_TEXTURE, "Texture id must be valid");
        Preconditions.checkArgument(!textureStartPositions.isEmpty(), "Texture start positions must not be empty");
        Preconditions.checkArgument(!textureDimensions.isEmpty(), "Texture dimensions must not be empty");
    }

    public void destroy() {
        GL30.glDeleteTextures(textureId);
    }

}
