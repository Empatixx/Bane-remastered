package cz.empatix.textureatlas.domain;

import com.google.common.base.Preconditions;
import cz.empatix.assets.builders.TextureBuilder;
import org.joml.Vector2i;
import org.lwjgl.opengl.GL33;

import javax.imageio.ImageIO;
import java.awt.image.BufferedImage;
import java.io.File;
import java.io.IOException;
import java.util.Map;
import java.util.stream.Collectors;

public record TextureAtlas(
        int textureId,
        Map<String, Entry> texturesInfos
) {
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
        final int textureId = TextureBuilder.builder()
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
        byte[] atlasData = raw.pixels();
        BufferedImage image = new BufferedImage(raw.width(), raw.height(), BufferedImage.TYPE_4BYTE_ABGR);
        image.getRaster().setDataElements(0, 0, raw.width(), raw.height(), atlasData);
        try {
            ImageIO.write(image, "png", new File("atlas.png"));
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
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
