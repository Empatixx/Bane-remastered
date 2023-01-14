package cz.empatix.assets.domain;

import java.nio.ByteBuffer;

import static org.lwjgl.stb.STBImage.stbi_image_free;

public record RawImage(int width,
                       int height,
                       int channels,
                       ByteBuffer data) {

    public void destroy() {
        stbi_image_free(data);
    }
}

