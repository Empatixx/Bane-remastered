package cz.empatix.textureatlas.domain;

import java.nio.ByteBuffer;
import java.util.Set;
import static org.lwjgl.opengl.GL33.*;

public class TextureArray {
    private final int textureArrayId;
    private final int width;
    private final int height;
    private final int layers;
    private final int channels;
    private final Set<TextureFlag> flags;

    public TextureArray(final int width, final int height, final int layers, final int channels, TextureFlag... flags) {
        this.width = width;
        this.height = height;
        this.layers = layers;
        this.channels = channels;
        this.flags = Set.of(flags);

        this.textureArrayId = glGenTextures();
        initTextureArray();
    }

    private void initTextureArray() {
        bind();
        final int internalFormat = getInternalFormat(channels);

        // Allocate storage for the texture array
        glTexImage3D(GL_TEXTURE_2D_ARRAY, 0, internalFormat, width, height, layers, 0, internalFormat, GL_UNSIGNED_BYTE, (ByteBuffer) null);

        if (flags.contains(TextureFlag.MIP_MAPPING)) {
            glGenerateMipmap(GL_TEXTURE_2D_ARRAY);
            glTexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR);
        } else {
            glTexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
        }

        glTexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
        glTexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
        glTexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
        unbind();
    }

    public void resize(final int width, final int height) {
        bind();
        final int internalFormat = getInternalFormat(channels);
        glTexImage3D(GL_TEXTURE_2D_ARRAY, 0, internalFormat, width, height, layers, 0, internalFormat, GL_UNSIGNED_BYTE, (ByteBuffer) null);
        unbind();
    }

    public void uploadLayer(final int layer, final ByteBuffer buffer) {
        if (layer < 0 || layer >= layers) {
            throw new IllegalArgumentException("Layer index out of bounds: " + layer);
        }
        bind();
        final int internalFormat = getInternalFormat(channels);

        // Upload data to a specific layer
        glTexSubImage3D(GL_TEXTURE_2D_ARRAY, 0, 0, 0, layer, width, height, 1, internalFormat, GL_UNSIGNED_BYTE, buffer);
        unbind();
    }

    public void bind() {
        glBindTexture(GL_TEXTURE_2D_ARRAY, textureArrayId);
    }

    public void bind(final int unit) {
        glActiveTexture(GL_TEXTURE0 + unit);
        glBindTexture(GL_TEXTURE_2D_ARRAY, textureArrayId);
    }

    public int getLayers() {
        return layers;
    }

    public void unbind() {
        glBindTexture(GL_TEXTURE_2D_ARRAY, 0);
    }

    public void destroy() {
        glDeleteTextures(textureArrayId);
    }

    private int getInternalFormat(final int channels) {
        return switch (channels) {
            case 1 -> GL_RED;
            case 3 -> GL_RGB;
            case 4 -> GL_RGBA;
            default -> throw new IllegalStateException("Unsupported channels: " + channels);
        };
    }

    public void attachFramebuffer(final int colorAttachment,final int layer) {
        glFramebufferTextureLayer(GL_FRAMEBUFFER, colorAttachment, textureArrayId, 0, layer);
    }
}
