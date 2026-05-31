package cz.empatix.testsupport;

import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetSource;
import cz.empatix.render.domain.ShaderProgram;
import cz.empatix.textureatlas.domain.Texture;
import cz.empatix.textureatlas.domain.TextureAtlas;
import cz.empatix.textureatlas.domain.TextureAtlasInfo;
import org.joml.Vector2i;

import java.util.Map;

/**
 * GL-free {@link AssetSource} for tests: hand-built stand-ins, no real assets, no GL, no static mocking.
 * Pass it into asset-coupled classes instead of stubbing {@code AssetsService} statically.
 *
 * <p>{@link #getTexture} returns a real {@link TextureAtlas.Entry} with real pixel dimensions so the
 * sprite/atlas math in HUD/animation constructors stays finite. {@link #getFont} returns a glyph-less
 * font (safe to construct with and to measure empty text); rendering actual glyphs is out of scope.
 */
public final class FakeAssetSource implements AssetSource {

    private static final Font EMPTY_FONT =
            new Font(Map.of(), FontSize.FONTSIZE_22, fakeAtlas());

    /** A GL-free atlas entry: a 16x16 sprite inside a 64x64 atlas. */
    public static TextureAtlas.Entry entry() {
        return new TextureAtlas.Entry(1, new Vector2i(64, 64), new TextureAtlasInfo(0, 0, 16, 16));
    }

    private static TextureAtlas fakeAtlas() {
        return new TextureAtlas(0, Map.of(
                "fake", new TextureAtlas.Entry(0, new Vector2i(1, 1), new TextureAtlasInfo(0, 0, 1, 1))));
    }

    @Override
    public TextureAtlas.Entry getTexture(final String name) {
        return entry();
    }

    @Override
    public ShaderProgram getShader(final String name) {
        return null; // domain ctors don't fetch shaders; they use fixed shader bindings
    }

    @Override
    public Font getFont(final String name, final FontSize size) {
        return EMPTY_FONT;
    }

    @Override
    public Texture getNoiseTexture(final String name) {
        return null;
    }
}
