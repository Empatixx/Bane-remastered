package cz.empatix.assets.service;

import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.render.domain.ShaderProgram;
import cz.empatix.textureatlas.domain.Texture;
import cz.empatix.textureatlas.domain.TextureAtlas;

/**
 * Injectable seam for fetching loaded GPU assets. Production code is wired with {@link StaticAssetSource}
 * (which delegates to the static {@link AssetsService} store filled at load time); tests inject a fake
 * that returns GL-free stand-ins, so asset-coupled classes can be constructed without a GL context and
 * without static mocking.
 */
public interface AssetSource {

    TextureAtlas.Entry getTexture(String name);

    ShaderProgram getShader(String name);

    Font getFont(String name, FontSize size);

    Texture getNoiseTexture(String name);
}
