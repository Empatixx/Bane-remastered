package cz.empatix.assets.domain;

import org.lwjgl.stb.STBImage;

import java.nio.ByteBuffer;

public record RawImage(int width,
                       int height,
                       int channels,
                       ByteBuffer data) {

    public void destroy() {
        STBImage.stbi_image_free(data);
    }
}

