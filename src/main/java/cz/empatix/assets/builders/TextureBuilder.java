package cz.empatix.assets.builders;

import com.google.common.base.Preconditions;
import org.lwjgl.opengl.GL30;

import java.nio.ByteBuffer;

import static org.lwjgl.opengl.GL11.GL_UNPACK_ALIGNMENT;
import static org.lwjgl.opengl.GL11.glPixelStorei;

public class TextureBuilder{
    private int width;
    private int height;
    private int channels;
    private boolean applyWraps;
    private boolean applyFilters;
    private ByteBuffer buffer;

    public static TextureBuilder builder(){
        final TextureBuilder builder = new TextureBuilder();
        builder.width = -1;
        builder.height = -1;
        builder.channels = -1;
        return builder;
    }
    public TextureBuilder width(final int width){
        Preconditions.checkArgument(width > 0, "Width must be positive");
        this.width = width;
        return this;
    }
    public TextureBuilder height(final int height){
        Preconditions.checkArgument(height > 0, "Height must be positive");
        this.height = height;
        return this;
    }
    public TextureBuilder channels(final int channels){
        Preconditions.checkArgument(channels > 0, "Channels must be positive");
        this.channels = switch (channels) {
            case 4 -> GL30.GL_RGBA;
            case 3 -> GL30.GL_RGB;
            case 1 -> GL30.GL_RED;
            default -> throw new IllegalArgumentException("Unsupported number of channels: " + channels);
        };
        return this;
    }
    public TextureBuilder applyWraps(){
        this.applyWraps = true;
        return this;
    }
    public TextureBuilder applyFilters(){
        this.applyFilters = true;
        return this;
    }
    public TextureBuilder buffer(final ByteBuffer buffer){
        Preconditions.checkNotNull(buffer, "Buffer must not be null");
        this.buffer = buffer;
        return this;
    }
    public int build(){
        Preconditions.checkArgument(width > 0, "Width must be set");
        Preconditions.checkArgument(height > 0, "Height must be set");
        Preconditions.checkArgument(channels > 0, "Channels must be set");
        Preconditions.checkNotNull(buffer, "Buffer must be set");
        final int texture = GL30.glGenTextures();
        GL30.glBindTexture(GL30.GL_TEXTURE_2D, texture);
        if(channels == GL30.GL_RED){
            glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
        }
        if(applyWraps) {
            GL30.glTexParameteri(GL30.GL_TEXTURE_2D, GL30.GL_TEXTURE_WRAP_S, GL30.GL_REPEAT);
            GL30.glTexParameteri(GL30.GL_TEXTURE_2D, GL30.GL_TEXTURE_WRAP_T, GL30.GL_REPEAT);
        }
        if(applyFilters){
            GL30.glTexParameteri(GL30.GL_TEXTURE_2D, GL30.GL_TEXTURE_MIN_FILTER, GL30.GL_LINEAR_MIPMAP_LINEAR);
            GL30.glTexParameteri(GL30.GL_TEXTURE_2D, GL30.GL_TEXTURE_MAG_FILTER, GL30.GL_LINEAR);
        }
        GL30.glTexImage2D(GL30.GL_TEXTURE_2D, 0, GL30.GL_RGBA, width, height,
                0, channels, GL30.GL_UNSIGNED_BYTE, buffer);
        GL30.glGenerateMipmap(GL30.GL_TEXTURE_2D);
        GL30.glBindTexture(GL30.GL_TEXTURE_2D, 0);
        return texture;
    }
}
