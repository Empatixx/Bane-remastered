package cz.empatix.textureatlas.domain;

import java.nio.ByteBuffer;
import java.util.Set;
import static org.lwjgl.opengl.GL33.*;

public class Texture {
    private final int textureId;

    private final int channels;
    private int width;
    private int height;

    private final Set<TextureFlag> flags;


    public Texture(final ByteBuffer buffer, final int width, final int height, final int channels, TextureFlag... flags){
        this.width = width;
        this.height = height;
        this.channels = channels;
        this.flags = Set.of(flags);

        this.textureId = glGenTextures();
        initTexture(buffer);
    }

    private void initTexture(final ByteBuffer buffer){
        final int bufferFormat = getInternalFormat(channels);
        bind();
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, bufferFormat, GL_UNSIGNED_BYTE, buffer);
        if (flags.contains(TextureFlag.MIP_MAPPING)){
            glGenerateMipmap(GL_TEXTURE_2D);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR);
        } else {
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
        }
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
        unbind();
    }

    public void resize(final int width, final int height){
        this.width = width;
        this.height = height;

        bind();
        final int bufferFormat = getInternalFormat(channels);
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, bufferFormat, GL_UNSIGNED_BYTE, (ByteBuffer) null);
        unbind();
    }

    public void bind(){
        glBindTexture(GL_TEXTURE_2D, textureId);
    }
    public void bind(final int unit){
        glActiveTexture(GL_TEXTURE0 + unit);
        glBindTexture(GL_TEXTURE_2D, textureId);
    }

    public void unbind(){
        glBindTexture(GL_TEXTURE_2D, 0);
    }

    public void destroy(){
        glDeleteTextures(textureId);
    }

    public void attachFramebuffer(final int attachment){
        glFramebufferTexture2D(GL_FRAMEBUFFER, attachment, GL_TEXTURE_2D, textureId, 0);
    }

    public int getTextureId(){
        return textureId;
    }

    private int getInternalFormat(final int channels){
        return switch (channels){
            case 1 -> GL_RED;
            case 3 -> GL_RGB;
            case 4 -> GL_RGBA;
            default -> throw new IllegalStateException("Unsupported channels: "+channels);
        };
    }
}
