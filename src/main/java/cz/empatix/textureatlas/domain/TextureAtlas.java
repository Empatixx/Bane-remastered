package cz.empatix.textureatlas.domain;

import com.google.common.base.Preconditions;
import cz.empatix.assets.builders.TextureBuilder;
import cz.empatix.assets.domain.TextureImage;
import cz.empatix.render.domain.RenderData;
import org.joml.Vector2i;

import java.util.Map;

public record TextureAtlas(
        TextureImage image,
        Map<String, Vector2i> textureStartPositions,
        Map<String, Vector2i> textureDimensions
) {
    public final static int RGBA_CHANNELS = 4;
    public TextureAtlas {
        Preconditions.checkArgument(image.width() > 0, "Width must be positive");
        Preconditions.checkArgument(image.height() > 0, "Height must be positive");
        Preconditions.checkArgument(image.textureId() != RenderData.NO_TEXTURE, "Texture id must be valid");
        Preconditions.checkArgument(!textureStartPositions.isEmpty(), "Texture start positions must not be empty");
        Preconditions.checkArgument(!textureDimensions.isEmpty(), "Texture dimensions must not be empty");
    }

    public void destroy() {
        image.destroy();
    }
    public static TextureAtlas from(final RawTextureAtlas raw){
        final int textureId = TextureBuilder.builder()
                .buffer(raw.pixels())
                .width(raw.width())
                .height(raw.height())
                .channels(RGBA_CHANNELS)
                .applyNearestFilters()
                .applyClampToEdgeWrap()
                .build();
        return new TextureAtlas(new TextureImage(raw.width(), raw.height(), textureId), raw.textureStartPositions(), raw.textureDimensions());
    }
}
