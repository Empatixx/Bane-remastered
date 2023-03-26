package cz.empatix.textureatlas.domain;

import com.google.common.base.Preconditions;
import cz.empatix.render.builders.TextureBuilder;
import org.joml.Vector2i;
import org.lwjgl.opengl.GL33;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Map;
import java.util.stream.Collectors;

public record TextureAtlas(
        int textureId,
        Map<String, Entry> texturesInfos
) {
    private final static Logger LOGGER = LoggerFactory.getLogger(TextureAtlas.class);
    public final static int RGBA_CHANNELS = 4;
    public final static int RGB_CHANNELS = 3;
    public final static int R_CHANNELS = 1;

    public TextureAtlas {
        Preconditions.checkArgument(!texturesInfos.isEmpty(), "Textures info must not be empty");
    }

    public void destroy() {
        GL33.glDeleteTextures(textureId);
    }

    public static TextureAtlas from(final RawTextureAtlas raw) {
        return from(raw, TextureBuilder.BuildParam.NEAREST_FILTER);
    }
    public static TextureAtlas from(final RawTextureAtlas raw, final TextureBuilder.BuildParam type) {
        final int textureId = switch (type) {
            case NEAREST_FILTER -> nearestTexture(raw);
            case LINEAR_FILTER -> linearTexture(raw);
            default -> {
                LOGGER.error("Unsupported texture type: {}", type);
                throw new IllegalArgumentException("Unsupported texture type: " + type);
            }
        };
        final Vector2i atlasDimensions = new Vector2i(raw.width(), raw.height());
        final Map<String, Entry> texturesInfos = raw.texturesInfos().entrySet().stream()
                .map(entry -> Map.entry(entry.getKey(), new Entry(textureId, atlasDimensions, entry.getValue())))
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
        return new TextureAtlas(textureId, texturesInfos);
    }
    private static int linearTexture(final RawTextureAtlas raw){
        return TextureBuilder.builder()
                .buffer(raw.pixels())
                .width(raw.width())
                .height(raw.height())
                .channels(RGBA_CHANNELS)
                .applyLinearFilters()
                .applyClampToEdgeWrap()
                .build();
    }
    private static int nearestTexture(final RawTextureAtlas raw){
        return TextureBuilder.builder()
                .buffer(raw.pixels())
                .width(raw.width())
                .height(raw.height())
                .channels(RGBA_CHANNELS)
                .applyNearestFilters()
                .applyClampToEdgeWrap()
                .build();
        final Vector2i atlasDimensions = new Vector2i(raw.width(), raw.height());
        final Map<String, Entry> texturesInfos = raw.texturesInfos().entrySet().stream()
                .map(entry -> Map.entry(entry.getKey(), new Entry(textureId, atlasDimensions, entry.getValue())))
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
        return new TextureAtlas(textureId, texturesInfos);
    }

    public record Entry(
            int textureId,
            Vector2i atlasDimensions,
            TextureAtlasInfo info
    ) {
        public int atlasWidth() {
            return atlasDimensions.x();
        }
        public int atlasHeight() {
            return atlasDimensions.y();
        }
        public int width(){
            return info.textureDimensions().x();
        }
        public int height(){
            return info.textureDimensions().y();
        }
    }
}
