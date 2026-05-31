package cz.empatix.assets.service;

import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.render.domain.ShaderProgram;
import cz.empatix.textureatlas.domain.Texture;
import cz.empatix.textureatlas.domain.TextureAtlas;

/**
 * Production {@link AssetSource} that reads from the static {@link AssetsService} store (populated by the
 * loading pipeline at boot). Stateless — one shared instance can be threaded through the object graph.
 */
public final class StaticAssetSource implements AssetSource {

    public static final StaticAssetSource INSTANCE = new StaticAssetSource();

    @Override
    public TextureAtlas.Entry getTexture(final String name) {
        return AssetsService.getTexture(name);
    }

    @Override
    public ShaderProgram getShader(final String name) {
        return AssetsService.getShader(name);
    }

    @Override
    public Font getFont(final String name, final FontSize size) {
        return AssetsService.getFont(name, size);
    }

    @Override
    public Texture getNoiseTexture(final String name) {
        return AssetsService.getNoiseTexture(name);
    }
}
