package cz.empatix.assets.providers;
import cz.empatix.core.Viewport;

import cz.empatix.textureatlas.domain.Texture;
import cz.empatix.textureatlas.domain.TextureFlag;
import cz.empatix.textureatlas.service.PerlinNoiseGenerator;
import org.lwjgl.opengl.GL30;

import java.nio.ByteBuffer;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

public class NoiseTextureProvider implements AssetProvider {
    private final PerlinNoiseGenerator perlinNoiseGenerator = new PerlinNoiseGenerator();
    private final Map<String, Texture> textureMap = new ConcurrentHashMap<>();

    @Override
    public void create() {
        if (textureMap.isEmpty()) {
            final Texture noise = getNoiseTexture(Viewport.TARGET_WIDTH, Viewport.TARGET_HEIGHT);
            textureMap.put("lightningNoise", noise);
        }
    }

    private Texture getNoiseTexture(final int width, final int height) {
        final ByteBuffer buffer = perlinNoiseGenerator.generateBuffer(width, height);
        return new Texture(buffer, width,height, 4, TextureFlag.MIP_MAPPING);
    }

    @Override
    public void load() {
    }

    @Override
    public void preload() {

    }

    @Override
    public void destroy() {

    }

    public Texture get(final String name) {
        return textureMap.get(name);
    }
}
