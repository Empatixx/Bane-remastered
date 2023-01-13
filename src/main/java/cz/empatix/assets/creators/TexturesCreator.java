package cz.empatix.assets.creators;

import cz.empatix.assets.domain.RawImage;
import org.lwjgl.opengl.GL30;

import java.util.Map;
import java.util.stream.Collectors;

public class TexturesCreator implements AssetCreator<RawImage, Integer> {
    private Map<String, Integer> textures;

    @Override
    public Integer get(String name) {
        return textures.get(name);
    }

    @Override
    public void create(final Map<String, RawImage> data) {
        this.textures = data.entrySet().stream()
                .map(entry -> Map.entry(entry.getKey(), createTexture(entry.getValue())))
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
    }

    @Override
    public void destroy() {
        textures.forEach((s, integer) -> GL30.glDeleteTextures(integer));
    }
    private int createTexture(final RawImage image){
        final int texture = GL30.glGenTextures();
        GL30.glBindTexture(GL30.GL_TEXTURE_2D, texture);
        GL30.glTexParameteri(GL30.GL_TEXTURE_2D, GL30.GL_TEXTURE_WRAP_S, GL30.GL_REPEAT);
        GL30.glTexParameteri(GL30.GL_TEXTURE_2D, GL30.GL_TEXTURE_WRAP_T, GL30.GL_REPEAT);
        GL30.glTexParameteri(GL30.GL_TEXTURE_2D, GL30.GL_TEXTURE_MIN_FILTER, GL30.GL_LINEAR);
        GL30.glTexParameteri(GL30.GL_TEXTURE_2D, GL30.GL_TEXTURE_MAG_FILTER, GL30.GL_LINEAR);
        GL30.glTexImage2D(GL30.GL_TEXTURE_2D, 0, GL30.GL_RGBA, image.width(), image.height(), 0, GL30.GL_RGBA, GL30.GL_UNSIGNED_BYTE, image.data());
        GL30.glGenerateMipmap(GL30.GL_TEXTURE_2D);
        image.destroy();
        return texture;
    }
}

